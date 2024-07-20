use model::CSTNode;

pub trait Matches {
    fn matches(&self, right: &CSTNode) -> bool;
}

impl Matches for CSTNode<'_> {
    fn matches(&self, right: &CSTNode) -> bool {
        match (self, right) {
            (CSTNode::Terminal(left), CSTNode::Terminal(right)) => {
                left.get_identifier() == right.get_identifier()
            }
            (CSTNode::NonTerminal(left), CSTNode::NonTerminal(right)) => {
                if let (Some(left_identifier), Some(right_identifier)) =
                    (left.get_identifier(), right.get_identifier())
                {
                    left_identifier == right_identifier
                } else {
                    left.kind == right.kind
                }
            }
            (_, _) => false,
        }
    }
}
