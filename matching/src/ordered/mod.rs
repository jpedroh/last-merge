mod identical;
mod yang;

use crate::{matches::Matches, ordered::identical::identical_matches, Matchings};

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode<'a>,
    right: &'a model::CSTNode<'a>,
    matchings: &mut Matchings<'a>,
) -> usize {
    if let (model::CSTNode::NonTerminal(nt_left), model::CSTNode::NonTerminal(nt_right)) =
        (left, right)
    {
        let root_matching: usize = (left.matches(right)).into();

        let (prefix, suffix, identical_children_score) =
            identical_matches(nt_left.get_children(), nt_right.get_children(), matchings);

        let left_children = nt_left.get_children();
        let right_children = nt_right.get_children();

        let remaining_children_left = left_children.len() - prefix - suffix;
        let remaining_children_right = right_children.len() - prefix - suffix;

        if remaining_children_left == 0 && remaining_children_right == 0 {
            log::debug!("Identical suffix/prefix fully reduced search space");
            matchings.push(left, right, identical_children_score + root_matching);
            identical_children_score + root_matching
        } else {
            log::debug!(
                "Identical suffix/prefix reduced search space from {:?}x{:?} to {:?}x{:?}",
                left_children.len(),
                right_children.len(),
                remaining_children_left,
                remaining_children_right,
            );

            let maximum_children_score = yang::yang(
                left_children[prefix..left_children.len() - suffix].as_ref(),
                right_children[prefix..right_children.len() - suffix].as_ref(),
                matchings,
            );
            matchings.push(
                left,
                right,
                identical_children_score + maximum_children_score + root_matching,
            );
            identical_children_score + maximum_children_score + root_matching
        }
    } else {
        0
    }
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
        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 1, column: 7 },
            children: vec![child.clone()],
            ..Default::default()
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 1, column: 7 },
            children: vec![child.clone()],
            ..Default::default()
        });

        let mut matchings = Matchings::empty();
        super::calculate_matchings(&left, &right, &mut matchings);

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

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            children: vec![left_child.clone()],
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 0, column: 7 },
            ..Default::default()
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            children: vec![right_child.clone()],
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 0, column: 7 },
            ..Default::default()
        });

        let mut matchings = Matchings::empty();
        super::calculate_matchings(&left, &right, &mut matchings);
        assert!(matchings
            .get_matching_entry(&left_child, &right_child)
            .is_none())
    }

    #[test]
    fn the_matching_between_two_subtrees_is_the_sum_of_the_matchings_plus_the_root() {
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

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone()],
            ..Default::default()
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone(), unique_right_child],
            ..Default::default()
        });

        let mut matchings = Matchings::empty();
        let score = super::calculate_matchings(&left, &right, &mut matchings);

        let left_right_matchings = matchings.get_matching_entry(&left, &right).unwrap();
        assert_eq!(2, score);
        assert_eq!(score, left_right_matchings.score);
        assert!(!left_right_matchings.is_perfect_match);
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

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone()],
            ..Default::default()
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone()],
            ..Default::default()
        });

        let mut matchings = Matchings::empty();
        let score = super::calculate_matchings(&left, &right, &mut matchings);

        let left_right_matchings = matchings.get_matching_entry(&left, &right).unwrap();
        assert_eq!(2, score);
        assert_eq!(score, left_right_matchings.score);
        assert!(left_right_matchings.is_perfect_match);
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

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![intermediate.clone()],
            ..Default::default()
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![intermediate.clone()],
            ..Default::default()
        });

        let mut matchings = Matchings::empty();
        let score = super::calculate_matchings(&left, &right, &mut matchings);

        let intermediate_matching = matchings
            .get_matching_entry(&intermediate, &intermediate)
            .unwrap();
        assert_eq!(2, intermediate_matching.score);
        assert!(intermediate_matching.is_perfect_match);

        let left_right_matching = matchings.get_matching_entry(&left, &right).unwrap();
        assert_eq!(3, score);
        assert_eq!(score, left_right_matching.score);
        assert!(left_right_matching.is_perfect_match);
    }
}
