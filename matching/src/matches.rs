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
                left.kind == right.kind && left.get_identifier() == right.get_identifier()
            }
            (_, _) => false,
        }
    }
}
