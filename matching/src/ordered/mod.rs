mod identical;
mod yang;

use model::{cst_node::NonTerminal, CSTNode};
use tracing::Span;

use crate::Matchings;

#[tracing::instrument(
    name = "calculate_ordered_subtrees_matching",
    skip(matchings),
    fields(
        left_children_len=left.get_children().len(),
        right_children_len=right.get_children().len(),
        prefix,
        suffix
    )
)]
pub fn calculate_subtree_matching<'a>(
    left: &'a NonTerminal<'a>,
    right: &'a NonTerminal<'a>,
    matchings: &mut Matchings<'a>,
) -> usize {
    let left_children: Vec<_> = left.children_without_delimiters().collect();
    let right_children: Vec<_> = right.children_without_delimiters().collect();

    let (prefix, suffix, identical_children_score) = identical::identical_matches(
        left_children.as_slice(),
        right_children.as_slice(),
        matchings,
    );
    Span::current().record("prefix", prefix);
    Span::current().record("suffix", suffix);

    debug_assert!(prefix + suffix <= left_children.len());
    debug_assert!(prefix + suffix <= right_children.len());

    let remaining_children_left = left_children[prefix..left_children.len() - suffix].as_ref();
    let remaining_children_right: &[&CSTNode<'_>] =
        right_children[prefix..right_children.len() - suffix].as_ref();

    if remaining_children_left.is_empty() || remaining_children_right.is_empty() {
        tracing::debug!("Identical suffix/prefix fully reduced search space");
        identical_children_score
    } else {
        tracing::debug!(
            "Identical suffix/prefix reduced search space from {:?}x{:?} to {:?}x{:?}",
            left_children.len(),
            right_children.len(),
            remaining_children_left.len(),
            remaining_children_right.len(),
        );
        identical_children_score
            + calculate_remaining_children_matching(
                remaining_children_left,
                remaining_children_right,
                matchings,
            )
    }
}

fn calculate_remaining_children_matching<'tree>(
    left: &[&'tree CSTNode<'tree>],
    right: &[&'tree CSTNode<'tree>],
    matchings: &mut Matchings<'tree>,
) -> usize {
    match (left.len(), right.len()) {
        (0, _) | (_, 0) => 0,
        (1, _) => match_single_child(left[0], right, matchings),
        (_, 1) => match_single_child(right[0], left, matchings),
        _ => yang::yang(left, right, matchings),
    }
}

fn match_single_child<'tree>(
    single_child: &'tree CSTNode<'tree>,
    other_children: &[&'tree CSTNode<'tree>],
    matchings: &mut Matchings<'tree>,
) -> usize {
    let mut best_score = 0;
    let mut best_matchings = None;

    for other_child in other_children {
        let candidate = crate::calculate_matchings(single_child, other_child);

        let score: usize = candidate
            .get_matching_entry(single_child, other_child)
            .map(|m| m.score)
            .unwrap_or(0);

        if score > best_score {
            best_score = score;
            best_matchings = Some(candidate);
        }
    }

    if let Some(best_matchings) = best_matchings {
        matchings.extend(best_matchings);
    }

    best_score
}

#[cfg(test)]
mod tests {
    use model::{
        cst_node::{NonTerminal, Terminal},
        CSTNode, Point,
    };

    use crate::Matchings;

    #[test]
    fn it_matches_deep_nodes_as_well() {
        let child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            value: "value_b",
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 1, column: 7 },
            leading_white_space: None,
        });
        let left = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 1, column: 7 },
            children: vec![child.clone()],
            ..Default::default()
        };
        let right = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 1, column: 7 },
            children: vec![child.clone()],
            ..Default::default()
        };

        let mut matchings = Matchings::empty();
        super::calculate_subtree_matching(&left, &right, &mut matchings);

        let child_matching = matchings.get_matching_entry(&child, &child);
        assert!(child_matching.is_some());
        assert_eq!(1, child_matching.unwrap().score);
        assert!(child_matching.unwrap().is_perfect_match)
    }

    #[test]
    fn if_no_match_is_found_it_returns_none() {
        let left_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            value: "value_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            leading_white_space: None,
        });
        let right_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_c",
            value: "value_c",
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 1, column: 7 },
            leading_white_space: None,
        });

        let left = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            children: vec![left_child.clone()],
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 0, column: 7 },
            ..Default::default()
        };
        let right = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            children: vec![right_child.clone()],
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 0, column: 7 },
            ..Default::default()
        };

        let mut matchings = Matchings::empty();
        super::calculate_subtree_matching(&left, &right, &mut matchings);
        assert!(matchings
            .get_matching_entry(&left_child, &right_child)
            .is_none())
    }

    #[test]
    fn the_matching_between_two_subtrees_is_the_sum_of_the_matchings() {
        let common_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            value: "value_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            leading_white_space: None,
        });
        let unique_right_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_c",
            value: "value_c",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            leading_white_space: None,
        });

        let left = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone()],
            ..Default::default()
        };
        let right = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone(), unique_right_child],
            ..Default::default()
        };

        let mut matchings = Matchings::empty();
        let score = super::calculate_subtree_matching(&left, &right, &mut matchings);
        assert_eq!(1, score);
    }

    #[test]
    fn perfect_matching_deep_nodes() {
        let common_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value_b",
            leading_white_space: None,
        });

        let left = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone()],
            ..Default::default()
        };
        let right = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone()],
            ..Default::default()
        };

        let mut matchings = Matchings::empty();
        let score = super::calculate_subtree_matching(&left, &right, &mut matchings);
        assert_eq!(1, score);
    }

    #[test]
    fn perfect_matching_deeper_nodes() {
        let leaf = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value_b",
            ..Default::default()
        });

        let intermediate = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "intermediate",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![leaf],
            ..Default::default()
        });

        let left = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![intermediate.clone()],
            ..Default::default()
        };
        let right = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![intermediate.clone()],
            ..Default::default()
        };

        let mut matchings = Matchings::empty();
        let score = super::calculate_subtree_matching(&left, &right, &mut matchings);
        assert_eq!(2, score);

        let intermediate_matching = matchings
            .get_matching_entry(&intermediate, &intermediate)
            .unwrap();
        assert_eq!(2, intermediate_matching.score);
        assert!(intermediate_matching.is_perfect_match);
    }

    #[test]
    fn it_matches_when_one_side_is_fully_consumed_by_prefix_reduction() {
        fn terminal(kind: &'static str) -> CSTNode<'static> {
            CSTNode::Terminal(Terminal {
                id: uuid::Uuid::new_v4(),
                kind,
                value: kind,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 0 },
                leading_white_space: None,
            })
        }

        for left_has_extra_children in [true, false] {
            let common = terminal("common");

            let mut left_children = vec![common.clone()];
            let mut right_children = vec![common.clone()];

            if left_has_extra_children {
                left_children.push(terminal("left_only_1"));
                left_children.push(terminal("left_only_2"));
            } else {
                right_children.push(terminal("right_only_1"));
                right_children.push(terminal("right_only_2"));
            }

            let left = NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "parent",
                are_children_unordered: false,
                children: left_children,
                ..Default::default()
            };

            let right = NonTerminal {
                id: uuid::Uuid::new_v4(),
                kind: "parent",
                are_children_unordered: false,
                children: right_children,
                ..Default::default()
            };

            let mut matchings = Matchings::empty();

            let score = super::calculate_subtree_matching(&left, &right, &mut matchings);

            assert_eq!(1, score);
            assert!(
                matchings
                    .get_matching_entry(&common, &common)
                    .unwrap()
                    .is_perfect_match
            );
        }
    }
}
