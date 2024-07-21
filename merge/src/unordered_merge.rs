use std::collections::HashSet;

use matching::Matchings;
use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode,
};

use crate::{merge, MergeError, MergedCSTNode};

pub fn unordered_merge<'a>(
    left: &'a NonTerminal<'a>,
    right: &'a NonTerminal<'a>,
    base_left_matchings: &'a Matchings<'a>,
    base_right_matchings: &'a Matchings<'a>,
    left_right_matchings: &'a Matchings<'a>,
) -> Result<MergedCSTNode<'a>, MergeError> {
    // Nodes of different kind, early return
    if left.kind != right.kind {
        return Err(MergeError::NodesWithDifferentKinds(
            left.kind.to_string(),
            right.kind.to_string(),
        ));
    }

    let mut result_children = vec![];
    let mut processed_nodes: HashSet<uuid::Uuid> = HashSet::new();

    for left_child in left.children.iter() {
        if let CSTNode::Terminal(Terminal {
            is_block_end_delimiter,
            ..
        }) = left_child
        {
            if *is_block_end_delimiter {
                break;
            }
        }

        let matching_base_left = base_left_matchings.find_matching_for(left_child);
        let matching_left_right = left_right_matchings.find_matching_for(left_child);

        match (matching_base_left, matching_left_right) {
            // Added only by left
            (None, None) => {
                result_children.push(left_child.to_owned().into());
                processed_nodes.insert(left_child.id());
            }
            (None, Some(right_matching)) => {
                result_children.push(merge(
                    left_child,
                    left_child,
                    right_matching.matching_node,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                )?);
                processed_nodes.insert(left_child.id());
                processed_nodes.insert(right_matching.matching_node.id());
            }
            // Removed in right
            (Some(matching_base_left), None) => {
                // Changed in left, conflict!
                if !matching_base_left.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: Some(Box::new(left_child.to_owned().into())),
                        right: None,
                    })
                }
                processed_nodes.insert(left_child.id());
            }
            (Some(_), Some(right_matching)) => {
                result_children.push(merge(
                    left_child,
                    left_child,
                    right_matching.matching_node,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                )?);
                processed_nodes.insert(left_child.id());
                processed_nodes.insert(right_matching.matching_node.id());
            }
        }
    }

    for right_child in right
        .children
        .iter()
        .filter(|node| !processed_nodes.contains(&node.id()))
    {
        let matching_base_right = base_right_matchings.find_matching_for(right_child);
        let matching_left_right = left_right_matchings.find_matching_for(right_child);

        match (matching_base_right, matching_left_right) {
            // Added only by right
            (None, None) => {
                result_children.push(right_child.to_owned().into());
            }
            (None, Some(matching_left_right)) => {
                result_children.push(merge(
                    right_child,
                    matching_left_right.matching_node,
                    right_child,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                )?);
            }
            // Removed in left
            (Some(matching_base_right), None) => {
                // Changed in right, conflict!
                if !matching_base_right.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(right_child.to_owned().into())),
                    })
                }
            }
            (Some(_), Some(matching_left_right)) => {
                result_children.push(merge(
                    right_child,
                    matching_left_right.matching_node,
                    right_child,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                )?);
            }
        }
    }

    Ok(MergedCSTNode::NonTerminal {
        kind: left.kind,
        children: result_children,
    })
}

#[cfg(test)]
mod tests {
    use matching::{unordered::calculate_matchings, Matchings};
    use model::{
        cst_node::{NonTerminal, Terminal},
        CSTNode, Language, Point,
    };

    use crate::{MergeError, MergedCSTNode};

    use super::unordered_merge;

    fn assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
        base: &CSTNode,
        parent_a: &CSTNode,
        parent_b: &CSTNode,
        expected_merge: &MergedCSTNode,
    ) -> Result<(), MergeError> {
        let matchings_base_parent_a = calculate_matchings(base, parent_a);
        let matchings_base_parent_b = calculate_matchings(base, parent_b);
        let matchings_parents = calculate_matchings(parent_a, parent_b);

        let merged_tree = unordered_merge(
            parent_a.try_into().unwrap(),
            parent_b.try_into().unwrap(),
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
        )?;
        let merged_tree_swap = unordered_merge(
            parent_b.try_into().unwrap(),
            parent_a.try_into().unwrap(),
            &matchings_base_parent_b,
            &matchings_base_parent_a,
            &matchings_parents,
        )?;

        assert_eq!(expected_merge, &merged_tree);
        assert_eq!(expected_merge, &merged_tree_swap);

        Ok(())
    }

    fn assert_merge_output_is(
        base: &CSTNode,
        parent_a: &CSTNode,
        parent_b: &CSTNode,
        expected_merge: &MergedCSTNode,
    ) -> Result<(), MergeError> {
        let matchings_base_parent_a = calculate_matchings(base, parent_a);
        let matchings_base_parent_b = calculate_matchings(base, parent_b);
        let matchings_parents = calculate_matchings(parent_a, parent_b);

        let merged_tree = unordered_merge(
            parent_a.try_into().unwrap(),
            parent_b.try_into().unwrap(),
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
        )?;

        assert_eq!(expected_merge, &merged_tree);

        Ok(())
    }

    #[test]
    fn test_merge_node_added_only_by_one_parent() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 1, column: 1 },
                    end_position: model::Point { row: 1, column: 1 },
                    is_block_end_delimiter: true,
                }),
            ],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "method_declaration",
                    value: "main",
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                    is_block_end_delimiter: true,
                }),
            ],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 1, column: 1 },
                    end_position: model::Point { row: 1, column: 1 },
                    is_block_end_delimiter: true,
                }),
            ],
            ..Default::default()
        });

        let merge = MergedCSTNode::NonTerminal {
            kind: "interface_body",
            children: vec![
                MergedCSTNode::Terminal {
                    kind: "{",
                    value: String::from("{"),
                },
                MergedCSTNode::Terminal {
                    kind: "method_declaration",
                    value: String::from("main"),
                },
                MergedCSTNode::Terminal {
                    kind: "}",
                    value: String::from("}"),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base, &parent_a, &parent_b, &merge,
        )
    }

    #[test]
    fn test_both_parents_add_the_same_node_and_both_subtrees_are_equal() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 1, column: 1 },
                    end_position: model::Point { row: 1, column: 1 },
                    is_block_end_delimiter: true,
                }),
            ],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "a_method_declaration",
                    are_children_unordered: false,
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    children: vec![CSTNode::Terminal(Terminal {
                        id: uuid::Uuid::new_v4(),
                        kind: "identifier",
                        value: "main",
                        start_position: model::Point { row: 0, column: 1 },
                        end_position: model::Point { row: 0, column: 1 },
                        is_block_end_delimiter: false,
                    })],
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                    is_block_end_delimiter: true,
                }),
            ],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "a_method_declaration",
                    are_children_unordered: false,
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    children: vec![CSTNode::Terminal(Terminal {
                        id: uuid::Uuid::new_v4(),
                        kind: "identifier",
                        value: "main",
                        start_position: model::Point { row: 0, column: 1 },
                        end_position: model::Point { row: 0, column: 1 },
                        is_block_end_delimiter: false,
                    })],
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                    is_block_end_delimiter: true,
                }),
            ],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "interface_body",
            children: vec![
                MergedCSTNode::Terminal {
                    kind: "{",
                    value: String::from("{"),
                },
                MergedCSTNode::NonTerminal {
                    kind: "a_method_declaration",
                    children: vec![MergedCSTNode::Terminal {
                        kind: "identifier",
                        value: String::from("main"),
                    }],
                },
                MergedCSTNode::Terminal {
                    kind: "}",
                    value: String::from("}"),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &parent_a,
            &parent_b,
            &expected_merge,
        )
    }

    #[test]
    fn test_merge_one_parent_removes_a_node_while_the_other_keeps_it_unchanged(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "method_declaration",
                    are_children_unordered: false,
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    children: vec![
                        CSTNode::Terminal(Terminal {
                            id: uuid::Uuid::new_v4(),
                            kind: "formal_parameters",
                            value: "formal_parameters",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                            is_block_end_delimiter: false,
                        }),
                        CSTNode::Terminal(Terminal {
                            id: uuid::Uuid::new_v4(),
                            kind: "identifier",
                            value: "main",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                            is_block_end_delimiter: false,
                        }),
                    ],
                    identifier: Some(vec!["main"]),
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 1, column: 1 },
                    end_position: model::Point { row: 1, column: 1 },
                    is_block_end_delimiter: true,
                }),
            ],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "method_declaration",
                    are_children_unordered: false,
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    children: vec![
                        CSTNode::Terminal(Terminal {
                            id: uuid::Uuid::new_v4(),
                            kind: "formal_parameters",
                            value: "formal_parameters",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                            is_block_end_delimiter: false,
                        }),
                        CSTNode::Terminal(Terminal {
                            id: uuid::Uuid::new_v4(),
                            kind: "identifier",
                            value: "main",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                            is_block_end_delimiter: false,
                        }),
                    ],
                    identifier: Some(vec!["main"]),
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                    is_block_end_delimiter: true,
                }),
            ],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                    is_block_end_delimiter: true,
                }),
            ],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "interface_body",
            children: vec![
                MergedCSTNode::Terminal {
                    kind: "{",
                    value: String::from("{"),
                },
                MergedCSTNode::Terminal {
                    kind: "}",
                    value: String::from("}"),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &parent_a,
            &parent_b,
            &expected_merge,
        )
    }

    #[test]
    fn test_merge_one_parent_removes_a_node_while_the_other_changed_it() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "method_declaration",
                    are_children_unordered: false,
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    children: vec![
                        CSTNode::Terminal(Terminal {
                            id: uuid::Uuid::new_v4(),
                            kind: "formal_parameters",
                            value: "formal_parameters",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                            is_block_end_delimiter: false,
                        }),
                        CSTNode::Terminal(Terminal {
                            id: uuid::Uuid::new_v4(),
                            kind: "identifier",
                            value: "method",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                            is_block_end_delimiter: false,
                        }),
                        CSTNode::Terminal(Terminal {
                            id: uuid::Uuid::new_v4(),
                            kind: "kind_a",
                            value: "value_a",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                            is_block_end_delimiter: false,
                        }),
                        CSTNode::Terminal(Terminal {
                            id: uuid::Uuid::new_v4(),
                            kind: "kind_b",
                            value: "value_b",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                            is_block_end_delimiter: false,
                        }),
                    ],
                    identifier: Some(vec!["method"]),
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 1, column: 1 },
                    end_position: model::Point { row: 1, column: 1 },
                    is_block_end_delimiter: true,
                }),
            ],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "method_declaration",
                    are_children_unordered: false,
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    children: vec![
                        CSTNode::Terminal(Terminal {
                            id: uuid::Uuid::new_v4(),
                            kind: "formal_parameters",
                            value: "formal_parameters",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                            is_block_end_delimiter: false,
                        }),
                        CSTNode::Terminal(Terminal {
                            id: uuid::Uuid::new_v4(),
                            kind: "identifier",
                            value: "method",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                            is_block_end_delimiter: false,
                        }),
                        CSTNode::Terminal(Terminal {
                            id: uuid::Uuid::new_v4(),
                            kind: "kind_a",
                            value: "value_a",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                            is_block_end_delimiter: false,
                        }),
                        CSTNode::Terminal(Terminal {
                            id: uuid::Uuid::new_v4(),
                            kind: "kind_b",
                            value: "new_value_b",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                            is_block_end_delimiter: false,
                        }),
                    ],
                    identifier: Some(vec!["method"]),
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                    is_block_end_delimiter: true,
                }),
            ],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                    is_block_end_delimiter: true,
                }),
            ],
            ..Default::default()
        });

        assert_merge_output_is(
            &base,
            &parent_a,
            &parent_b,
            &MergedCSTNode::NonTerminal {
                kind: "interface_body",
                children: vec![
                    MergedCSTNode::Terminal {
                        kind: "{",
                        value: String::from("{"),
                    },
                    MergedCSTNode::Conflict {
                        left: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "method_declaration",
                            children: vec![
                                MergedCSTNode::Terminal {
                                    kind: "formal_parameters",
                                    value: String::from("formal_parameters"),
                                },
                                MergedCSTNode::Terminal {
                                    kind: "identifier",
                                    value: String::from("method"),
                                },
                                MergedCSTNode::Terminal {
                                    kind: "kind_a",
                                    value: String::from("value_a"),
                                },
                                MergedCSTNode::Terminal {
                                    kind: "kind_b",
                                    value: String::from("new_value_b"),
                                },
                            ],
                        })),
                        right: None,
                    },
                    MergedCSTNode::Terminal {
                        kind: "}",
                        value: String::from("}"),
                    },
                ],
            },
        )?;
        assert_merge_output_is(
            &base,
            &parent_b,
            &parent_a,
            &MergedCSTNode::NonTerminal {
                kind: "interface_body",
                children: vec![
                    MergedCSTNode::Terminal {
                        kind: "{",
                        value: String::from("{"),
                    },
                    MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "method_declaration",
                            children: vec![
                                MergedCSTNode::Terminal {
                                    kind: "formal_parameters",
                                    value: String::from("formal_parameters"),
                                },
                                MergedCSTNode::Terminal {
                                    kind: "identifier",
                                    value: String::from("method"),
                                },
                                MergedCSTNode::Terminal {
                                    kind: "kind_a",
                                    value: String::from("value_a"),
                                },
                                MergedCSTNode::Terminal {
                                    kind: "kind_b",
                                    value: String::from("new_value_b"),
                                },
                            ],
                        })),
                    },
                    MergedCSTNode::Terminal {
                        kind: "}",
                        value: String::from("}"),
                    },
                ],
            },
        )
    }

    #[test]
    fn i_get_an_error_if_i_try_to_merge_nodes_of_different_kinds() {
        let kind_a = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            are_children_unordered: true,
            ..Default::default()
        };
        let kind_b = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            are_children_unordered: true,
            ..Default::default()
        };

        let matchings = Matchings::empty();
        let result = unordered_merge(&kind_a, &kind_b, &matchings, &matchings, &matchings);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MergeError::NodesWithDifferentKinds("kind_a".to_string(), "kind_b".to_string())
        );
    }
}
