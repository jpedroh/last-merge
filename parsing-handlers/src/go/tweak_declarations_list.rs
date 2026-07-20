use model::{cst_node::NonTerminal, CSTNode};

pub fn tweak_declarations_list(root: CSTNode<'_>) -> CSTNode<'_> {
    match root {
        CSTNode::NonTerminal(nt) if nt.kind == "const_declaration" => {
            handle(nt, "const_spec", "const_spec_list")
        }
        CSTNode::NonTerminal(nt) if nt.kind == "type_declaration" => {
            handle(nt, "type_spec", "type_spec_list")
        }
        _ => root,
    }
}

fn handle<'a>(
    declaration: NonTerminal<'a>,
    child_name: &'static str,
    new_kind: &'static str,
) -> CSTNode<'a> {
    let NonTerminal {
        id,
        kind,
        children,
        start_position,
        end_position,
        are_children_unordered,
        identifier,
        leading_white_space,
        delimiters,
        subtree_size_without_delimiters,
    } = declaration;

    let internal_declaration_count = children
        .iter()
        .filter(|node| node.kind() == child_name)
        .count();

    log::debug!(
        "Found {:?} declarations of type {:?}",
        internal_declaration_count,
        child_name
    );

    if internal_declaration_count <= 1 {
        CSTNode::NonTerminal(NonTerminal {
            id,
            kind,
            children,
            start_position,
            end_position,
            are_children_unordered,
            identifier,
            leading_white_space,
            delimiters,
            subtree_size_without_delimiters,
        })
    } else {
        let trailing_nodes: Vec<_> = children.iter().take(2).cloned().collect();
        let final_node = children.last().cloned().expect("List should not be empty");

        let internal_declarations: Vec<_> = children
            .into_iter()
            .filter(|node| node.kind() == child_name)
            .collect();

        let declaration_list_node = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: new_kind,
            start_position: internal_declarations
                .first()
                .expect("Should not be empty")
                .start_position(),
            end_position: internal_declarations
                .last()
                .expect("Should not be empty")
                .end_position(),
            children: internal_declarations,
            are_children_unordered: true,
            identifier: None,
            leading_white_space: None,
            delimiters: None,
            subtree_size_without_delimiters: subtree_size_without_delimiters.clone(),
        });

        let mut resulting_children = vec![];
        resulting_children.extend(trailing_nodes);
        resulting_children.push(declaration_list_node);
        resulting_children.push(final_node);

        CSTNode::NonTerminal(NonTerminal {
            id,
            kind,
            children: resulting_children,
            start_position,
            end_position,
            are_children_unordered,
            identifier,
            leading_white_space,
            delimiters,
            subtree_size_without_delimiters,
        })
    }
}

#[cfg(test)]
mod tests {
    use model::{
        cst_node::{NonTerminal, Point, Terminal},
        CSTNode,
    };

    fn const_spec(kind: &'static str) -> CSTNode<'static> {
        CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind,
            start_position: Point { row: 1, column: 1 },
            end_position: Point { row: 1, column: 1 },
            children: vec![],
            are_children_unordered: false,
            identifier: None,
            leading_white_space: None,
            delimiters: None,
            subtree_size_without_delimiters: std::cell::OnceCell::new(),
        })
    }

    #[test]
    fn it_leaves_non_const_declarations_unchanged() {
        let root = CSTNode::Terminal(Terminal {
            kind: "program",
            ..Default::default()
        });

        assert_eq!(super::tweak_declarations_list(root.clone()), root);
    }

    #[test]
    fn it_leaves_single_const_spec_declarations_unchanged() {
        let root = CSTNode::NonTerminal(NonTerminal {
            kind: "const_declaration",
            children: vec![const_spec("const_spec"), const_spec("other")],
            ..Default::default()
        });

        assert_eq!(super::tweak_declarations_list(root.clone()), root);
    }

    #[test]
    fn it_extracts_multiple_const_specs_without_cloning_unrelated_children() {
        let root = CSTNode::NonTerminal(NonTerminal {
            kind: "const_declaration",
            children: vec![
                const_spec("const_spec"),
                const_spec("other"),
                const_spec("const_spec"),
            ],
            ..Default::default()
        });

        let tweaked = super::tweak_declarations_list(root);

        match tweaked {
            CSTNode::NonTerminal(non_terminal) => {
                assert_eq!(non_terminal.kind, "const_declaration");
                assert!(!non_terminal.are_children_unordered);
                assert_eq!(non_terminal.children.len(), 4);
                assert_eq!(non_terminal.children[0].kind(), "const_spec");
                assert_eq!(non_terminal.children[1].kind(), "other");
                assert_eq!(non_terminal.children[2].kind(), "const_spec_list");
                assert_eq!(non_terminal.children[3].kind(), "const_spec");

                match &non_terminal.children[2] {
                    CSTNode::NonTerminal(list_node) => {
                        assert!(list_node.are_children_unordered);
                        assert_eq!(list_node.kind, "const_spec_list");
                        assert_eq!(list_node.children.len(), 2);
                        assert!(list_node
                            .children
                            .iter()
                            .all(|node| node.kind() == "const_spec"));
                    }
                    CSTNode::Terminal(_) => panic!("expected synthetic list node"),
                }
            }
            CSTNode::Terminal(_) => panic!("expected non-terminal result"),
        }
    }
}
