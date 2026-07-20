use std::cell::OnceCell;

use model::{cst_node::NonTerminal, CSTNode};

pub fn tweak_source_file(root: CSTNode<'_>) -> CSTNode<'_> {
    match root {
        CSTNode::NonTerminal(source_file) if root.kind() == "source_file" => {
            let (head, tail): (Vec<_>, Vec<_>) = source_file
                .children
                .iter()
                .map(|v| v.to_owned())
                .partition(|node| {
                    node.kind() == "package_clause" || node.kind() == "import_declaration"
                });

            if tail.is_empty() {
                return CSTNode::NonTerminal(source_file);
            }

            log::debug!("Transforming source_file node for better matching. Found {:?} reserved and {:?} non-reserved nodes", head.len(), tail.len());

            let start_position = tail
                .first()
                .expect("Tail should not be empty")
                .start_position();
            let end_position = tail
                .last()
                .expect("Tail should not be empty")
                .end_position();

            let source_file_tail_node = CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "source_file_synthetic_tail",
                children: tail,
                start_position,
                end_position,
                are_children_unordered: true,
                identifier: None,
                leading_white_space: None,
                delimiters: None,
                subtree_size_without_delimiters: OnceCell::new(),
            });

            let mut new_program_children: Vec<CSTNode<'_>> = vec![];
            new_program_children.extend(head);
            new_program_children.push(source_file_tail_node);

            CSTNode::NonTerminal(NonTerminal {
                id: source_file.id,
                kind: source_file.kind,
                start_position: source_file.start_position,
                end_position: source_file.end_position,
                children: new_program_children,
                are_children_unordered: source_file.are_children_unordered,
                identifier: source_file.identifier,
                leading_white_space: None,
                delimiters: source_file.delimiters,
                subtree_size_without_delimiters: OnceCell::new(),
            })
        }
        _ => root,
    }
}
