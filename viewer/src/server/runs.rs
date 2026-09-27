//! Loading research runs from disk.
//!
//! Each run is a directory under the research root containing a `meta.toml`
//! and any number of artifact files. Directories starting with `.` or `_`
//! are skipped (use `_template/` for scaffolding).

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;
use walkdir::WalkDir;

/// Files checked, in order, for the run's main report.
pub const REPORT_NAMES: &[&str] = &["report.md", "README.md", "index.md"];

/// Text files larger than this are listed but not indexed for search.
const MAX_INDEXED_BYTES: u64 = 2 * 1024 * 1024;

const TEXT_EXTENSIONS: &[&str] = &[
    "md", "markdown", "txt", "csv", "tsv", "json", "jsonl", "yaml", "yml", "toml", "log", "py",
    "rs", "sh",
];

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    pub title: String,
    pub date: toml::value::Datetime,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub status: Status,
    #[serde(default)]
    pub summary: String,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Draft,
    Running,
    #[default]
    Complete,
    Abandoned,
}

impl Status {
    pub const ALL: [Status; 4] = [Status::Draft, Status::Running, Status::Complete, Status::Abandoned];

    pub fn as_str(self) -> &'static str {
        match self {
            Status::Draft => "draft",
            Status::Running => "running",
            Status::Complete => "complete",
            Status::Abandoned => "abandoned",
        }
    }
}

#[derive(Debug)]
pub struct Run {
    pub slug: String,
    pub dir: PathBuf,
    pub meta: Meta,
    /// Markdown source of the main report, if one exists.
    pub report: Option<String>,
    /// Paths relative to `dir`, using `/` separators, sorted.
    pub files: Vec<String>,
    /// Concatenated text of all indexable files, for search.
    pub text: String,
}

/// A run directory that failed to load, surfaced in the UI so it can be fixed.
#[derive(Debug)]
pub struct Broken {
    pub slug: String,
    pub error: String,
}

pub fn load_all(root: &Path) -> Result<(Vec<Run>, Vec<Broken>)> {
    let mut runs = Vec::new();
    let mut broken = Vec::new();

    let entries =
        std::fs::read_dir(root).with_context(|| format!("reading {}", root.display()))?;
    for entry in entries {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let slug = entry.file_name().to_string_lossy().into_owned();
        if slug.starts_with(['.', '_']) {
            continue;
        }
        match load_run(&entry.path(), slug.clone()) {
            Ok(run) => runs.push(run),
            Err(e) => broken.push(Broken { slug, error: format!("{e:#}") }),
        }
    }

    // Newest first; ISO dates sort lexically.
    runs.sort_by(|a, b| {
        b.meta
            .date
            .to_string()
            .cmp(&a.meta.date.to_string())
            .then_with(|| a.slug.cmp(&b.slug))
    });
    broken.sort_by(|a, b| a.slug.cmp(&b.slug));
    Ok((runs, broken))
}

fn load_run(dir: &Path, slug: String) -> Result<Run> {
    let raw = std::fs::read_to_string(dir.join("meta.toml"))
        .context("missing or unreadable meta.toml")?;
    let meta: Meta = toml::from_str(&raw).context("invalid meta.toml")?;

    let mut files = Vec::new();
    let mut text = String::new();
    let walker = WalkDir::new(dir)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| e.depth() == 0 || !e.file_name().to_string_lossy().starts_with('.'));
    for entry in walker {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let rel = entry.path().strip_prefix(dir)?.to_string_lossy().replace('\\', "/");
        if rel != "meta.toml"
            && is_text(entry.path())
            && entry.metadata()?.len() <= MAX_INDEXED_BYTES
        {
            if let Ok(s) = std::fs::read_to_string(entry.path()) {
                text.push_str(&s);
                text.push('\n');
            }
        }
        files.push(rel);
    }

    let report = REPORT_NAMES
        .iter()
        .find(|name| files.iter().any(|f| f == *name))
        .map(|name| std::fs::read_to_string(dir.join(name)))
        .transpose()
        .context("reading report")?;

    Ok(Run { slug, dir: dir.to_path_buf(), meta, report, files, text })
}

pub fn is_text(path: &Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| TEXT_EXTENSIONS.contains(&e.to_ascii_lowercase().as_str()))
}
