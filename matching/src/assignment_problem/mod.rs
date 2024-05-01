use std::cmp::max;

use matching_handlers::MatchingHandlers;
use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode,
};
use pathfinding::{kuhn_munkres::Weights, matrix};
use unordered_pair::UnorderedPair;

use crate::{calculate_matchings, MatchingEntry, Matchings};

pub fn unordered_tree_matching<'a>(
    left: &'a CSTNode,
    right: &'a CSTNode,
    matching_handlers: &'a MatchingHandlers<'a>,
) -> crate::Matchings<'a> {
    match (left, right) {
        (
            CSTNode::Terminal(Terminal {
                kind: kind_left,
                value: value_left,
                ..
            }),
            CSTNode::Terminal(Terminal {
                kind: kind_right,
                value: value_right,
                ..
            }),
        ) => {
            let is_perfetch_match = kind_left == kind_right && value_left == value_right;
            Matchings::from_single(
                UnorderedPair(left, right),
                MatchingEntry::new(is_perfetch_match.into(), is_perfetch_match),
            )
        }
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
            let root_matching: usize = (kind_left == kind_right).into();

            let matchings: Vec<Vec<(usize, Matchings<'a>)>> = children_left
                .iter()
                .map(|left_child| {
                    children_right
                        .iter()
                        .map(|right_child| {
                            let w = calculate_matchings(left_child, right_child, matching_handlers);
                            let matching = w
                                .get_matching_entry(left_child, right_child)
                                .unwrap_or_default();
                            (matching.score, w)
                        })
                        .collect()
                })
                .collect();

            return solve_assignment_problem(left, right, matchings, root_matching);
        }
        (_, _) => unreachable!("Invalid configuration reached"),
    }
}

fn solve_assignment_problem<'a>(
    left: &'a CSTNode,
    right: &'a CSTNode,
    children_matchings: Vec<Vec<(usize, Matchings<'a>)>>,
    root_matching: usize,
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
        MatchingEntry {
            score: max_matching as usize + root_matching,
            is_perfect_match: left.contents() == right.contents(),
        },
    ));

    result
}