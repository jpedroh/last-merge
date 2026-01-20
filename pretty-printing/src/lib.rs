use tree_sitter::Node;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Source {
    Base,
    Left,
    Right,
}

impl Source {
    fn select<'a>(&self, sources: &'a SourceCodes<'a>) -> &'a str {
        match self {
            Source::Base => sources.base,
            Source::Left => sources.left,
            Source::Right => sources.right,
        }
    }
}

pub enum MergedCstNode<'a, T: PrettyPrintableNode> {
    Clean(Source, &'a T),
    Conflict(Option<&'a T>, Option<&'a T>),
}

pub trait PrettyPrintableNode {
    fn trailing_white_space<'a>(&self, src: &'a str) -> Option<&'a str>;
    fn raw_source_code<'a>(&self, src: &'a str) -> &'a str;
    fn write_pretty(&self, src: &str, out: &mut String);
}

impl PrettyPrintableNode for Node<'_> {
    fn trailing_white_space<'a>(&self, src: &'a str) -> Option<&'a str> {
        self.prev_sibling().map(|previous| {
            let previous_end = previous.end_byte();
            let current_start = self.start_byte();
            &src[previous_end..current_start]
        })
    }

    fn raw_source_code<'a>(&self, src: &'a str) -> &'a str {
        self
            .utf8_text(src.as_bytes())
            .expect("Only UTF8 valid code is accepted")
    }

    fn write_pretty(&self, src: &str, out: &mut String) {
        if let Some(ws) = self.trailing_white_space(src) {
            out.push_str(ws);
        }
        out.push_str(self.raw_source_code(src));
    }
}

pub struct SourceCodes<'a> {
    base: &'a str,
    left: &'a str,
    right: &'a str,
}

impl SourceCodes<'_> {
    pub fn total_len(&self) -> usize {
        self.base.len() + self.left.len() + self.right.len()
    }
}

pub fn pretty_print_tree<T: PrettyPrintableNode>(
    tree: &[MergedCstNode<T>],
    source_codes: &SourceCodes,
) -> String {
    let mut out = String::with_capacity(source_codes.total_len());

    for node in tree {
        match node {
            MergedCstNode::Clean(source, current) => {
                current.write_pretty(source.select(source_codes), &mut out);
            }
            MergedCstNode::Conflict(left, right) => {
                out.push_str("\n<<<<<<<\n");
                if let Some(left) = left {
                    left.write_pretty(source_codes.left, &mut out);
                }
                out.push_str("\n=======\n");
                if let Some(right) = right {
                    right.write_pretty(source_codes.right, &mut out);
                }
                out.push_str("\n>>>>>>>\n");
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::SourceCodes;

    #[test]
    fn pretty_print_real_tree_sitter_nodes() {
        use tree_sitter::{Parser, Tree};

        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_java::LANGUAGE.into())
            .expect("Error loading grammar");

        let src = r#"public class Main {
            public static void main() {

            }
        }"#;
        let tree: Tree = parser.parse(src, None).unwrap();
        let root = tree.root_node();

        let mut nodes = Vec::new();
        let mut cursor = root.walk();

        let children: Vec<_> = root.children(&mut cursor).collect();

        for child in children.iter() {
            nodes.push(super::MergedCstNode::Clean(super::Source::Base, child));
        }

        let source_codes = SourceCodes {
            base: src,
            left: src,
            right: src,
        };
        let result = super::pretty_print_tree(&nodes, &source_codes);

        println!("{}", result);

        assert_eq!(result, src);
    }
}
