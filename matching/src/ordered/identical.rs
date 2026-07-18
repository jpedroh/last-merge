use model::CSTNode;

use crate::{matches::Matches, Matchings};

pub fn identical_matches<'a>(
    left_children: &'a [CSTNode<'a>],
    right_children: &'a [CSTNode<'a>],
    matchings: &mut Matchings<'a>,
) -> (usize, usize) {
    let len = left_children.len().min(right_children.len());

    let mut score = 0;

    for i in 0..len {
        if let Some(child_score) = identical_match(&left_children[i], &right_children[i], matchings)
        {
            log::debug!(
                "Marking {:?} and {:?} as identical",
                &left_children[i].contents(),
                &right_children[i].contents()
            );
            score += child_score;
        } else {
            return (i, score);
        }
    }

    (len, score)
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
