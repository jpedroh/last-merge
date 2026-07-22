use model::CSTNode;

use crate::{can_match::CanMatch, Matchings};

#[tracing::instrument(skip(left_children, right_children, matchings))]
pub fn identical_matches<'tree>(
    left_children: &[&'tree CSTNode<'tree>],
    right_children: &[&'tree CSTNode<'tree>],
    matchings: &mut Matchings<'tree>,
) -> (usize, usize, usize) {
    let len = left_children.len().min(right_children.len());

    let mut score = 0;
    let mut prefix = 0;

    // Find common prefix
    for i in 0..len {
        if let Some(child_score) = identical_match(left_children[i], right_children[i], matchings) {
            score += child_score;
            prefix += 1;
        } else {
            break;
        }
    }

    let mut suffix = 0;

    // Find common suffix
    while suffix < len - prefix {
        let left_index = left_children.len() - suffix - 1;
        let right_index = right_children.len() - suffix - 1;

        if let Some(child_score) = identical_match(
            left_children[left_index],
            right_children[right_index],
            matchings,
        ) {
            score += child_score;
            suffix += 1;
        } else {
            break;
        }
    }

    (prefix, suffix, score)
}

fn identical_match<'tree>(
    left: &'tree CSTNode<'tree>,
    right: &'tree CSTNode<'tree>,
    matchings: &mut Matchings<'tree>,
) -> Option<usize> {
    if !left.can_match(right) {
        return None;
    }

    match (left, right) {
        (CSTNode::Terminal(_), CSTNode::Terminal(_)) => {
            let score = 1;
            matchings.push(left, right, score);
            Some(score)
        }

        (CSTNode::NonTerminal(nt_left), CSTNode::NonTerminal(nt_right)) => {
            let left_children: Vec<_> = nt_left.children_without_delimiters().collect();
            let right_children: Vec<_> = nt_right.children_without_delimiters().collect();

            if left_children.len() != right_children.len() {
                return None;
            }

            let mut score = 1;

            for (l, r) in left_children.iter().zip(right_children.iter()) {
                score += identical_match(l, r, matchings)?;
            }

            matchings.push(left, right, score);
            Some(score)
        }

        _ => None,
    }
}
