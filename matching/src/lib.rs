mod matches;
mod matching;
mod matching_entry;
mod matchings;
pub mod ordered;
pub mod unordered;

use matches::Matches;
pub use matching_entry::MatchingEntry;
pub use matchings::Matchings;
use unordered_pair::UnorderedPair;

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode,
    right: &'a model::CSTNode,
) -> Matchings<'a> {
    if !left.matches(right) {
        return Matchings::empty();
    }

    match (left, right) {
        (
            model::CSTNode::NonTerminal(non_terminal_left),
            model::CSTNode::NonTerminal(non_terminal_right),
        ) => {
            if non_terminal_left.are_children_unordered && non_terminal_right.are_children_unordered
            {
                unordered::calculate_matchings(left, right)
            } else {
                ordered::calculate_matchings(left, right)
            }
        }
        (
            model::CSTNode::Terminal(model::cst_node::Terminal {
                kind: kind_left,
                value: value_left,
                ..
            }),
            model::CSTNode::Terminal(model::cst_node::Terminal {
                kind: kind_right,
                value: value_right,
                ..
            }),
        ) => {
            let is_perfect_match = kind_left == kind_right && value_left == value_right;
            Matchings::from_single(
                UnorderedPair(left, right),
                MatchingEntry::new(left, right, is_perfect_match.into()),
            )
        }
        (_, _) => Matchings::empty(),
    }
}

#[cfg(test)]
mod tests {
    use crate::calculate_matchings;
    use model::{cst_node::Terminal, CSTNode, Point};

    #[test]
    fn two_terminal_nodes_matches_with_a_score_of_one_if_they_have_the_same_kind_and_value() {
        let left = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
            is_block_end_delimiter: false,
        });
        let right = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
            is_block_end_delimiter: false,
        });

        let matchings = calculate_matchings(&left, &right);

        let left_right_matching = matchings.get_matching_entry(&left, &right).unwrap();
        assert_eq!(1, left_right_matching.score);
        assert!(left_right_matching.is_perfect_match);
    }
}
