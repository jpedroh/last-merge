use model::{cst_node::NonTerminal, CSTNode};

pub fn tweak_declarations_list(root: CSTNode<'_>) -> CSTNode<'_> {
    match root {
        CSTNode::Terminal(_) => root,
        CSTNode::NonTerminal(non_terminal) if non_terminal.kind != "const_declaration" => {
            CSTNode::NonTerminal(non_terminal)
        }
        CSTNode::NonTerminal(const_declaration) => {
            let internal_declaration_count = const_declaration
                .children
                .iter()
                .filter(|node| node.kind() == "const_spec")
                .count();

            if internal_declaration_count <= 1 {
                CSTNode::NonTerminal(const_declaration)
            } else {
                let NonTerminal {
                    id,
                    kind,
                    children,
                    start_position,
                    end_position,
                    are_children_unordered: _,
                    identifier: _,
                    leading_white_space,
                    delimiters,
                } = const_declaration;

                let internal_declarations: Vec<_> = children
                    .into_iter()
                    .filter(|node| node.kind() == "const_spec")
                    .collect();

                CSTNode::NonTerminal(NonTerminal {
                    id,
                    kind,
                    start_position,
                    end_position,
                    children: internal_declarations,
                    are_children_unordered: true,
                    identifier: None,
                    leading_white_space,
                    delimiters,
                })
            }
        }
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
                assert!(non_terminal.are_children_unordered);
                assert_eq!(non_terminal.children.len(), 2);
                assert!(non_terminal
                    .children
                    .iter()
                    .all(|node| node.kind() == "const_spec"));
            }
            CSTNode::Terminal(_) => panic!("expected non-terminal result"),
        }
    }
}
