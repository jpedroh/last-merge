use std::cmp::max;

use model::{cst_node::NonTerminal, CSTNode};
use pathfinding::{kuhn_munkres::Weights, matrix};
use unordered_pair::UnorderedPair;

use crate::{MatchingEntry, Matchings};

pub fn calculate_matchings<'a>(
    left: &'a CSTNode<'a>,
    right: &'a CSTNode<'a>,
) -> crate::Matchings<'a> {
    match (left, right) {
        (
            CSTNode::NonTerminal(NonTerminal {
                kind: kind_left,
                children: children_left,
                ..
            }),
            CSTNode::NonTerminal(NonTerminal {
                kind: kind_right,
                children: children_right,
                ..
            }),
        ) => {
            if kind_left != kind_right {
                return Matchings::empty();
            }

            let children_matchings = children_left
                .iter()
                .map(|left_child| {
                    children_right
                        .iter()
                        .map(|right_child| {
                            let w = crate::calculate_matchings(left_child, right_child);
                            let matching = w
                                .get_matching_entry(left_child, right_child)
                                .unwrap_or_default();
                            (matching.score, w)
                        })
                        .collect()
                })
                .collect();

            solve_assignment_problem(left, right, children_matchings)
        }
        (_, _) => unreachable!(
            "Unordered matching must never be called if the nodes are not NonTerminals."
        ),
    }
}

fn solve_assignment_problem<'a>(
    left: &'a CSTNode,
    right: &'a CSTNode,
    children_matchings: Vec<Vec<(usize, Matchings<'a>)>>,
) -> Matchings<'a> {
    let m = children_matchings.len();
    let n = children_matchings[0].len();
    let max_size = max(m, n);

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; max_size]; max_size];
    for i in 0..m {
        for j in 0..n {
            matrix[i][j] = children_matchings[i][j].0.try_into().unwrap();
        }
    }

    let weights_matrix = matrix::Matrix::from_rows(matrix)
        .expect("Could not build weights matrix for assignment problem.");
    let (max_matching, best_matches) = pathfinding::kuhn_munkres::kuhn_munkres(&weights_matrix);

    let mut result = Matchings::empty();

    for i in 0..best_matches.len() {
        let j = best_matches[i];
        let cur_matching = weights_matrix.at(i, j);
        if cur_matching > 0 {
            result.extend(children_matchings[i][j].1.clone());
        }
    }

    result.extend(Matchings::from_single(
        UnorderedPair(left, right),
        MatchingEntry::new(left, right, max_matching as usize + 1),
    ));

    result
}
