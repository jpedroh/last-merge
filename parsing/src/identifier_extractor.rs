use regex::Regex;
use tree_sitter::{Node, Query, QueryCursor};

pub trait IdentifierExtractor {
    fn extract_identifier_from_node<'a>(&self, node: Node, src: &'a str) -> Option<Vec<&'a str>>;
}

pub struct RegularExpression(Regex);

impl RegularExpression {
    pub fn new(regex: &'static str) -> Self {
        let regex_query = regex::Regex::new(regex)
            .expect("Invalid regex provided for building RegularExpression");
        Self(regex_query)
    }
}

impl IdentifierExtractor for RegularExpression {
    fn extract_identifier_from_node<'a>(&self, node: Node, src: &'a str) -> Option<Vec<&'a str>> {
        self.0
            .find(node.utf8_text(src.as_bytes()).ok()?)
            .map(|m| vec![m.as_str()])
    }
}

pub struct TreeSitterQuery(&'static str);

impl TreeSitterQuery {
    pub fn new(query: &'static str) -> Self {
        Self(query)
    }
}

impl IdentifierExtractor for TreeSitterQuery {
    fn extract_identifier_from_node<'a>(&self, node: Node, src: &'a str) -> Option<Vec<&'a str>> {
        let query = Query::new(node.language(), self.0).ok()?;
        let mut cursor = QueryCursor::new();
        let identifier = cursor
            .matches(&query, node, src.as_bytes())
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
