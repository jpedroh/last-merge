use matching::Matchings;
use model::cst_node::NonTerminal;

use crate::{MergeError, MergedCSTNode};

pub fn ordered_merge<'a>(
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

    let mut children_left_it = left.get_children().iter();
    let mut children_right_it = right.get_children().iter();

    let mut cur_left_option = children_left_it.next();
    let mut cur_right_option = children_right_it.next();

    while let (Some(cur_left), Some(cur_right)) = (cur_left_option, cur_right_option) {
        let matching_base_left = base_left_matchings.find_matching_for(cur_left);
        let matching_base_right = base_right_matchings.find_matching_for(cur_right);
        let left_matching_in_right = left_right_matchings.find_matching_for(cur_left);
        let right_matching_in_left = left_right_matchings.find_matching_for(cur_right);
        let has_bidirectional_matching_left_right =
            left_right_matchings.has_bidirectional_matching(cur_left, cur_right);

        match (
            has_bidirectional_matching_left_right,
            left_matching_in_right,
            matching_base_left,
            right_matching_in_left,
            matching_base_right,
        ) {
            (true, Some(_), Some(_), Some(_), Some(_)) => {
                result_children.push(crate::merge(
                    cur_left,
                    cur_left,
                    cur_right,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                )?);

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (true, Some(_), None, Some(_), None) => {
                result_children.push(crate::merge(
                    cur_left,
                    cur_left,
                    cur_right,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                )?);

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (false, Some(_), Some(_), None, Some(matching_base_right)) => {
                if !matching_base_right.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(cur_right.to_owned().into())),
                    });
                }

                cur_right_option = children_right_it.next();
            }
            (false, Some(_), Some(_), None, None) => {
                result_children.push(cur_right.to_owned().into());

                cur_right_option = children_right_it.next();
            }
            (false, Some(_), None, None, Some(matching_base_right)) => {
                if !matching_base_right.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(cur_right.to_owned().into())),
                    })
                }
                cur_right_option = children_right_it.next();
            }
            (false, Some(_), None, None, None) => {
                result_children.push(cur_right.to_owned().into());
                cur_right_option = children_right_it.next();
            }
            (false, None, Some(matching_base_left), Some(_), Some(_)) => {
                if !matching_base_left.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: Some(Box::new(cur_left.to_owned().into())),
                        right: None,
                    });
                }

                cur_left_option = children_left_it.next();
            }
            (false, None, Some(matching_base_left), Some(_), None) => {
                if !matching_base_left.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: Some(Box::new(cur_left.to_owned().into())),
                        right: None,
                    })
                }
                cur_left_option = children_left_it.next();
            }
            (false, None, Some(matching_base_left), None, Some(matching_base_right)) => {
                match (
                    matching_base_left.is_perfect_match,
                    matching_base_right.is_perfect_match,
                ) {
                    (true, true) => {}
                    (true, false) => result_children.push(MergedCSTNode::Conflict {
                        left: Some(Box::new(cur_left.to_owned().into())),
                        right: None,
                    }),
                    (false, true) => result_children.push(MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(cur_right.to_owned().into())),
                    }),
                    (false, false) => result_children.push(MergedCSTNode::Conflict {
                        left: Some(Box::new(cur_left.to_owned().into())),
                        right: Some(Box::new(cur_right.to_owned().into())),
                    }),
                };

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (false, None, Some(matching_base_left), None, None) => {
                result_children.push(cur_right.to_owned().into());

                if !matching_base_left.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: Some(Box::new(cur_left.to_owned().into())),
                        right: None,
                    })
                }

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (false, None, None, Some(_), Some(_)) => {
                result_children.push(cur_left.to_owned().into());
                cur_left_option = children_left_it.next();
            }
            (false, None, None, Some(_), None) => {
                result_children.push(cur_left.to_owned().into());
                cur_left_option = children_left_it.next();
            }
            (false, None, None, None, Some(matching_base_right)) => {
                result_children.push(cur_left.to_owned().into());
                if !matching_base_right.is_perfect_match {
                    result_children.push(MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(cur_right.to_owned().into())),
                    })
                }

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (false, None, None, None, None) => {
                result_children.push(MergedCSTNode::Conflict {
                    left: Some(Box::new(cur_left.to_owned().into())),
                    right: Some(Box::new(cur_right.to_owned().into())),
                });

                cur_left_option = children_left_it.next();
                cur_right_option = children_right_it.next();
            }
            (a, b, c, d, e) => {
                return Err(MergeError::InvalidMatchingConfiguration(
                    a,
                    b.is_some(),
                    c.is_some(),
                    d.is_some(),
                    e.is_some(),
                ));
            }
        }
    }

    while let Some(cur_left) = cur_left_option {
        result_children.push(cur_left.to_owned().into());
        cur_left_option = children_left_it.next();
    }

    while let Some(cur_right) = cur_right_option {
        result_children.push(cur_right.to_owned().into());
        cur_right_option = children_right_it.next();
    }

    Ok(MergedCSTNode::NonTerminal {
        kind: left.kind,
        children: result_children,
    })
}

#[cfg(test)]
mod tests {
    use std::vec;

    use matching::{ordered, Matchings};
    use model::{cst_node::NonTerminal, cst_node::Terminal, CSTNode, Language, Point};

    use crate::{MergeError, MergedCSTNode};

    use super::ordered_merge;

    fn assert_merge_is_correct_and_idempotent_with_respect_to_parent_side<'a>(
        base: &'a CSTNode<'a>,
        parent_a: &'a CSTNode<'a>,
        parent_b: &'a CSTNode<'a>,
        expected_merge: &'a MergedCSTNode<'a>,
    ) -> Result<(), MergeError> {
        let matchings_base_parent_a = ordered::calculate_matchings(base, parent_a);
        let matchings_base_parent_b = ordered::calculate_matchings(base, parent_b);
        let matchings_parents = ordered::calculate_matchings(parent_a, parent_b);

        let merged_tree = ordered_merge(
            parent_a.try_into().unwrap(),
            parent_b.try_into().unwrap(),
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
        )?;
        let merged_tree_swap = ordered_merge(
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
        let matchings_base_parent_a = ordered::calculate_matchings(base, parent_a);
        let matchings_base_parent_b = ordered::calculate_matchings(base, parent_b);
        let matchings_parents = ordered::calculate_matchings(parent_a, parent_b);

        let merged_tree = ordered_merge(
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
    fn it_merges_non_terminals_if_there_are_non_changes() -> Result<(), MergeError> {
        let tree = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &tree,
            &tree,
            &tree,
            &tree.clone().into(),
        )
    }

    #[test]
    fn it_merges_non_terminals_if_both_left_and_right_add_the_same_things() -> Result<(), MergeError>
    {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            ..Default::default()
        });
        let parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &parent,
            &parent,
            &parent.clone().into(),
        )
    }

    #[test]
    fn it_merges_non_terminals_if_only_one_parent_adds_a_node_in_an_initially_empty_children_list(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            ..Default::default()
        });

        let initially_empty_parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            ..Default::default()
        });

        let parent_that_added = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",
            children: vec![MergedCSTNode::Terminal {
                kind: "kind_a",
                value: "value_a".to_string(),
            }],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &initially_empty_parent,
            &parent_that_added,
            &expected_merge,
        )
    }

    #[test]
    fn it_merges_non_terminals_if_only_one_parent_adds_a_node_in_non_empty_children_list(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let unchanged_parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let parent_that_added = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![
                MergedCSTNode::Terminal {
                    kind: "kind_a",
                    value: "value_a".to_string(),
                },
                MergedCSTNode::Terminal {
                    kind: "kind_b",
                    value: "value_b".to_string(),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &unchanged_parent,
            &parent_that_added,
            &merge,
        )
    }

    #[test]
    fn it_merges_when_one_parent_adds_a_node_and_removes_one_that_was_not_edited_in_the_other(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let changed_parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_b",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_b",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let unchanged_parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![MergedCSTNode::Terminal {
                kind: "kind_b",
                value: "value_b".to_string(),
            }],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &changed_parent,
            &unchanged_parent,
            &expected_merge,
        )
    }

    #[test]
    fn it_merges_when_one_parent_adds_a_node_and_removes_from_another_that_was_changed(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "subtree",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "another_subtree",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "subtree",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        let matchings_base_parent_a = ordered::calculate_matchings(&base, &parent_a);
        let matchings_base_parent_b = ordered::calculate_matchings(&base, &parent_b);
        let matchings_parents = ordered::calculate_matchings(&parent_a, &parent_b);

        let merged_tree = ordered_merge(
            (&parent_a).try_into().unwrap(),
            (&parent_b).try_into().unwrap(),
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
        )?;
        let merged_tree_swap = ordered_merge(
            (&parent_b).try_into().unwrap(),
            (&parent_a).try_into().unwrap(),
            &matchings_base_parent_b,
            &matchings_base_parent_a,
            &matchings_parents,
        )?;

        assert_eq!(
            MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![
                    MergedCSTNode::NonTerminal {
                        kind: "another_subtree",
                        children: vec![MergedCSTNode::Terminal {
                            kind: "kind_b",
                            value: "value_b".to_string(),
                        }],
                    },
                    MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "subtree",
                            children: vec![MergedCSTNode::Terminal {
                                kind: "kind_c",
                                value: "value_c".to_string(),
                            }],
                        })),
                    },
                ],
            },
            merged_tree
        );

        assert_eq!(
            MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![
                    MergedCSTNode::NonTerminal {
                        kind: "another_subtree",
                        children: vec![MergedCSTNode::Terminal {
                            kind: "kind_b",
                            value: "value_b".to_string(),
                        }],
                    },
                    MergedCSTNode::Conflict {
                        left: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "subtree",
                            children: vec![MergedCSTNode::Terminal {
                                kind: "kind_c",
                                value: "value_c".to_string(),
                            }],
                        })),
                        right: None,
                    },
                ],
            },
            merged_tree_swap
        );

        Ok(())
    }

    #[test]
    fn if_both_parents_add_different_nodes_then_we_have_a_conflict() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            ..Default::default()
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_b",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_b",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        assert_merge_output_is(
            &base,
            &left,
            &right,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![MergedCSTNode::Conflict {
                    left: Some(Box::new(MergedCSTNode::Terminal {
                        kind: "kind_a",
                        value: "value_a".to_string(),
                    })),
                    right: Some(Box::new(MergedCSTNode::Terminal {
                        kind: "kind_b",
                        value: "value_b".to_string(),
                    })),
                }],
            },
        )
    }

    #[test]
    fn it_merges_when_one_parent_removes_a_node_that_was_not_changed_in_another_parent(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_b",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_b",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![MergedCSTNode::Terminal {
                kind: "kind_b",
                value: "value_b".to_string(),
            }],
        };

        assert_merge_output_is(&base, &left, &right, &expected_merge)
    }

    #[test]
    fn it_detects_a_conflict_when_one_parent_removes_a_node_that_was_changed_in_another_parent(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "subtree",
                    are_children_unordered: false,
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    children: vec![CSTNode::Terminal(Terminal {
                        id: uuid::Uuid::new_v4(),
                        kind: "kind_b",
                        start_position: Point { row: 0, column: 0 },
                        end_position: Point { row: 0, column: 7 },
                        value: "value_b",
                        is_block_end_delimiter: false,
                    })],
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "subtree",
                    are_children_unordered: false,
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    children: vec![CSTNode::Terminal(Terminal {
                        id: uuid::Uuid::new_v4(),
                        kind: "kind_c",
                        start_position: Point { row: 0, column: 0 },
                        end_position: Point { row: 0, column: 7 },
                        value: "value_c",
                        is_block_end_delimiter: false,
                    })],
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        assert_merge_output_is(
            &base,
            &left,
            &right,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![
                    MergedCSTNode::Conflict {
                        left: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "subtree",
                            children: vec![MergedCSTNode::Terminal {
                                kind: "kind_c",
                                value: "value_c".to_string(),
                            }],
                        })),
                        right: None,
                    },
                    MergedCSTNode::Terminal {
                        kind: "kind_a",
                        value: "value_a".to_string(),
                    },
                ],
            },
        )?;

        assert_merge_output_is(
            &base,
            &right,
            &left,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![
                    MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "subtree",
                            children: vec![MergedCSTNode::Terminal {
                                kind: "kind_c",
                                value: "value_c".to_string(),
                            }],
                        })),
                    },
                    MergedCSTNode::Terminal {
                        kind: "kind_a",
                        value: "value_a".to_string(),
                    },
                ],
            },
        )
    }

    #[test]
    fn it_merges_when_a_parent_adds_a_node() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let unchanged_parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let changed_parent = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![
                MergedCSTNode::Terminal {
                    kind: "kind_a",
                    value: "value_a".to_string(),
                },
                MergedCSTNode::Terminal {
                    kind: "kind_b",
                    value: "value_b".to_string(),
                },
                MergedCSTNode::Terminal {
                    kind: "kind_c",
                    value: "value_c".to_string(),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &unchanged_parent,
            &changed_parent,
            &expected_merge,
        )
    }

    #[test]
    fn it_merges_when_one_parent_removes_and_add_a_node() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_b",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_b",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![MergedCSTNode::Terminal {
                kind: "kind_a",
                value: "value_a".to_string(),
            }],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &parent_a,
            &parent_b,
            &expected_merge,
        )
    }

    #[test]
    fn it_conflicts_when_one_parent_removes_and_add_a_node() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "subtree",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "subtree",
                    are_children_unordered: false,
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    children: vec![CSTNode::Terminal(Terminal {
                        id: uuid::Uuid::new_v4(),
                        kind: "kind_b",
                        start_position: Point { row: 0, column: 0 },
                        end_position: Point { row: 0, column: 7 },
                        value: "value_c",
                        is_block_end_delimiter: false,
                    })],
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        assert_merge_output_is(
            &base,
            &parent_a,
            &parent_b,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![
                    MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "subtree",
                            children: vec![MergedCSTNode::Terminal {
                                kind: "kind_b",
                                value: "value_c".to_string(),
                            }],
                        })),
                    },
                    MergedCSTNode::Terminal {
                        kind: "kind_a",
                        value: "value_a".to_string(),
                    },
                ],
            },
        )?;
        assert_merge_output_is(
            &base,
            &parent_b,
            &parent_a,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![
                    MergedCSTNode::Conflict {
                        left: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "subtree",
                            children: vec![MergedCSTNode::Terminal {
                                kind: "kind_b",
                                value: "value_c".to_string(),
                            }],
                        })),
                        right: None,
                    },
                    MergedCSTNode::Terminal {
                        kind: "kind_a",
                        value: "value_a".to_string(),
                    },
                ],
            },
        )
    }

    #[test]
    fn it_merges_when_a_parent_adds_one_node() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_a",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_a",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_a",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![
                MergedCSTNode::Terminal {
                    kind: "kind_c",
                    value: "value_c".to_string(),
                },
                MergedCSTNode::Terminal {
                    kind: "kind_a",
                    value: "value_a".to_string(),
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
    fn it_does_not_detect_a_conflict_if_am_merging_two_subtrees_that_have_not_changed_mutually(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_b",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_b",
                    is_block_end_delimiter: false,
                }),
                CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                }),
            ],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_b",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_b",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind_c",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value_c",
                is_block_end_delimiter: false,
            })],
            ..Default::default()
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "kind",

            children: vec![],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &parent_a,
            &parent_b,
            &expected_merge,
        )
    }

    #[test]
    fn it_detects_a_conflict_if_am_merging_two_subtrees_that_delete_a_node_that_was_changed_in_another_parent(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "subtree_a",
                    are_children_unordered: false,
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    children: vec![CSTNode::Terminal(Terminal {
                        id: uuid::Uuid::new_v4(),
                        kind: "kind_b",
                        start_position: Point { row: 0, column: 0 },
                        end_position: Point { row: 0, column: 7 },
                        value: "value_b",
                        is_block_end_delimiter: false,
                    })],
                    ..Default::default()
                }),
                CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "subtree_b",
                    are_children_unordered: false,
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    children: vec![CSTNode::Terminal(Terminal {
                        id: uuid::Uuid::new_v4(),
                        kind: "kind_c",
                        start_position: Point { row: 0, column: 0 },
                        end_position: Point { row: 0, column: 7 },
                        value: "value_c",
                        is_block_end_delimiter: false,
                    })],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "subtree_b",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_c",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            are_children_unordered: false,
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "subtree_a",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![CSTNode::Terminal(Terminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "kind_a",
                    start_position: Point { row: 0, column: 0 },
                    end_position: Point { row: 0, column: 7 },
                    value: "value_c",
                    is_block_end_delimiter: false,
                })],
                ..Default::default()
            })],
            ..Default::default()
        });

        assert_merge_output_is(
            &base,
            &parent_a,
            &parent_b,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![MergedCSTNode::Conflict {
                    left: Some(Box::new(MergedCSTNode::NonTerminal {
                        kind: "subtree_b",
                        children: vec![MergedCSTNode::Terminal {
                            kind: "kind_c",
                            value: "value_c".to_string(),
                        }],
                    })),
                    right: None,
                }],
            },
        )?;
        assert_merge_output_is(
            &base,
            &parent_b,
            &parent_a,
            &MergedCSTNode::NonTerminal {
                kind: "kind",
                children: vec![MergedCSTNode::Conflict {
                    left: None,
                    right: Some(Box::new(MergedCSTNode::NonTerminal {
                        kind: "subtree_b",
                        children: vec![MergedCSTNode::Terminal {
                            kind: "kind_c",
                            value: "value_c".to_string(),
                        }],
                    })),
                }],
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
        let result = ordered_merge(&kind_a, &kind_b, &matchings, &matchings, &matchings);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MergeError::NodesWithDifferentKinds("kind_a".to_string(), "kind_b".to_string())
        );
    }
}
