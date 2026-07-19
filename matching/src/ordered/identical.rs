use model::CSTNode;

use crate::{matches::Matches, Matchings};

pub fn identical_matches<'a>(
    left_children: &'a [CSTNode<'a>],
    right_children: &'a [CSTNode<'a>],
    matchings: &mut Matchings<'a>,
) -> (usize, usize, usize) {
    let len = left_children.len().min(right_children.len());

    let mut score = 0;
    let mut prefix = 0;

    // Find common prefix
    for i in 0..len {
        if let Some(child_score) = identical_match(&left_children[i], &right_children[i], matchings)
        {
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
            &left_children[left_index],
            &right_children[right_index],
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

fn identical_match<'a>(
    left: &'a CSTNode<'a>,
    right: &'a CSTNode<'a>,
    matchings: &mut Matchings<'a>,
) -> Option<usize> {
    if !left.matches(right) {
        return None;
    }

    match (left, right) {
        (CSTNode::Terminal(_), CSTNode::Terminal(_)) => {
            let score = 1;
            matchings.push(left, right, score);
            Some(score)
        }

        (CSTNode::NonTerminal(nt_left), CSTNode::NonTerminal(nt_right)) => {
            let left_children = nt_left.get_children();
            let right_children = nt_right.get_children();

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
