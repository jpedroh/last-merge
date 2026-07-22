mod assignment_problem;
mod unique_label;

use crate::Matchings;

#[tracing::instrument(skip( matchings))]
pub fn calculate_subtree_matching<'a>(
    left: &'a model::cst_node::NonTerminal<'a>,
    right: &'a model::cst_node::NonTerminal<'a>,
    matchings: &mut Matchings<'a>,
) -> usize {
    log::debug!(
        "Starting matching between {:?} and {:?} children",
        left.get_children().len(),
        right.get_children().len()
    );

    let (label_matchings, label_score, remaining_left_children, remaining_right_children) =
        unique_label::calculate_label_matchings(left, right);

    matchings.extend(label_matchings);

    log::debug!(
        "After matching with label there are {:?} and {:?} remaining children",
        remaining_left_children.len(),
        remaining_right_children.len()
    );

    if remaining_left_children.is_empty() && remaining_right_children.is_empty() {
        log::debug!(
            "Matching children of \"{}\" with \"{}\" using unique label matching.",
            left.kind,
            right.kind
        );
        label_score
    } else {
        log::debug!(
                    "Matching children of \"{}\" with \"{}\" using hybrid unique label plus assignment problem matching.",
                    left.kind,
                    right.kind
                );

        let assignment_score = assignment_problem::calculate_matchings_for_children(
            &remaining_left_children,
            &remaining_right_children,
            matchings,
        );

        label_score + assignment_score
    }
}

#[cfg(test)]
mod tests {
    use crate::Matchings;
    use model::{cst_node::NonTerminal, CSTNode, Point};

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
            subtree_size_without_delimiters: std::cell::OnceCell::new(),
            subtree_size: std::cell::OnceCell::new(),
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
            subtree_size_without_delimiters: std::cell::OnceCell::new(),
            subtree_size: std::cell::OnceCell::new(),
        });
        let duplicate_right_child = duplicate_left_child.clone();

        let left = NonTerminal {
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
            subtree_size_without_delimiters: std::cell::OnceCell::new(),
            subtree_size: std::cell::OnceCell::new(),
        };
        let right = NonTerminal {
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
            subtree_size_without_delimiters: std::cell::OnceCell::new(),
            subtree_size: std::cell::OnceCell::new(),
        };

        let mut matchings = Matchings::empty();
        let children_matching_score =
            super::calculate_subtree_matching(&left, &right, &mut matchings);
        assert_eq!(3, children_matching_score);
    }
}
