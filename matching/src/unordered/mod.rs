use model::cst_node::NonTerminal;

mod assignment_problem;
mod unique_label;

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode<'a>,
    right: &'a model::CSTNode<'a>,
) -> crate::Matchings<'a> {
    match (left, right) {
        (model::CSTNode::NonTerminal(left_nt), model::CSTNode::NonTerminal(right_nt)) => {
            if all_children_labeled(left_nt) && all_children_labeled(right_nt) {
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

fn all_children_labeled(node: &NonTerminal) -> bool {
    node.children.iter().all(|child| child.has_identifier())
}
