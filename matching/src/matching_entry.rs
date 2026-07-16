use model::CSTNode;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchingEntry {
    pub score: usize,
    pub is_perfect_match: bool,
}

impl MatchingEntry {
    pub fn new(left: &CSTNode, right: &CSTNode, score: usize) -> Self {
        MatchingEntry {
            score,
            is_perfect_match: (2 * score)
                == (left.get_tree_size_without_delimiters()
                    + right.get_tree_size_without_delimiters()),
        }
    }
}

impl Default for &MatchingEntry {
    fn default() -> Self {
        &MatchingEntry {
            score: 0,
            is_perfect_match: false,
        }
    }
}
