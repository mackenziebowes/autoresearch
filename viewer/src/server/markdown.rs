//! Markdown rendering. Research artifacts often quote scraped web content, so
//! raw HTML is shown as text and script-capable link schemes are neutralised.
//! Relative links are rewritten to point at the run's files.

use pulldown_cmark::{CowStr, Event, Options, Parser, Tag, html};

/// Render `src`, a markdown file at `<slug>/<dir><name>`. `dir` is empty or ends in `/`.
pub fn render(src: &str, slug: &str, dir: &str) -> String {
    let opts = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS;
    let rewrite = |url: CowStr<'static>| -> CowStr<'static> {
        match classify(&url) {
            Url::Absolute | Url::Anchor => url,
            Url::Unsafe => "#".into(),
            Url::Relative if url.ends_with(".md") => format!("/runs/{slug}/md/{dir}{url}").into(),
            Url::Relative => format!("/files/{slug}/{dir}{url}").into(),
        }
    };
    let events = Parser::new_ext(src, opts).map(|event| match event {
        Event::Html(raw) | Event::InlineHtml(raw) => Event::Text(raw),
        Event::Start(Tag::Link { link_type, dest_url, title, id }) => Event::Start(Tag::Link {
            link_type,
            dest_url: rewrite(dest_url.into_static()),
            title,
            id,
        }),
        Event::Start(Tag::Image { link_type, dest_url, title, id }) => Event::Start(Tag::Image {
            link_type,
            dest_url: rewrite(dest_url.into_static()),
            title,
            id,
        }),
        other => other,
    });
    let mut out = String::new();
    html::push_html(&mut out, events);
    out
}

#[derive(Debug, PartialEq)]
enum Url {
    /// http, https or mailto, or a root-relative path.
    Absolute,
    /// `#fragment` within the page.
    Anchor,
    /// Relative to the markdown file.
    Relative,
    /// Any other scheme (`javascript:`, `data:`, …).
    Unsafe,
}

fn classify(url: &str) -> Url {
    // Browsers ignore tabs/newlines and leading control chars when parsing URLs.
    let cleaned: String = url.chars().filter(|c| !matches!(c, '\t' | '\n' | '\r')).collect();
    let cleaned = cleaned.trim_start_matches(|c: char| c <= ' ');
    if cleaned.starts_with('#') {
        return Url::Anchor;
    }
    if cleaned.starts_with('/') {
        return Url::Absolute;
    }
    match cleaned.find([':', '/', '?', '#']) {
        Some(i) if cleaned[i..].starts_with(':') => {
            match cleaned[..i].to_ascii_lowercase().as_str() {
                "http" | "https" | "mailto" => Url::Absolute,
                _ => Url::Unsafe,
            }
        }
        _ => Url::Relative,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_html_is_escaped() {
        let out = render("hi <script>alert(1)</script>", "r", "");
        assert!(!out.contains("<script>"));
        assert!(out.contains("&lt;script&gt;"));
    }

    #[test]
    fn unsafe_schemes_are_neutralised() {
        for url in ["javascript:alert(1)", "JavaScript:alert(1)", "java\tscript:x", "data:text/html,x"] {
            assert_eq!(classify(url), Url::Unsafe, "{url}");
        }
    }

    #[test]
    fn links_are_rewritten() {
        let out = render(
            "[a](figures/plot.png) [b](https://example.com) [c](#sec) [d](notes.md) [e](a/b:c)",
            "run-1",
            "sub/",
        );
        assert!(out.contains(r#"href="/files/run-1/sub/figures/plot.png""#));
        assert!(out.contains(r#"href="https://example.com""#));
        assert!(out.contains(r##"href="#sec""##));
        assert!(out.contains(r#"href="/runs/run-1/md/sub/notes.md""#));
        assert!(out.contains(r#"href="/files/run-1/sub/a/b:c""#));
    }
}
