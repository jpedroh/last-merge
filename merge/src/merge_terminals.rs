use model::cst_node::Terminal;

use crate::{MergeError, MergedCSTNode};

pub fn merge_terminals<'a>(
    base: &'a Terminal<'a>,
    left: &'a Terminal<'a>,
    right: &'a Terminal<'a>,
) -> Result<MergedCSTNode<'a>, MergeError> {
    log::trace!("Calling merge terminal");

    // Nodes of different kind, early return
    if left.kind != right.kind {
        return Err(MergeError::NodesWithDifferentKinds(
            left.kind.to_string(),
            right.kind.to_string(),
        ));
    }

    let left_equals_base = left.value == base.value;
    let right_equals_base = right.value == base.value;

    // Unchanged
    if left_equals_base && right_equals_base {
        log::trace!("Unchanged");
        Ok(base.into())
    // Changed in both
    } else if !left_equals_base && !right_equals_base {
        log::trace!("Changed in both");
        match diffy::merge(base.value, left.value, right.value) {
            Ok(value) => Ok(MergedCSTNode::Terminal {
                kind: base.kind,
                value: std::borrow::Cow::Owned(value),
            }),
            Err(value) => Ok(MergedCSTNode::Terminal {
                kind: base.kind,
                value: std::borrow::Cow::Owned(value),
            }),
        }
    // Only left changed
    } else if right_equals_base {
        log::trace!("Only left changed");
        Ok(left.into())
    // Only right changed
    } else {
        log::trace!("Only right changed");
        Ok(right.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MergedCSTNode;
    use model::{cst_node::Terminal, Point};

    fn assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
        base: &Terminal,
        parent_a: &Terminal,
        parent_b: &Terminal,
        expected_merge: &MergedCSTNode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let merged_tree = merge_terminals(base, parent_a, parent_b)?;
        let merged_tree_swap = merge_terminals(base, parent_b, parent_a)?;

        assert_eq!(expected_merge, &merged_tree);
        assert_eq!(expected_merge, &merged_tree_swap);
        Ok(())
    }

    #[test]
    fn if_i_am_merging_three_unchanged_nodes_it_is_a_success(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let node = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value",
            is_block_end_delimiter: false,
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &node,
            &node,
            &node,
            &(&node).into(),
        )
    }

    #[test]
    fn returns_success_if_there_are_changes_in_both_parents_and_they_are_not_conflicting(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let base = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "\nvalue\n",
            is_block_end_delimiter: false,
        };
        let left = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "left\nvalue\n",
            is_block_end_delimiter: false,
        };
        let right = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "\nvalue\nright",
            is_block_end_delimiter: false,
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &left,
            &right,
            &MergedCSTNode::Terminal {
                kind: "kind",
                value: std::borrow::Cow::Borrowed("left\nvalue\nright"),
            },
        )
    }

    #[test]
    fn returns_conflict_if_there_are_changes_in_both_parents_and_they_are_conflicting(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let base = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value",
            is_block_end_delimiter: false,
        };
        let left = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "left_value",
            is_block_end_delimiter: false,
        };
        let right = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "right_value",
            is_block_end_delimiter: false,
        };

        assert_eq!(
            merge_terminals(&base, &left, &right)?,
           MergedCSTNode::Terminal {
                kind: "kind",
                value: std::borrow::Cow::Borrowed("<<<<<<< ours\nleft_value||||||| original\nvalue=======\nright_value>>>>>>> theirs\n")
            }
        );

        Ok(())
    }

    #[test]
    fn if_there_is_a_change_only_in_one_parent_it_returns_the_changes_from_this_parent(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let base_and_left = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value",
            is_block_end_delimiter: false,
        };
        let changed_parent = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value_right",
            is_block_end_delimiter: false,
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base_and_left,
            &base_and_left,
            &changed_parent,
            &(&changed_parent).into(),
        )
    }

    #[test]
    fn i_get_an_error_if_i_try_to_merge_nodes_of_different_kinds() {
        let kind_a = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value",
            is_block_end_delimiter: false,
        };
        let kind_b = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value_right",
            is_block_end_delimiter: false,
        };

        let result = merge_terminals(&kind_a, &kind_a, &kind_b);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MergeError::NodesWithDifferentKinds("kind_a".to_string(), "kind_b".to_string())
        );
    }
}
