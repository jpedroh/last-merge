use regex::Regex;
use tree_sitter::{Language, Node, Query, QueryCursor};

pub trait IdentifierExtractor {
    fn extract_identifier_from_node<'a>(&self, node: Node, src: &'a str) -> Option<Vec<&'a str>>;
}

pub struct RegularExpression(Regex);

impl RegularExpression {
    pub fn new(regex: &'static str) -> Self {
        Self(
            regex::Regex::new(regex)
                .expect("Invalid regex provided for building RegularExpression"),
        )
    }
}

impl IdentifierExtractor for RegularExpression {
    fn extract_identifier_from_node<'a>(&self, node: Node, src: &'a str) -> Option<Vec<&'a str>> {
        self.0
            .find(node.utf8_text(src.as_bytes()).ok()?)
            .map(|m| vec![m.as_str()])
    }
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
        let identifier = cursor
            .matches(&self.0, node, src.as_bytes())
            .flat_map(|a_match| {
                a_match
                    .captures
                    .iter()
                    .filter(|capture| {
                        capture.node.start_byte() >= node.start_byte()
                            && capture.node.end_byte() <= node.end_byte()
                    })
                    .filter_map(|capture_index| capture_index.node.utf8_text(src.as_bytes()).ok())
            })
            .collect();
        Some(identifier)
    }
}
