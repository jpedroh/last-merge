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
            leading_white_space: node
                .prev_sibling()
                .map(|previous| &src[previous.end_byte()..node.start_byte()]),
        })
    } else {
        let mut cursor = node.walk();
        let identifier = config
            .identifier_extractors
            .get(node.kind())
            .and_then(|extractor| extractor.extract_identifier_from_node(node, src));

        if let Some(ref identifier) = identifier {
            log::debug!("Found identifier {:?} for node {:?}", identifier, node);
        }

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
            identifier,
            leading_white_space: node
                .prev_sibling()
                .map(|previous| &src[previous.end_byte()..node.start_byte()]),
            are_children_unordered: config.kinds_with_unordered_children.contains(node.kind()),
            delimiters: config.delimiters.get(node.kind()),
        })
    }
}

pub fn parse_string<'a>(
    src: &'a str,
    config: &'a ParserConfiguration,
) -> Result<CSTNode<'a>, &'static str> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&config.language)
        .map_err(|_| "There was an error while setting the parser language")?;

    let parsed = parser
        .parse(src, None)
        .ok_or("It was not possible to parse the tree.")?;
    let root = explore_node(parsed.root_node(), src, config);
    Ok(config.handlers.run(root))
}
