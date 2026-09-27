//! In-memory full-text index over runs, rebuilt on every (re)load.

use anyhow::Result;
use tantivy::collector::TopDocs;
use tantivy::query::{Query, QueryParser};
use tantivy::schema::{Field, STORED, STRING, Schema, TEXT, Value};
use tantivy::snippet::SnippetGenerator;
use tantivy::{Index, IndexReader, IndexWriter, TantivyDocument};

use super::runs::Run;

pub struct SearchIndex {
    // Kept alive for the reader.
    _index: Index,
    reader: IndexReader,
    slug: Field,
    body: Field,
    parser: QueryParser,
}

pub struct Hit {
    pub slug: String,
    /// HTML fragment with matches wrapped in `<b>`; already escaped.
    pub snippet: String,
}

impl SearchIndex {
    pub fn build(runs: &[Run]) -> Result<Self> {
        let mut sb = Schema::builder();
        let slug = sb.add_text_field("slug", STRING | STORED);
        let title = sb.add_text_field("title", TEXT);
        let summary = sb.add_text_field("summary", TEXT);
        let tags = sb.add_text_field("tags", TEXT);
        let body = sb.add_text_field("body", TEXT);
        let index = Index::create_in_ram(sb.build());

        let mut writer: IndexWriter = index.writer_with_num_threads(1, 50_000_000)?;
        for run in runs {
            let mut doc = TantivyDocument::default();
            doc.add_text(slug, &run.slug);
            doc.add_text(title, &run.meta.title);
            doc.add_text(summary, &run.meta.summary);
            for tag in &run.meta.tags {
                doc.add_text(tags, tag);
            }
            doc.add_text(body, &run.text);
            writer.add_document(doc)?;
        }
        writer.commit()?;

        let reader = index.reader()?;
        let mut parser = QueryParser::for_index(&index, vec![title, summary, tags, body]);
        parser.set_field_boost(title, 3.0);
        parser.set_field_boost(tags, 2.0);
        parser.set_field_boost(summary, 2.0);

        Ok(Self { _index: index, reader, slug, body, parser })
    }

    /// Search with a lenient parser, so malformed user input still returns results.
    /// `text_for` supplies each run's body so snippets don't require storing it twice.
    pub fn search<'a>(
        &self,
        q: &str,
        limit: usize,
        text_for: impl Fn(&str) -> Option<&'a str>,
    ) -> Result<Vec<Hit>> {
        let (query, _errors) = self.parser.parse_query_lenient(q);
        let searcher = self.reader.searcher();
        let top = searcher.search(&query, &TopDocs::with_limit(limit))?;
        let snippets = SnippetGenerator::create(&searcher, &query as &dyn Query, self.body)?;

        let mut hits = Vec::with_capacity(top.len());
        for (_score, addr) in top {
            let doc: TantivyDocument = searcher.doc(addr)?;
            let Some(slug) = doc.get_first(self.slug).and_then(|v| v.as_str()) else {
                continue;
            };
            let snippet = text_for(slug)
                .map(|text| snippets.snippet(text).to_html())
                .unwrap_or_default();
            hits.push(Hit { slug: slug.to_owned(), snippet });
        }
        Ok(hits)
    }
}
