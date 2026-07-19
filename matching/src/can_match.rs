use model::CSTNode;

pub trait CanMatch {
    fn can_match(&self, other: &Self) -> bool;
}

impl CanMatch for CSTNode<'_> {
    fn can_match(&self, other: &CSTNode) -> bool {
        match (self, other) {
            (CSTNode::Terminal(left), CSTNode::Terminal(right)) => {
                left.get_identifier() == right.get_identifier()
            }
            (CSTNode::NonTerminal(left), CSTNode::NonTerminal(right)) => {
                left.kind == right.kind && left.get_identifier() == right.get_identifier()
            }
            (_, _) => false,
        }
    }
}
