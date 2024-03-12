use crate::merge_error::MergeError;
use crate::merge_terminals::merge_terminals;
use crate::ordered_merge::ordered_merge;
use crate::unordered_merge::unordered_merge;
use matching::Matchings;
use model::CSTNode;

use crate::merged_cst_node::MergedCSTNode;

pub fn merge<'a>(
    base: &'a CSTNode<'a>,
    left: &'a CSTNode<'a>,
    right: &'a CSTNode<'a>,
    base_left_matchings: &'a Matchings<'a>,
    base_right_matchings: &'a Matchings<'a>,
    left_right_matchings: &'a Matchings<'a>,
) -> Result<MergedCSTNode<'a>, MergeError> {
    match (base, left, right) {
        (CSTNode::Terminal(a_base), CSTNode::Terminal(a_left), CSTNode::Terminal(a_right)) => {
            merge_terminals(a_base, a_left, a_right)
        }
        (CSTNode::NonTerminal(_), CSTNode::NonTerminal(a_left), CSTNode::NonTerminal(a_right)) => {
            if a_left.are_children_unordered && a_right.are_children_unordered {
                Ok(unordered_merge(
                    a_left,
                    a_right,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                )?)
            } else {
                Ok(ordered_merge(
                    a_left,
                    a_right,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                )?)
            }
        }
        (_, _, _) => Err(MergeError::MergingTerminalWithNonTerminal),
    }
}

#[cfg(test)]
mod tests {
    use super::merge;
    use crate::MergeError;
    use matching::Matchings;
    use model::{
        cst_node::{NonTerminal, Terminal},
        CSTNode, Point,
    };
    use std::vec;

    #[test]
    fn test_can_not_merge_terminal_with_non_terminal() -> Result<(), Box<dyn std::error::Error>> {
        let error = merge(
            &CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value",
                is_block_end_delimiter: false,
            }),
            &CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value",
                is_block_end_delimiter: false,
            }),
            &CSTNode::NonTerminal(NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "kind",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![],
            }),
            &Matchings::empty(),
            &Matchings::empty(),
            &Matchings::empty(),
        )
        .unwrap_err();

        assert_eq!(error, MergeError::MergingTerminalWithNonTerminal);

        Ok(())
    }
}
