#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum CSTNode {
    Terminal {
        kind: String,
        value: String,
    },
    NonTerminal {
        kind: String,
        children: Vec<CSTNode>,
    },
}