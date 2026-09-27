# autoresearch

Artifacts from autoresearch runs, plus a small web app for searching and reviewing them.

## Layout

```
research/
  _template/              copy this to start a new run (ignored by the viewer)
  2026-09-27-some-topic/  one directory per run
    meta.toml             required: title, date, tags, status, summary
    report.md             main write-up (README.md or index.md also work)
    ...                   anything else: notes, data, figures, logs
viewer/                   Rust web app (Leptos + Thaw, served by axum)
```

`meta.toml`:

```toml
title = "What question this run investigated"
date = 2026-09-27
tags = ["llm", "evals"]
status = "complete"   # draft | running | complete | abandoned
summary = "One or two sentences on the finding."
```

Name run directories `YYYY-MM-DD-short-slug`. Directories starting with `_` or `.` are skipped. A run with a missing or invalid `meta.toml` shows up as a warning at the top of the viewer rather than disappearing.

## Viewer

Full-text search over every run (titles, summaries, tags and all text files) using tantivy, filters by tag and status, rendered reports, and a file browser for each run.

Setup (once):

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --version 0.2.35 --locked
```

Run it from `viewer/`:

```bash
cargo leptos watch
```

Then open http://127.0.0.1:3000. By default it reads `../research`; set `AUTORESEARCH_DIR` to point it elsewhere. The index is built in memory at startup; press **Reload** in the header after adding runs.

For a release build: `cargo leptos build --release`, then run `target/release/viewer` with `LEPTOS_SITE_ROOT=target/site`.

Markdown is rendered with raw HTML shown as text, and raw files are served with a sandboxing CSP, since research artifacts often contain scraped web content.

`cargo-leptos` is pinned to 0.2.35 because newer releases need a newer rustc than 1.87. Once the toolchain is upgraded, the pin (and the `tantivy = "0.25"` cap in `viewer/Cargo.toml`) can be lifted.
