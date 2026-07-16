use std::collections::{HashMap, HashSet};

use model::cst_node::{CSTNode, Delimiters, NonTerminal};

mod assignment_problem;
mod unique_label;

use crate::matches::Matches;
use crate::{MatchingEntry, Matchings};
use unordered_pair::UnorderedPair;

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode<'a>,
    right: &'a model::CSTNode<'a>,
) -> crate::Matchings<'a> {
    match (left, right) {
        (model::CSTNode::NonTerminal(left_nt), model::CSTNode::NonTerminal(right_nt)) => {
            let root_matching: usize = left.matches(right).into();

            log::debug!(
                "Starting matching between {:?} and {:?} children",
                left_nt.get_children().len(),
                right_nt.get_children().len()
            );

            let (label_matchings, label_score, remaining_left_children, remaining_right_children) =
                calculate_label_matchings(left_nt, right_nt);

            log::debug!(
                "After matching with label there are {:?} and {:?} remaining children",
                remaining_left_children.len(),
                remaining_right_children.len()
            );

            if remaining_left_children.is_empty() && remaining_right_children.is_empty() {
                log::debug!(
                    "Matching children of \"{}\" with \"{}\" using unique label matching.",
                    left.kind(),
                    right.kind()
                );
                let mut result = label_matchings;
                result.extend(Matchings::from_single(
                    UnorderedPair(left, right),
                    MatchingEntry::new(left, right, label_score + root_matching),
                ));
                result
            } else {
                log::debug!(
                    "Matching children of \"{}\" with \"{}\" using hybrid unique label plus assignment problem matching.",
                    left.kind(),
                    right.kind()
                );

                let assignment_matchings = assignment_problem::calculate_matchings_for_children(
                    left,
                    right,
                    &remaining_left_children,
                    &remaining_right_children,
                );

                let assignment_score = assignment_matchings
                    .get_matching_entry(left, right)
                    .map(|matching_entry| matching_entry.score)
                    .unwrap_or(0);

                let mut result = label_matchings;
                for (pair, entry) in assignment_matchings.into_iter() {
                    if pair != UnorderedPair(left, right) {
                        result.extend(Matchings::from_single(pair, entry));
                    }
                }
                result.extend(Matchings::from_single(
                    UnorderedPair(left, right),
                    MatchingEntry::new(left, right, label_score + assignment_score),
                ));
                result
            }
        }
        _ => unreachable!("Unordered matching is only supported for non-terminals."),
    }
}

fn calculate_label_matchings<'a>(
    left_nt: &'a NonTerminal<'a>,
    right_nt: &'a NonTerminal<'a>,
) -> (
    crate::Matchings<'a>,
    usize,
    Vec<&'a CSTNode<'a>>,
    Vec<&'a CSTNode<'a>>,
) {
    let left_children: Vec<&'a CSTNode<'a>> = left_nt
        .get_children()
        .iter()
        .filter(|child| !is_delimiter(child, left_nt.delimiters))
        .collect();
    let right_children: Vec<&'a CSTNode<'a>> = right_nt
        .get_children()
        .iter()
        .filter(|child| !is_delimiter(child, right_nt.delimiters))
        .collect();

    let left_identifier_counts = identifier_counts(&left_children);
    let right_identifier_counts = identifier_counts(&right_children);
    let shared_unique_identifiers =
        shared_unique_identifiers(&left_identifier_counts, &right_identifier_counts);

    let right_children_by_identifier: HashMap<_, _> = right_children
        .iter()
        .filter_map(|child| child_identifier(child).map(|identifier| (identifier, *child)))
        .collect();

    let mut result = crate::Matchings::empty();
    let mut matched_identifiers = HashSet::new();
    let mut remaining_left = Vec::new();
    let mut label_score = 0;

    for left_child in left_children {
        match child_identifier(left_child) {
            Some(identifier) if shared_unique_identifiers.contains(&identifier) => {
                if let Some(right_child) = right_children_by_identifier.get(&identifier) {
                    let child_matchings = crate::calculate_matchings(left_child, right_child);

                    if let Some(matching_entry) =
                        child_matchings.get_matching_entry(left_child, right_child)
                    {
                        if matching_entry.score >= 1 {
                            matched_identifiers.insert(identifier);
                            label_score += matching_entry.score;
                            result.extend(child_matchings);
                            continue;
                        }
                    }
                }

                remaining_left.push(left_child);
            }
            _ => remaining_left.push(left_child),
        }
    }

    let mut remaining_right = Vec::new();
    for right_child in right_children {
        match child_identifier(right_child) {
            Some(identifier) if matched_identifiers.contains(&identifier) => {}
            _ => remaining_right.push(right_child),
        }
    }

    (result, label_score, remaining_left, remaining_right)
}

fn identifier_counts<'a>(children: &[&'a CSTNode<'a>]) -> HashMap<String, usize> {
    let mut counts = HashMap::with_capacity(children.len());

    for child in children {
        if let Some(identifier) = child_identifier(child) {
            *counts.entry(identifier).or_insert(0) += 1;
        }
    }

    counts
}

fn shared_unique_identifiers(
    left_counts: &HashMap<String, usize>,
    right_counts: &HashMap<String, usize>,
) -> HashSet<String> {
    left_counts
        .iter()
        .filter_map(|(identifier, left_count)| {
            (left_count == &1 && right_counts.get(identifier) == Some(&1))
                .then_some(identifier.clone())
        })
        .collect()
}

fn child_identifier<'a>(child: &'a CSTNode<'a>) -> Option<String> {
    match child {
        CSTNode::Terminal(terminal) => Some(format!(
            "terminal:{}\u{1f}{}",
            terminal.kind, terminal.value
        )),
        CSTNode::NonTerminal(non_terminal) => non_terminal.get_identifier().map(|identifier| {
            format!(
                "nonterminal:{}\u{1f}{}",
                non_terminal.kind,
                identifier.join("\u{1f}")
            )
        }),
    }
}

fn is_delimiter(child: &CSTNode<'_>, delimiters: Option<&Delimiters<'_>>) -> bool {
    delimiters.is_some_and(|delimiters| delimiters.is_delimiter(child))
}

#[cfg(test)]
mod tests {
    use model::{
        cst_node::{NonTerminal, Terminal},
        CSTNode, Point,
    };

    use crate::unordered::identifier_counts;

    fn all_children_have_unique_identifiers(node: &NonTerminal) -> bool {
        let children: Vec<&CSTNode<'_>> = node.get_children().iter().collect();
        let counts = identifier_counts(&children);
        counts.values().all(|count| *count == 1)
    }

    #[test]
    fn it_accepts_children_with_unique_identifiers() {
        let left_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "identifier",
            value: "left",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 4 },
            leading_white_space: None,
        });
        let right_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "identifier",
            value: "right",
            start_position: Point { row: 0, column: 5 },
            end_position: Point { row: 0, column: 10 },
            leading_white_space: None,
        });
        let node = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "object",
            children: vec![left_child, right_child],
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 10 },
            are_children_unordered: true,
            identifier: None,
            leading_white_space: None,
            delimiters: None,
        };

        assert!(all_children_have_unique_identifiers(&node));
    }

    #[test]
    fn it_rejects_children_with_duplicate_identifiers() {
        let shared_child = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "pair",
            children: vec![],
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 1 },
            are_children_unordered: false,
            identifier: Some(vec!["abbr"]),
            leading_white_space: None,
            delimiters: None,
        });
        let node = NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "object",
            children: vec![shared_child.clone(), shared_child],
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 1 },
            are_children_unordered: true,
            identifier: None,
            leading_white_space: None,
            delimiters: None,
        };

        assert!(!all_children_have_unique_identifiers(&node));
    }

    #[test]
    fn it_combines_unique_label_and_assignment_matchings() {
        let unique_left_child = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "pair",
            children: vec![],
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 1 },
            are_children_unordered: false,
            identifier: Some(vec!["unique"]),
            leading_white_space: None,
            delimiters: None,
        });
        let unique_right_child = unique_left_child.clone();

        let duplicate_left_child = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "pair",
            children: vec![],
            start_position: Point { row: 0, column: 2 },
            end_position: Point { row: 0, column: 3 },
            are_children_unordered: false,
            identifier: Some(vec!["dup"]),
            leading_white_space: None,
            delimiters: None,
        });
        let duplicate_right_child = duplicate_left_child.clone();

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "object",
            children: vec![
                unique_left_child,
                duplicate_left_child.clone(),
                duplicate_left_child,
            ],
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 3 },
            are_children_unordered: true,
            identifier: None,
            leading_white_space: None,
            delimiters: None,
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "object",
            children: vec![
                unique_right_child,
                duplicate_right_child.clone(),
                duplicate_right_child,
            ],
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 3 },
            are_children_unordered: true,
            identifier: None,
            leading_white_space: None,
            delimiters: None,
        });

        let matchings = super::calculate_matchings(&left, &right);
        let root_matching = matchings.get_matching_entry(&left, &right).unwrap();

        assert_eq!(4, root_matching.score);
        assert!(root_matching.is_perfect_match);
    }
}
