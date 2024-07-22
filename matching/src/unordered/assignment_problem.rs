use std::cmp::max;

use pathfinding::{kuhn_munkres::Weights, matrix};
use unordered_pair::UnorderedPair;

use crate::{matches::Matches, MatchingEntry, Matchings};

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode<'a>,
    right: &'a model::CSTNode<'a>,
) -> Matchings<'a> {
    match (left, right) {
        (model::CSTNode::NonTerminal(nt_left), model::CSTNode::NonTerminal(nt_right)) => {
            if !left.matches(right) {
                return Matchings::empty();
            }

            let children_matchings = nt_left
                .get_children()
                .iter()
                .map(|left_child| {
                    nt_right
                        .get_children()
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
    left: &'a model::CSTNode,
    right: &'a model::CSTNode,
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
