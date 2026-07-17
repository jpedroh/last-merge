use model::{cst_node::NonTerminal, CSTNode};

pub fn tweak_declarations_list(root: CSTNode<'_>) -> CSTNode<'_> {
    if root.kind() != "const_declaration" {
        return root.to_owned();
    }

    match root {
        CSTNode::Terminal(_) => root,
        CSTNode::NonTerminal(const_declaration) => {
            let internal_declarations: Vec<_> = const_declaration
                .children
                .iter()
                .map(|node| node.to_owned())
                .filter(|node| node.kind() == "const_spec")
                .collect();

            if internal_declarations.len() <= 1 {
                CSTNode::NonTerminal(const_declaration)
            } else {
                CSTNode::NonTerminal(NonTerminal {
                    id: const_declaration.id,
                    kind: const_declaration.kind,
                    start_position: const_declaration.start_position,
                    end_position: const_declaration.end_position,
                    children: internal_declarations,
                    are_children_unordered: true,
                    identifier: None,
                    leading_white_space: const_declaration.leading_white_space,
                    delimiters: const_declaration.delimiters,
                })
            }
        }
    }
}
