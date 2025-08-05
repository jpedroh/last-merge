use std::error;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum MergeError {
    NodesWithDifferentKinds(String, String),
    MergingTerminalWithNonTerminal,
    InvalidMatchingConfiguration(bool, bool, bool, bool, bool),
}

impl fmt::Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MergeError::MergingTerminalWithNonTerminal => {
                write!(f, "Merging terminal with non-terminal")
            }
            MergeError::InvalidMatchingConfiguration(a, b, c, d, e) => {
                write!(f, "Invalid matching configuration: {a}, {b}, {c}, {d}, {e}")
            }
            MergeError::NodesWithDifferentKinds(kind_a, kind_b) => {
                write!(
                    f,
                    "Tried to merge node of kind \"{kind_a}\" with node of kind \"{kind_b}\""
                )
            }
        }
    }
}

impl error::Error for MergeError {}
