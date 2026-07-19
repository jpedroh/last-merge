mod can_match;
mod matching;
mod matching_entry;
mod matchings;
mod ordered;
mod unordered;

use can_match::CanMatch;
pub use matching_entry::MatchingEntry;
pub use matchings::Matchings;

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode,
    right: &'a model::CSTNode,
) -> Matchings<'a> {
    let largest_tree = left.get_tree_size().max(right.get_tree_size());
    let mut matchings = Matchings::with_capacity(largest_tree);
    if left.can_match(right) {
        if let Some(matching_score) = calculate_matchings_internal(left, right, &mut matchings) {
            matchings.push(left, right, matching_score);
        }
    }
    matchings
}

fn calculate_matchings_internal<'a>(
    left: &'a model::CSTNode<'a>,
    right: &'a model::CSTNode<'a>,
    matchings: &mut Matchings<'a>,
) -> Option<usize> {
    match (left, right) {
        (model::CSTNode::NonTerminal(nt_left), model::CSTNode::NonTerminal(nt_right)) => {
            if nt_left.are_children_unordered && nt_right.are_children_unordered {
                Some(1 + unordered::calculate_matchings(nt_left, nt_right, matchings))
            } else {
                Some(1 + ordered::calculate_matchings(nt_left, nt_right, matchings))
            }
        }
        (model::CSTNode::Terminal(_), model::CSTNode::Terminal(_)) => Some(1),
        (_, _) => None,
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
            leading_white_space: None,
        });
        let right = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
            leading_white_space: None,
        });

        let matchings = calculate_matchings(&left, &right);

        let left_right_matching = matchings.get_matching_entry(&left, &right).unwrap();
        assert_eq!(1, left_right_matching.score);
        assert!(left_right_matching.is_perfect_match);
    }
}
