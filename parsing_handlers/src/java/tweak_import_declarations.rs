use model::{cst_node::NonTerminal, CSTNode};

pub fn tweak_import_declarations(root: CSTNode<'_>) -> CSTNode<'_> {
    if root.kind() != "program" {
        return root.to_owned();
    }

    match root {
        CSTNode::Terminal(_) => root,
        CSTNode::NonTerminal(program) => {
            let import_declaration_children: Vec<CSTNode> = program
                .children
                .iter()
                .filter(|node| node.kind() == "import_declaration")
                .cloned()
                .collect();

            if import_declaration_children.is_empty() {
                return CSTNode::NonTerminal(program);
            }

            let first_import_declaration = import_declaration_children.first().unwrap().clone();

            let import_declarations_start = first_import_declaration.start_position();

            let import_declarations_end =
                import_declaration_children.last().unwrap().end_position();

            let import_declarations = CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "import_declarations",
                children: import_declaration_children,
                start_position: import_declarations_start,
                end_position: import_declarations_end,
                are_children_unordered: true,
                identifier: None,
                leading_white_space: Some("\n"),
                delimiters: None,
            });

            let first_import_declaration_index = program
                .children
                .iter()
                .position(|node| node.kind() == "import_declaration")
                .unwrap();
            let last_import_declaration_index = program
                .children
                .iter()
                .rposition(|node| node.kind() == "import_declaration")
                .unwrap();

            let mut new_program_children: Vec<CSTNode<'_>> = vec![];
            new_program_children.extend_from_slice(
                &program.children.iter().as_slice()[..first_import_declaration_index],
            );
            new_program_children.push(import_declarations);
            new_program_children.extend_from_slice(
                &program.children.iter().as_slice()[last_import_declaration_index + 1..],
            );

            CSTNode::NonTerminal(NonTerminal {
                id: program.id,
                kind: program.kind,
                start_position: program.start_position,
                end_position: program.end_position,
                children: new_program_children,
                are_children_unordered: program.are_children_unordered,
                identifier: program.identifier,
                leading_white_space: None,
                delimiters: program.delimiters,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use model::{cst_node::Terminal, CSTNode};

    #[test]
    fn if_the_root_is_not_a_program_we_just_return_it() {
        let root = CSTNode::Terminal(Terminal {
            kind: "terminal",
            value: "not_a_program",
            ..Default::default()
        });

        assert_eq!(super::tweak_import_declarations(root.clone()), root);
    }

    #[test]
    fn if_somehow_the_root_is_a_terminal_node_we_just_return_it() {
        let root = CSTNode::Terminal(Terminal {
            kind: "program",
            ..Default::default()
        });

        assert_eq!(super::tweak_import_declarations(root.clone()), root);
    }
}
