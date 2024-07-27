use std::collections::HashMap;

use model::CSTNode;
use unordered_pair::UnorderedPair;

use crate::matching::Matching;
use crate::matching_entry::MatchingEntry;

#[derive(Debug, Clone)]
pub struct Matchings<'a> {
    pub matching_entries: HashMap<UnorderedPair<&'a CSTNode<'a>>, MatchingEntry>,
}

impl<'a> Matchings<'a> {
    pub fn empty() -> Self {
        Matchings {
            matching_entries: HashMap::new(),
        }
    }

    pub fn from_single(key: UnorderedPair<&'a CSTNode>, value: MatchingEntry) -> Self {
        Matchings {
            matching_entries: HashMap::from([(key, value)]),
        }
    }

    pub fn new(matching_entries: HashMap<UnorderedPair<&'a CSTNode<'a>>, MatchingEntry>) -> Self {
        Matchings { matching_entries }
    }

    pub fn find_matching_for(&self, a_node: &'a CSTNode) -> Option<Matching> {
        self.matching_entries
            .iter()
            .find(|(UnorderedPair(left, right), ..)| {
                left.id() == a_node.id() || right.id() == a_node.id()
            })
            .map(|(UnorderedPair(left, right), matching)| {
                let matching_node = if left.id() == a_node.id() {
                    right
                } else {
                    left
                };
                Matching {
                    matching_node,
                    score: matching.score,
                    is_perfect_match: matching.is_perfect_match,
                }
            })
    }

    pub fn find_matching_node_in_children(
        &'a self,
        a_node: &'a CSTNode<'a>,
        children: &'a [CSTNode<'a>],
    ) -> Option<&'a MatchingEntry> {
        children
            .iter()
            .find_map(|left_child| self.get_matching_entry(left_child, a_node))
    }

    pub fn get_matching_entry(
        &'a self,
        left: &'a CSTNode<'a>,
        right: &'a CSTNode<'a>,
    ) -> Option<&MatchingEntry> {
        self.matching_entries.get(&UnorderedPair(left, right))
    }

    pub fn extend(&mut self, matchings: Matchings<'a>) {
        self.matching_entries.extend(matchings);
    }
}

impl Default for Matchings<'_> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a> IntoIterator for Matchings<'a> {
    type Item = (UnorderedPair<&'a CSTNode<'a>>, MatchingEntry);

    type IntoIter =
        std::collections::hash_map::IntoIter<UnorderedPair<&'a CSTNode<'a>>, MatchingEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.matching_entries.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use model::{cst_node::Terminal, Point};

    use super::*;

    #[test]
    fn returns_none_if_a_matching_for_the_node_is_not_found() {
        let a_node = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
            is_block_end_delimiter: false,
        });

        assert_eq!(None, Matchings::empty().find_matching_for(&a_node))
    }

    #[test]
    fn returns_some_match_if_a_matching_for_the_node_is_found() {
        let a_node = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
            is_block_end_delimiter: false,
        });

        let mut matchings = HashMap::new();
        matchings.insert(
            UnorderedPair(&a_node, &a_node),
            MatchingEntry::new(&a_node, &a_node, 1),
        );

        assert_eq!(
            Some(Matching {
                matching_node: &a_node,
                score: 1,
                is_perfect_match: true
            }),
            Matchings::new(matchings).find_matching_for(&a_node)
        )
    }
}
