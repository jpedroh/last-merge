use tree_sitter::Node;

pub enum MergedCstNode<'a, T: PrettyPrintableNode> {
    Clean(&'a T),
    Conflict(Option<&'a T>, Option<&'a T>),
}

pub trait PrettyPrintableNode {
    fn trailing_white_space<'a>(&'a self, src: &'a str) -> Option<&'a str>;
    fn raw_source_code<'a>(&'a self, src: &'a str) -> &'a str;
    fn write_pretty(&self, src: &str, out: &mut String);
}

impl PrettyPrintableNode for Node<'_> {
    fn trailing_white_space<'a>(&'a self, src: &'a str) -> Option<&'a str> {
        self.prev_sibling().map(|previous| {
            let previous_end = previous.end_byte();
            let current_start = self.start_byte();
            &src[previous_end..current_start]
        })
    }

    fn raw_source_code<'a>(&'a self, src: &'a str) -> &'a str {
        &self.utf8_text(src.as_bytes()).expect("Only UTF8 valid code is accepted")
    }

    fn write_pretty(&self, src: &str, out: &mut String) {
        if let Some(ws) = self.trailing_white_space(src) {
            out.push_str(ws);
        }
        out.push_str(self.raw_source_code(src));
    }
}

pub fn pretty_print_tree<T: PrettyPrintableNode>(tree: &[MergedCstNode<T>], src: &str) -> String {
    let mut out = String::new();

    for node in tree {
        match node {
            MergedCstNode::Clean(current) => {
                current.write_pretty(src, &mut out);
            }
            MergedCstNode::Conflict(left, right) => {
                out.push_str("\n<<<<<<<\n");
                if let Some(left) = left {
                    left.write_pretty(src, &mut out);
                }
                out.push_str("\n=======\n");
                if let Some(right) = right {
                    right.write_pretty(src, &mut out);
                }
                out.push_str("\n>>>>>>>\n");
            }
        }
    }

    out
}

#[cfg(test)]
mod tests {
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
            nodes.push(super::MergedCstNode::Clean(child));
        }

        let result = super::pretty_print_tree(&nodes, src);

        println!("{}", result);

        assert_eq!(result, src);
    }
}
