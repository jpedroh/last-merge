use std::collections::HashMap;

use crate::tree_sitter_parser::ParserConfiguration;
use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode, Point,
};
use tree_sitter::{Node, Query, QueryCursor};

fn extract_identifier_from_node<'a>(
    node: Node,
    src: &'a str,
    config: &'a ParserConfiguration,
) -> Option<Vec<&'a str>> {
    let queries = HashMap::from([
        (
            "constructor_declaration",
            r#"
            (
    constructor_declaration
    name:
    (
        identifier
    )
    @method_name
    [parameters:
    (
        formal_parameters [
        (
            formal_parameter
            type:
            (
                _
            )
            @argument_type
        )
        (
            spread_parameter (type_identifier) @spread
        )
        ]
    )
    _
    ]
)

            
            "#,
        ),
        (
            "method_declaration",
            r#"
            (
    method_declaration
    name:
    (
        identifier
    )
    @method_name
    [parameters:
    (
        formal_parameters [
        (
            formal_parameter
            type:
            (
                _
            )
            @argument_type
        )
        (
            spread_parameter (type_identifier) @spread
        )
        ]
    )
    _
    ]
)

            
            "#,
        ),
        (
            "field_declaration",
            r#"(variable_declarator name: _ @name)"#,
        ),
        (
            "import_declaration",
            r#"(import_declaration ( scoped_identifier ) @namespace)"#,
        ),
        (
            "class_declaration",
            r#"(class_declaration name: (identifier) @name)"#,
        ),
        (
            "enum_declaration",
            r#"(enum_declaration name: (identifier) @name)"#,
        ),
        (
            "interface_declaration",
            r#"(interface_declaration name: (identifier) @name)"#,
        ),
    ]);

    let query_string = queries.get(node.kind())?;
    log::debug!("Using {:?} as query_string", query_string);
    let query = Query::new(config.language, &query_string).ok()?;
    let mut cursor = QueryCursor::new();
    let identifier = cursor
    .matches(&query, node, src.as_bytes())
    .into_iter()
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

    log::debug!("Found {:?} as identifier for node {:?}", identifier, node.utf8_text(src.as_bytes()).unwrap());

    Some(identifier
    )
}

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
            identifier: extract_identifier_from_node(node, &src, &config),
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
