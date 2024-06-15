use crate::{
    matching_configuration::MatchingConfiguration, matching_entry::MatchingEntry, Matchings,
};
use model::{cst_node::NonTerminal, CSTNode};
use unordered_pair::UnorderedPair;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Direction {
    Top,
    Left,
    Diag,
}

#[derive(Clone)]
struct Entry<'a>(pub Direction, pub Matchings<'a>);

impl<'a> Default for Entry<'a> {
    fn default() -> Self {
        Self(Direction::Top, Default::default())
    }
}

pub fn calculate_matchings<'a>(
    left: &'a CSTNode,
    right: &'a CSTNode,
    config: &'a MatchingConfiguration<'a>,
) -> Matchings<'a> {
    match (left, right) {
        (
            CSTNode::NonTerminal(NonTerminal {
                children: children_left,
                ..
            }),
            CSTNode::NonTerminal(NonTerminal {
                children: children_right,
                ..
            }),
        ) => {
            let root_matching: usize = config
                .handlers
                .compute_matching_score(left, right)
                .unwrap_or((left.kind() == right.kind()).into());

            let m = children_left.len();
            let n = children_right.len();

            let mut matrix_m = vec![vec![0; n + 1]; m + 1];
            let mut matrix_t = vec![vec![Entry::default(); n + 1]; m + 1];

            for i in 1..m + 1 {
                for j in 1..n + 1 {
                    let left_child = children_left.get(i - 1).unwrap();
                    let right_child = children_right.get(j - 1).unwrap();

                    let w = crate::calculate_matchings(left_child, right_child, config);
                    let matching = w
                        .get_matching_entry(left_child, right_child)
                        .unwrap_or_default();

                    if matrix_m[i][j - 1] > matrix_m[i - 1][j] {
                        if matrix_m[i][j - 1] > matrix_m[i - 1][j - 1] + matching.score {
                            matrix_m[i][j] = matrix_m[i][j - 1];
                            matrix_t[i][j] = Entry(Direction::Left, w);
                        } else {
                            matrix_m[i][j] = matrix_m[i - 1][j - 1] + matching.score;
                            matrix_t[i][j] = Entry(Direction::Diag, w);
                        }
                    } else if matrix_m[i - 1][j] > matrix_m[i - 1][j - 1] + matching.score {
                        matrix_m[i][j] = matrix_m[i - 1][j];
                        matrix_t[i][j] = Entry(Direction::Top, w);
                    } else {
                        matrix_m[i][j] = matrix_m[i - 1][j - 1] + matching.score;
                        matrix_t[i][j] = Entry(Direction::Diag, w);
                    }
                }
            }

            let mut i = m;
            let mut j = n;

            let mut matchings = Matchings::from_single(
                UnorderedPair(left, right),
                MatchingEntry::new(left, right, matrix_m[m][n] + root_matching),
            );

            while i >= 1 && j >= 1 {
                match matrix_t.get(i).unwrap().get(j).unwrap().0 {
                    Direction::Top => i -= 1,
                    Direction::Left => j -= 1,
                    Direction::Diag => {
                        if matrix_m[i][j] > matrix_m[i - 1][j - 1] {
                            matchings.extend(matrix_t[i][j].1.clone());
                        }
                        i -= 1;
                        j -= 1;
                    }
                }
            }

            matchings
        }
        (_, _) => Matchings::empty(),
    }
}

#[cfg(test)]
mod tests {
    use crate::MatchingConfiguration;
    use model::{
        cst_node::{NonTerminal, Terminal},
        language, CSTNode, Language, Point,
    };

    #[test]
    fn it_matches_deep_nodes_as_well() {
        let child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            value: "value_b",
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 1, column: 7 },
            is_block_end_delimiter: false,
        });
        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 1, column: 7 },
            children: vec![child.clone()],
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 1, column: 7 },
            children: vec![child.clone()],
        });

        let matching_configuration = MatchingConfiguration::default();
        let matchings = super::calculate_matchings(&left, &right, &matching_configuration);

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
            is_block_end_delimiter: false,
        });
        let right_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_c",
            value: "value_c",
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 1, column: 7 },
            is_block_end_delimiter: false,
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            children: vec![left_child.clone()],
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 0, column: 7 },
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            children: vec![right_child.clone()],
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 0, column: 7 },
        });

        let matching_configuration = MatchingConfiguration::from(Language::Java);
        let matchings = super::calculate_matchings(&left, &right, &matching_configuration);
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
            is_block_end_delimiter: false,
        });
        let unique_right_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_c",
            value: "value_c",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            is_block_end_delimiter: false,
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone()],
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone(), unique_right_child],
        });

        let matching_configuration = MatchingConfiguration::from(language::Language::Java);
        let matchings = super::calculate_matchings(&left, &right, &matching_configuration);

        let left_right_matchings = matchings.get_matching_entry(&left, &right).unwrap();
        assert_eq!(2, left_right_matchings.score);
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
            is_block_end_delimiter: false,
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone()],
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone()],
        });

        let matching_configuration = MatchingConfiguration::from(language::Language::Java);
        let matchings = super::calculate_matchings(&left, &right, &matching_configuration);

        let left_right_matchings = matchings.get_matching_entry(&left, &right).unwrap();
        assert_eq!(2, left_right_matchings.score);
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
            is_block_end_delimiter: false,
        });

        let intermediate = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "intermediate",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![leaf],
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![intermediate.clone()],
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![intermediate.clone()],
        });

        let matching_configuration = MatchingConfiguration::default();
        let matchings = super::calculate_matchings(&left, &right, &matching_configuration);

        let intermediate_matching = matchings
            .get_matching_entry(&intermediate, &intermediate)
            .unwrap();
        assert_eq!(2, intermediate_matching.score);
        assert!(intermediate_matching.is_perfect_match);

        let left_right_matching = matchings.get_matching_entry(&left, &right).unwrap();
        assert_eq!(3, left_right_matching.score);
        assert!(left_right_matching.is_perfect_match);
    }
}
