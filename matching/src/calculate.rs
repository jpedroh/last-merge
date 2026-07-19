use crate::{can_match::CanMatch, ordered, unordered, Matchings};

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode,
    right: &'a model::CSTNode,
) -> Matchings<'a> {
    let mut matchings = Matchings::with_capacity(left.get_tree_size().max(right.get_tree_size()));
    if !left.can_match(right) {
        return matchings;
    }

    let subtrees_matching = calculate_subtree_matching(left, right, &mut matchings);
    let root_matching = 1;
    matchings.push(left, right, root_matching + subtrees_matching);

    matchings
}

fn calculate_subtree_matching<'a>(
    left: &'a model::CSTNode<'a>,
    right: &'a model::CSTNode<'a>,
    matchings: &mut Matchings<'a>,
) -> usize {
    match (left, right) {
        (model::CSTNode::NonTerminal(nt_left), model::CSTNode::NonTerminal(nt_right)) => {
            if nt_left.are_children_unordered && nt_right.are_children_unordered {
                unordered::calculate_subtree_matching(nt_left, nt_right, matchings)
            } else {
                ordered::calculate_subtree_matching(nt_left, nt_right, matchings)
            }
        }
        (model::CSTNode::Terminal(_), model::CSTNode::Terminal(_)) => 0,
        _ => unreachable!("can_match guarantees both nodes have the same variant"),
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
