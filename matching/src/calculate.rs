use model::{cst_node::NonTerminal, CSTNode};
use tracing::Span;

use crate::{can_match::CanMatch, ordered, unordered, MatchingEntry, Matchings};

#[tracing::instrument(fields(score = tracing::field::Empty, is_perfect_match = tracing::field::Empty))]
pub fn calculate_matchings<'a>(left: &'a CSTNode, right: &'a CSTNode) -> Matchings<'a> {
    let mut matchings = Matchings::with_capacity(left.get_tree_size().max(right.get_tree_size()));
    if !left.can_match(right) {
        return matchings;
    }

    let subtrees_matching_score: usize =
        if let (CSTNode::NonTerminal(left), CSTNode::NonTerminal(right)) = (left, right) {
            calculate_subtree_matching(left, right, &mut matchings)
        } else {
            0
        };

    let matching_entry = MatchingEntry::new(left, right, 1 + subtrees_matching_score);
    Span::current().record("score", matching_entry.score);
    Span::current().record("is_perfect_match", matching_entry.is_perfect_match);

    matchings.insert_entry(left, right, matching_entry);

    matchings
}

#[tracing::instrument(skip(matchings), fields(left_children_len = tracing::field::Empty, right_children_len = tracing::field::Empty, left_children_ordered = tracing::field::Empty, right_children_ordered = tracing::field::Empty))]
fn calculate_subtree_matching<'a>(
    left: &'a NonTerminal<'a>,
    right: &'a NonTerminal<'a>,
    matchings: &mut Matchings<'a>,
) -> usize {
    Span::current().record("left_children_len", left.get_children().len());
    Span::current().record("right_children_len", right.get_children().len());

    Span::current().record("left_children_ordered", left.are_children_unordered);
    Span::current().record("right_children_ordered", right.are_children_unordered);

    if left.are_children_unordered && right.are_children_unordered {
        unordered::calculate_subtree_matching(left, right, matchings)
    } else {
        ordered::calculate_subtree_matching(left, right, matchings)
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
