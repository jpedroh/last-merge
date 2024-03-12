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

    let parsed = parser.parse(src, None);
    match parsed {
        Some(parsed) => Result::Ok(explore_node(parsed.root_node(), src, config)),
        None => Result::Err("It was not possible to parse the tree."),
    }
}

#[cfg(test)]
mod tests {
    use model::cst_node::{NonTerminal, Terminal};
    use model::CSTNode;
    use model::Point;

    use super::*;

    // #[test]
    // fn it_parses_an_interface() {
    //     let code = r#"
    //         public static interface HelloWorld {
    //             void sayHello(String name);
    //         }
    //     "#;
    //     let parser_configuration = ParserConfiguration {
    //         language: tree_sitter_java::language(),
    //         stop_compilation_at: [].into_iter().collect(),
    //         kinds_with_unordered_children: [].into(),
    //         block_end_delimiters: ["}"].into(),
    //     };
    //     let result = parse_string(code, &parser_configuration);
    //     let expected = CSTNode::NonTerminal(NonTerminal {
    //         id: uuid::Uuid::new_v4(),
    //         kind: "program",
    //         are_children_unordered: false,
    //         children: vec![CSTNode::NonTerminal(NonTerminal {
    //             id: uuid::Uuid::new_v4(),
    //             kind: "interface_declaration",
    //             are_children_unordered: false,
    //             children: vec![
    //                 CSTNode::NonTerminal(NonTerminal {
    //                     id: uuid::Uuid::new_v4(),
    //                     kind: "modifiers",
    //                     are_children_unordered: false,
    //                     children: vec![
    //                         CSTNode::Terminal(Terminal {
    //                             id: uuid::Uuid::new_v4(),
    //                             kind: "public",
    //                             value: "public",
    //                             start_position: Point { row: 1, column: 12 },
    //                             end_position: Point { row: 1, column: 18 },
    //                             is_block_end_delimiter: false,
    //                         }),
    //                         CSTNode::Terminal(Terminal {
    //                             id: uuid::Uuid::new_v4(),
    //                             kind: "static",
    //                             value: "static",
    //                             start_position: Point { row: 1, column: 19 },
    //                             end_position: Point { row: 1, column: 25 },
    //                             is_block_end_delimiter: false,
    //                         }),
    //                     ],
    //                     start_position: Point { row: 1, column: 12 },
    //                     end_position: Point { row: 1, column: 25 },
    //                 }),
    //                 CSTNode::Terminal(Terminal {
    //                     id: uuid::Uuid::new_v4(),
    //                     kind: "interface",
    //                     value: "interface",
    //                     start_position: Point { row: 1, column: 26 },
    //                     end_position: Point { row: 1, column: 35 },
    //                     is_block_end_delimiter: false,
    //                 }),
    //                 CSTNode::Terminal(Terminal {
    //                     id: uuid::Uuid::new_v4(),
    //                     kind: "identifier",
    //                     value: "HelloWorld",
    //                     start_position: Point { row: 1, column: 36 },
    //                     end_position: Point { row: 1, column: 46 },
    //                     is_block_end_delimiter: false,
    //                 }),
    //                 CSTNode::NonTerminal(NonTerminal {
    //                     id: uuid::Uuid::new_v4(),
    //                     kind: "interface_body",
    //                     are_children_unordered: false,
    //                     children: vec![
    //                         CSTNode::Terminal(Terminal {
    //                             id: uuid::Uuid::new_v4(),
    //                             kind: "{",
    //                             value: "{",
    //                             start_position: Point { row: 1, column: 47 },
    //                             end_position: Point { row: 1, column: 48 },
    //                             is_block_end_delimiter: false,
    //                         }),
    //                         CSTNode::NonTerminal(NonTerminal {
    //                             id: uuid::Uuid::new_v4(),
    //                             kind: "method_declaration",
    //                             are_children_unordered: false,
    //                             children: vec![
    //                                 CSTNode::Terminal(Terminal {
    //                                     id: uuid::Uuid::new_v4(),
    //                                     kind: "void_type",
    //                                     value: "void",
    //                                     start_position: Point { row: 2, column: 16 },
    //                                     end_position: Point { row: 2, column: 20 },
    //                                     is_block_end_delimiter: false,
    //                                 }),
    //                                 CSTNode::Terminal(Terminal {
    //                                     id: uuid::Uuid::new_v4(),
    //                                     kind: "identifier",
    //                                     value: "sayHello",
    //                                     start_position: Point { row: 2, column: 21 },
    //                                     end_position: Point { row: 2, column: 29 },
    //                                     is_block_end_delimiter: false,
    //                                 }),
    //                                 CSTNode::NonTerminal(NonTerminal {
    //                                     id: uuid::Uuid::new_v4(),
    //                                     kind: "formal_parameters",
    //                                     are_children_unordered: false,
    //                                     children: vec![
    //                                         CSTNode::Terminal(Terminal {
    //                                             id: uuid::Uuid::new_v4(),
    //                                             kind: "(",
    //                                             value: "(",
    //                                             start_position: Point { row: 2, column: 29 },
    //                                             end_position: Point { row: 2, column: 30 },
    //                                             is_block_end_delimiter: false,
    //                                         }),
    //                                         CSTNode::NonTerminal(NonTerminal {
    //                                             id: uuid::Uuid::new_v4(),
    //                                             kind: "formal_parameter",
    //                                             are_children_unordered: false,
    //                                             children: vec![
    //                                                 CSTNode::Terminal(Terminal {
    //                                                     id: uuid::Uuid::new_v4(),
    //                                                     kind: "type_identifier",
    //                                                     value: "String",
    //                                                     start_position: Point {
    //                                                         row: 2,
    //                                                         column: 30,
    //                                                     },
    //                                                     end_position: Point { row: 2, column: 36 },
    //                                                     is_block_end_delimiter: false,
    //                                                 }),
    //                                                 CSTNode::Terminal(Terminal {
    //                                                     id: uuid::Uuid::new_v4(),
    //                                                     kind: "identifier",
    //                                                     value: "name",
    //                                                     start_position: Point {
    //                                                         row: 2,
    //                                                         column: 37,
    //                                                     },
    //                                                     end_position: Point { row: 2, column: 41 },
    //                                                     is_block_end_delimiter: false,
    //                                                 }),
    //                                             ],
    //                                             start_position: Point { row: 2, column: 30 },
    //                                             end_position: Point { row: 2, column: 41 },
    //                                         }),
    //                                         CSTNode::Terminal(Terminal {
    //                                             id: uuid::Uuid::new_v4(),
    //                                             kind: ")",
    //                                             value: ")",
    //                                             start_position: Point { row: 2, column: 41 },
    //                                             end_position: Point { row: 2, column: 42 },
    //                                             is_block_end_delimiter: false,
    //                                         }),
    //                                     ],
    //                                     start_position: Point { row: 2, column: 29 },
    //                                     end_position: Point { row: 2, column: 42 },
    //                                 }),
    //                                 CSTNode::Terminal(Terminal {
    //                                     id: uuid::Uuid::new_v4(),
    //                                     kind: ";",
    //                                     value: ";",
    //                                     start_position: Point { row: 2, column: 42 },
    //                                     end_position: Point { row: 2, column: 43 },
    //                                     is_block_end_delimiter: false,
    //                                 }),
    //                             ],
    //                             start_position: Point { row: 2, column: 16 },
    //                             end_position: Point { row: 2, column: 43 },
    //                         }),
    //                         CSTNode::Terminal(Terminal {
    //                             id: uuid::Uuid::new_v4(),
    //                             kind: "}",
    //                             value: "}",
    //                             start_position: Point { row: 3, column: 12 },
    //                             end_position: Point { row: 3, column: 13 },
    //                             is_block_end_delimiter: true,
    //                         }),
    //                     ],
    //                     start_position: Point { row: 1, column: 47 },
    //                     end_position: Point { row: 3, column: 13 },
    //                 }),
    //             ],
    //             start_position: Point { row: 1, column: 12 },
    //             end_position: Point { row: 3, column: 13 },
    //         })],
    //         start_position: Point { row: 1, column: 12 },
    //         end_position: Point { row: 4, column: 8 },
    //     });
    //     assert_eq!(expected, result.unwrap())
    // }
    //
    // #[test]
    // fn it_stops_the_compilation_when_reach_a_configured_node() {
    //     let code = "public static interface HelloWorld {void sayHello(String name);}";
    //     let parser_configuration = ParserConfiguration {
    //         language: tree_sitter_java::language(),
    //         stop_compilation_at: ["interface_body"].into_iter().collect(),
    //         kinds_with_unordered_children: [].into(),
    //         block_end_delimiters: ["}"].into(),
    //     };
    //     let result = parse_string(code, &parser_configuration);
    //
    //     let expected = CSTNode::NonTerminal(NonTerminal {
    //         id: uuid::Uuid::new_v4(),
    //         kind: "program",
    //         are_children_unordered: false,
    //         children: vec![CSTNode::NonTerminal(NonTerminal {
    //             id: uuid::Uuid::new_v4(),
    //             kind: "interface_declaration",
    //             are_children_unordered: false,
    //             children: vec![
    //                 CSTNode::NonTerminal(NonTerminal {
    //                     id: uuid::Uuid::new_v4(),
    //                     kind: "modifiers",
    //                     are_children_unordered: false,
    //                     children: vec![
    //                         CSTNode::Terminal(Terminal {
    //                             id: uuid::Uuid::new_v4(),
    //                             kind: "public",
    //                             value: "public",
    //                             start_position: Point { row: 0, column: 0 },
    //                             end_position: Point { row: 0, column: 6 },
    //                             is_block_end_delimiter: false,
    //                         }),
    //                         CSTNode::Terminal(Terminal {
    //                             id: uuid::Uuid::new_v4(),
    //                             kind: "static",
    //                             value: "static",
    //                             start_position: Point { row: 0, column: 7 },
    //                             end_position: Point { row: 0, column: 13 },
    //                             is_block_end_delimiter: false,
    //                         }),
    //                     ],
    //                     start_position: Point { row: 0, column: 0 },
    //                     end_position: Point { row: 0, column: 13 },
    //                 }),
    //                 CSTNode::Terminal(Terminal {
    //                     id: uuid::Uuid::new_v4(),
    //                     kind: "interface",
    //                     value: "interface",
    //                     start_position: Point { row: 0, column: 14 },
    //                     end_position: Point { row: 0, column: 23 },
    //                     is_block_end_delimiter: false,
    //                 }),
    //                 CSTNode::Terminal(Terminal {
    //                     id: uuid::Uuid::new_v4(),
    //                     kind: "identifier",
    //                     value: "HelloWorld",
    //                     start_position: Point { row: 0, column: 24 },
    //                     end_position: Point { row: 0, column: 34 },
    //                     is_block_end_delimiter: false,
    //                 }),
    //                 CSTNode::Terminal(Terminal {
    //                     id: uuid::Uuid::new_v4(),
    //                     kind: "interface_body",
    //                     value: "{void sayHello(String name);}",
    //                     start_position: Point { row: 0, column: 35 },
    //                     end_position: Point { row: 0, column: 64 },
    //                     is_block_end_delimiter: false,
    //                 }),
    //             ],
    //             start_position: Point { row: 0, column: 0 },
    //             end_position: Point { row: 0, column: 64 },
    //         })],
    //         start_position: Point { row: 0, column: 0 },
    //         end_position: Point { row: 0, column: 64 },
    //     });
    //     assert_eq!(expected, result.unwrap())
    // }
    //
    // #[test]
    // fn it_marks_nodes_with_unordered_children_as_unordered() {
    //     let code = "public static interface HelloWorld {void sayHello(String name);}";
    //     let parser_configuration = ParserConfiguration {
    //         language: tree_sitter_java::language(),
    //         stop_compilation_at: ["method_declaration"].into_iter().collect(),
    //         kinds_with_unordered_children: ["interface_body"].into(),
    //         block_end_delimiters: ["}"].into(),
    //     };
    //     let result = parse_string(code, &parser_configuration);
    //
    //     let expected = CSTNode::NonTerminal(NonTerminal {
    //         id: uuid::Uuid::new_v4(),
    //         kind: "program",
    //         are_children_unordered: false,
    //         children: vec![CSTNode::NonTerminal(NonTerminal {
    //             id: uuid::Uuid::new_v4(),
    //             kind: "interface_declaration",
    //             are_children_unordered: false,
    //             children: vec![
    //                 CSTNode::NonTerminal(NonTerminal {
    //                     kind: "modifiers",
    //                     are_children_unordered: false,
    //                     children: vec![
    //                         CSTNode::Terminal(Terminal {
    //                             kind: "public",
    //                             value: "public",
    //                             start_position: Point { row: 0, column: 0 },
    //                             end_position: Point { row: 0, column: 6 },
    //                             is_block_end_delimiter: false,
    //                         }),
    //                         CSTNode::Terminal(Terminal {
    //                             kind: "static",
    //                             value: "static",
    //                             start_position: Point { row: 0, column: 7 },
    //                             end_position: Point { row: 0, column: 13 },
    //                             is_block_end_delimiter: false,
    //                         }),
    //                     ],
    //                     start_position: Point { row: 0, column: 0 },
    //                     end_position: Point { row: 0, column: 13 },
    //                 }),
    //                 CSTNode::Terminal(Terminal {
    //                     kind: "interface",
    //                     value: "interface",
    //                     start_position: Point { row: 0, column: 14 },
    //                     end_position: Point { row: 0, column: 23 },
    //                     is_block_end_delimiter: false,
    //                 }),
    //                 CSTNode::Terminal(Terminal {
    //                     kind: "identifier",
    //                     value: "HelloWorld",
    //                     start_position: Point { row: 0, column: 24 },
    //                     end_position: Point { row: 0, column: 34 },
    //                     is_block_end_delimiter: false,
    //                 }),
    //                 CSTNode::NonTerminal(NonTerminal {
    //                     kind: "interface_body",
    //                     children: vec![
    //                         CSTNode::Terminal(Terminal {
    //                             kind: "{",
    //                             value: "{",
    //                             start_position: Point { row: 0, column: 35 },
    //                             end_position: Point { row: 0, column: 36 },
    //                             is_block_end_delimiter: false,
    //                         }),
    //                         CSTNode::Terminal(Terminal {
    //                             kind: "method_declaration",
    //                             value: "void sayHello(String name);",
    //                             start_position: Point { row: 0, column: 36 },
    //                             end_position: Point { row: 0, column: 63 },
    //                             is_block_end_delimiter: false,
    //                         }),
    //                         CSTNode::Terminal(Terminal {
    //                             kind: "}",
    //                             value: "}",
    //                             start_position: Point { row: 0, column: 63 },
    //                             end_position: Point { row: 0, column: 64 },
    //                             is_block_end_delimiter: true,
    //                         }),
    //                     ],
    //                     start_position: Point { row: 0, column: 35 },
    //                     end_position: Point { row: 0, column: 64 },
    //                     are_children_unordered: true,
    //                 }),
    //             ],
    //             start_position: Point { row: 0, column: 0 },
    //             end_position: Point { row: 0, column: 64 },
    //         })],
    //         start_position: Point { row: 0, column: 0 },
    //         end_position: Point { row: 0, column: 64 },
    //     });
    //     assert_eq!(expected, result.unwrap())
    // }
}
