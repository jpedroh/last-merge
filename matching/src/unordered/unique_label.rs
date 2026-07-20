use std::collections::{HashMap, HashSet};

use model::cst_node::{CSTNode, Delimiters};
use rustc_hash::FxBuildHasher;

pub fn calculate_label_matchings<'a>(
    left_nt: &'a model::cst_node::NonTerminal<'a>,
    right_nt: &'a model::cst_node::NonTerminal<'a>,
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

    let right_children_by_identifier: HashMap<_, _, FxBuildHasher> = right_children
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

fn identifier_counts<'a>(children: &[&'a CSTNode<'a>]) -> HashMap<String, usize, FxBuildHasher> {
    let mut counts = HashMap::with_capacity_and_hasher(children.len(), FxBuildHasher);

    for child in children {
        if let Some(identifier) = child_identifier(child) {
            *counts.entry(identifier).or_insert(0) += 1;
        }
    }

    counts
}

fn shared_unique_identifiers(
    left_counts: &HashMap<String, usize, FxBuildHasher>,
    right_counts: &HashMap<String, usize, FxBuildHasher>,
) -> HashSet<String, FxBuildHasher> {
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

    fn all_children_have_unique_identifiers(node: &NonTerminal) -> bool {
        let children: Vec<&CSTNode<'_>> = node.get_children().iter().collect();
        let counts = super::identifier_counts(&children);
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
            subtree_size_without_delimiters: std::cell::OnceCell::new(),
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
            subtree_size_without_delimiters: std::cell::OnceCell::new(),
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
            subtree_size_without_delimiters: std::cell::OnceCell::new(),
        };

        assert!(!all_children_have_unique_identifiers(&node));
    }
}
