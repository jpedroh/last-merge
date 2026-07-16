use std::collections::HashSet;

use model::cst_node::NonTerminal;

mod assignment_problem;
mod unique_label;

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode<'a>,
    right: &'a model::CSTNode<'a>,
) -> crate::Matchings<'a> {
    match (left, right) {
        (model::CSTNode::NonTerminal(left_nt), model::CSTNode::NonTerminal(right_nt)) => {
            if all_children_have_unique_identifiers(left_nt)
                && all_children_have_unique_identifiers(right_nt)
            {
                log::debug!(
                    "Matching children of \"{}\" with \"{}\" using unique label matching.",
                    left.kind(),
                    right.kind()
                );
                unique_label::calculate_matchings(left, right)
            } else {
                log::debug!(
                    "Matching children of \"{}\" with \"{}\" using assignment problem matching.",
                    left.kind(),
                    right.kind()
                );
                assignment_problem::calculate_matchings(left, right)
            }
        }
        _ => unreachable!("Unordered matching is only supported for non-terminals."),
    }
}

fn all_children_have_unique_identifiers(node: &NonTerminal) -> bool {
    let mut seen = HashSet::with_capacity(node.children.len());

    node.children
        .iter()
        .filter(|child| {
            node.delimiters
                .map(|delimiters| !delimiters.is_delimiter(child))
                .unwrap_or(true)
        })
        .all(|child| {
            let identifier = match child {
                model::CSTNode::Terminal(terminal) => {
                    Some(format!("terminal:{}\u{1f}{}", terminal.kind, terminal.value))
                }
                model::CSTNode::NonTerminal(non_terminal) => non_terminal
                    .get_identifier()
                    .map(|identifier| format!("nonterminal:{}\u{1f}{}", non_terminal.kind, identifier.join("\u{1f}"))),
            };

            identifier.is_some_and(|identifier| seen.insert(identifier))
        })
}

#[cfg(test)]
mod tests {
    use model::{
        cst_node::{NonTerminal, Terminal},
        CSTNode, Point,
    };

    use super::all_children_have_unique_identifiers;

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
}
