//! Server-only code: loads runs from disk, keeps the search index, serves raw files.

pub mod markdown;
pub mod runs;
pub mod search;

use std::collections::{BTreeMap, HashMap};
use std::path::{Component, Path, PathBuf};
use std::sync::{Arc, OnceLock, RwLock};

use anyhow::{Context, Result};
use axum::body::Body;
use axum::extract::Path as UrlPath;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};

use crate::app::{MarkdownDoc, RunDetail, RunList, RunSummary};
use runs::{Broken, Run};
use search::SearchIndex;

static STATE: OnceLock<AppState> = OnceLock::new();

struct AppState {
    root: PathBuf,
    loaded: RwLock<Arc<Loaded>>,
}

struct Loaded {
    runs: Vec<Run>,
    broken: Vec<Broken>,
    by_slug: HashMap<String, usize>,
    index: SearchIndex,
}

impl Loaded {
    fn build(root: &Path) -> Result<Self> {
        let (runs, broken) = runs::load_all(root)?;
        let index = SearchIndex::build(&runs)?;
        let by_slug = runs.iter().enumerate().map(|(i, r)| (r.slug.clone(), i)).collect();
        Ok(Self { runs, broken, by_slug, index })
    }

    fn get(&self, slug: &str) -> Option<&Run> {
        self.by_slug.get(slug).map(|&i| &self.runs[i])
    }
}

/// Load all runs under `root`. Call once at startup.
pub fn init(root: PathBuf) -> Result<()> {
    let root = root
        .canonicalize()
        .with_context(|| format!("research directory {} not found", root.display()))?;
    let loaded = Loaded::build(&root)?;
    tracing::info!(runs = loaded.runs.len(), broken = loaded.broken.len(), root = %root.display(), "loaded");
    STATE
        .set(AppState { root, loaded: RwLock::new(Arc::new(loaded)) })
        .map_err(|_| anyhow::anyhow!("server state already initialised"))
}

fn state() -> &'static AppState {
    STATE.get().expect("server::init not called")
}

fn current() -> Arc<Loaded> {
    state().loaded.read().expect("state lock poisoned").clone()
}

/// Re-scan the research directory and rebuild the index.
pub async fn reload() -> Result<()> {
    let root = state().root.clone();
    let loaded = tokio::task::spawn_blocking(move || Loaded::build(&root)).await??;
    *state().loaded.write().expect("state lock poisoned") = Arc::new(loaded);
    Ok(())
}

fn summary(run: &Run, snippet: Option<String>) -> RunSummary {
    RunSummary {
        slug: run.slug.clone(),
        title: run.meta.title.clone(),
        date: run.meta.date.to_string(),
        tags: run.meta.tags.clone(),
        status: run.meta.status.as_str().to_owned(),
        summary: run.meta.summary.clone(),
        snippet,
    }
}

pub fn list_runs(q: &str, tag: &str, status: &str) -> Result<RunList> {
    let loaded = current();
    let q = q.trim();

    let mut runs: Vec<RunSummary> = if q.is_empty() {
        loaded.runs.iter().map(|r| summary(r, None)).collect()
    } else {
        loaded
            .index
            .search(q, 500, |slug| loaded.get(slug).map(|r| r.text.as_str()))?
            .into_iter()
            .filter_map(|hit| {
                let snippet = Some(hit.snippet).filter(|s| !s.is_empty());
                loaded.get(&hit.slug).map(|r| summary(r, snippet))
            })
            .collect()
    };
    runs.retain(|r| {
        (tag.is_empty() || r.tags.iter().any(|t| t == tag))
            && (status.is_empty() || r.status == status)
    });

    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for run in &loaded.runs {
        for t in &run.meta.tags {
            *counts.entry(t).or_default() += 1;
        }
    }
    let mut tags: Vec<(String, usize)> = counts.into_iter().map(|(t, n)| (t.to_owned(), n)).collect();
    tags.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    Ok(RunList {
        runs,
        tags,
        total: loaded.runs.len(),
        broken: loaded.broken.iter().map(|b| (b.slug.clone(), b.error.clone())).collect(),
    })
}

pub fn get_run(slug: &str) -> Option<RunDetail> {
    let loaded = current();
    let run = loaded.get(slug)?;
    let report_name = runs::REPORT_NAMES
        .iter()
        .find(|name| run.files.iter().any(|f| f == *name))
        .map(|s| s.to_string());
    Some(RunDetail {
        run: summary(run, None),
        report_html: run.report.as_deref().map(|md| markdown::render(md, slug, "")),
        report_name,
        files: run.files.clone(),
    })
}

pub fn get_markdown(slug: &str, path: &str) -> Result<Option<MarkdownDoc>> {
    let loaded = current();
    let Some(run) = loaded.get(slug) else { return Ok(None) };
    let Some(full) = resolve(&run.dir, path) else { return Ok(None) };
    let src = std::fs::read_to_string(&full).with_context(|| format!("reading {path}"))?;
    let dir = match path.rfind('/') {
        Some(i) => &path[..=i],
        None => "",
    };
    Ok(Some(MarkdownDoc {
        run_title: run.meta.title.clone(),
        path: path.to_owned(),
        html: markdown::render(&src, slug, dir),
    }))
}

/// Resolve `rel` inside `dir`, refusing anything that could escape it.
fn resolve(dir: &Path, rel: &str) -> Option<PathBuf> {
    let rel = Path::new(rel);
    if rel.as_os_str().is_empty() || !rel.components().all(|c| matches!(c, Component::Normal(_))) {
        return None;
    }
    let full = dir.join(rel).canonicalize().ok()?;
    (full.starts_with(dir) && full.is_file()).then_some(full)
}

/// `GET /files/{slug}/{*path}`: raw artifact bytes.
pub async fn serve_file(UrlPath((slug, path)): UrlPath<(String, String)>) -> Response {
    let loaded = current();
    let Some(full) = loaded.get(&slug).and_then(|run| resolve(&run.dir, &path)) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let bytes = match tokio::fs::read(&full).await {
        Ok(b) => b,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    let mime = mime_guess::from_path(&full).first_or_octet_stream();
    Response::builder()
        .header(header::CONTENT_TYPE, mime.as_ref())
        .header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
        // Artifacts may be scraped HTML/SVG; never let them run script on this origin.
        .header(header::CONTENT_SECURITY_POLICY, "sandbox")
        .body(Body::from(bytes))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_rejects_escapes() {
        let dir = std::env::temp_dir().canonicalize().unwrap();
        for bad in ["", "../etc/passwd", "/etc/passwd", "a/../../x", "./x"] {
            assert!(resolve(&dir, bad).is_none(), "{bad}");
        }
    }
}
