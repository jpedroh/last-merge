use std::cmp::max;

use pathfinding::{kuhn_munkres::Weights, matrix};

use crate::Matchings;

#[tracing::instrument(skip(left_children, right_children, matchings))]
pub fn calculate_matchings_for_children<'a>(
    left_children: &[&'a model::CSTNode<'a>],
    right_children: &[&'a model::CSTNode<'a>],
    matchings: &mut Matchings<'a>,
) -> usize {
    let children_matchings = left_children
        .iter()
        .map(|left_child| {
            right_children
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

    solve_assignment_problem(children_matchings, matchings)
}

fn solve_assignment_problem<'a>(
    mut children_matchings: Vec<Vec<(usize, Matchings<'a>)>>,
    matchings: &mut Matchings<'a>,
) -> usize {
    let m = children_matchings.len();
    if m == 0 {
        return 1;
    }

    let n = children_matchings[0].len();
    if n == 0 {
        return 1;
    }

    let max_size = max(m, n);

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; max_size]; max_size];
    for i in 0..m {
        for j in 0..n {
            matrix[i][j] = children_matchings[i][j].0.try_into().unwrap();
        }
    }

    let weights_matrix: pathfinding::prelude::Matrix<i32> = matrix::Matrix::from_rows(matrix)
        .expect("Could not build weights matrix for assignment problem.");
    let (max_matching, best_matches) = pathfinding::kuhn_munkres::kuhn_munkres(&weights_matrix);

    for i in 0..best_matches.len() {
        let j = best_matches[i];
        let cur_matching = weights_matrix.at(i, j);
        if cur_matching > 0 {
            matchings.extend(std::mem::take(&mut children_matchings[i][j].1));
        }
    }

    max_matching as usize
}
