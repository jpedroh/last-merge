use tree_sitter::{Language, Node, Query, QueryCapture, QueryCursor};

pub trait IdentifierExtractor {
    fn extract_identifier_from_node<'a>(&self, node: Node, src: &'a str) -> Option<Vec<&'a str>>;
}

pub struct TreeSitterQuery(Query);

impl TreeSitterQuery {
    pub fn new(query: &'static str, language: Language) -> Self {
        Self(
            Query::new(language, query)
                .expect("Invalid Query provided for building TreeSitterQuery"),
        )
    }
}

impl IdentifierExtractor for TreeSitterQuery {
    fn extract_identifier_from_node<'a>(&self, node: Node, src: &'a str) -> Option<Vec<&'a str>> {
        let mut cursor = QueryCursor::new();
        cursor
            .matches(&self.0, node, src.as_bytes())
            .find(|a_match| {
                a_match
                    .captures
                    .iter()
                    .all(|a_capture| capture_is_within_node_bounds(a_capture, &node))
            })
            .map(|a_match| {
                a_match
                    .captures
                    .iter()
                    .flat_map(|capture| capture.node.utf8_text(src.as_bytes()).ok())
                    .collect()
            })
    }
}

fn capture_is_within_node_bounds(capture: &QueryCapture, node: &Node) -> bool {
    capture.node.start_byte() >= node.start_byte() && capture.node.end_byte() <= node.end_byte()
}
