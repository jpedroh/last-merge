use model::{cst_node::NonTerminal, CSTNode};

pub fn remove_block_comments(root: CSTNode<'_>) -> CSTNode<'_> {
    match root {
        CSTNode::Terminal(_) => root,
        CSTNode::NonTerminal(non_terminal) => CSTNode::NonTerminal(NonTerminal {
            id: non_terminal.id,
            kind: non_terminal.kind,
            start_position: non_terminal.start_position,
            end_position: non_terminal.end_position,
            children: non_terminal
                .children
                .into_iter()
                .filter(|node| node.kind() != "block_comment" && node.kind() != "line_comment")
                .map(|node| remove_block_comments(node))
                .collect(),
            are_children_unordered: non_terminal.are_children_unordered,
            identifier: non_terminal.identifier,
            leading_white_space: non_terminal.leading_white_space,
            delimiters: non_terminal.delimiters,
        }),
    }
}

#[cfg(test)]
mod tests {
    use model::{
        cst_node::{NonTerminal, Terminal},
        CSTNode,
    };

    #[test]
    fn it_removes_first_level_comments() {
        let root = CSTNode::NonTerminal(NonTerminal {
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "block_comment",
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    kind: "line_comment",
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    kind: "program",
                    ..Default::default()
                }),
            ],
            ..Default::default()
        });

        let expected_root = CSTNode::NonTerminal(NonTerminal {
            children: vec![CSTNode::Terminal(Terminal {
                kind: "program",
                ..Default::default()
            })],
            ..Default::default()
        });

        assert_eq!(
            super::remove_block_comments(root).contents(),
            expected_root.contents()
        );
    }

    #[test]
    fn it_removes_deep_comments() {
        let root = CSTNode::NonTerminal(NonTerminal {
            children: vec![CSTNode::NonTerminal(NonTerminal {
                children: vec![
                    CSTNode::Terminal(Terminal {
                        kind: "block_comment",
                        ..Default::default()
                    }),
                    CSTNode::Terminal(Terminal {
                        kind: "line_comment",
                        ..Default::default()
                    }),
                    CSTNode::Terminal(Terminal {
                        kind: "program",
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            })],
            ..Default::default()
        });

        let expected_root = CSTNode::NonTerminal(NonTerminal {
            children: vec![CSTNode::NonTerminal(NonTerminal {
                children: vec![CSTNode::Terminal(Terminal {
                    kind: "program",
                    ..Default::default()
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        assert_eq!(
            super::remove_block_comments(root).contents(),
            expected_root.contents()
        );
    }
}
