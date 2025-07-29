use tree_sitter::{Language, Node, Query, QueryCursor, StreamingIterator};

pub trait IdentifierExtractor {
    fn extract_identifier_from_node<'a>(&self, node: Node, src: &'a str) -> Option<Vec<&'a str>>;
}

pub struct TreeSitterQuery(Query);

impl TreeSitterQuery {
    pub fn new(query: &'static str, language: Language) -> Self {
        Self(
            Query::new(&language, query)
                .expect("Invalid Query provided for building TreeSitterQuery"),
        )
    }
}

impl IdentifierExtractor for TreeSitterQuery {
    fn extract_identifier_from_node<'a>(&self, node: Node, src: &'a str) -> Option<Vec<&'a str>> {
        let mut cursor = QueryCursor::new();
        cursor
            .set_byte_range(node.byte_range())
            .matches(&self.0, node, src.as_bytes())
            .next()
            .map(|a_match| {
                a_match
                    .captures
                    .iter()
                    .flat_map(|capture| capture.node.utf8_text(src.as_bytes()).ok())
                    .collect()
            })
    }
}
