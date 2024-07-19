use crate::tree_sitter_parser::ParserConfiguration;
use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode, Point,
};
use tree_sitter::Node;

fn explore_node<'a>(node: Node, src: &'a str, config: &'a ParserConfiguration) -> CSTNode<'a> {
    if node.child_count() == 0 || config.stop_compilation_at.contains(node.kind()) {
        CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: node.kind(),
            start_position: Point {
                row: node.start_position().row,
                column: node.start_position().column,
            },
            end_position: Point {
                row: node.end_position().row,
                column: node.end_position().column,
            },
            value: &src[node.byte_range()],
            is_block_end_delimiter: config.block_end_delimiters.contains(node.kind()),
        })
    } else {
        let mut cursor = node.walk();
        CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: node.kind(),
            start_position: Point {
                row: node.start_position().row,
                column: node.start_position().column,
            },
            end_position: Point {
                row: node.end_position().row,
                column: node.end_position().column,
            },
            children: node
                .children(&mut cursor)
                .map(|child| explore_node(child, src, config))
                .collect(),
            are_children_unordered: config.kinds_with_unordered_children.contains(node.kind()),
            identifier: None,
        })
    }
}

pub fn parse_string<'a>(
    src: &'a str,
    config: &'a ParserConfiguration,
) -> Result<CSTNode<'a>, &'static str> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(config.language)
        .map_err(|_| "There was an error while setting the parser language")?;

    let parsed = parser
        .parse(src, None)
        .ok_or("It was not possible to parse the tree.")?;
    let root = explore_node(parsed.root_node(), src, config);
    Ok(config.handlers.run(root))
}
