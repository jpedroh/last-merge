use std::collections::HashMap;

use model::CSTNode;
use unordered_pair::UnorderedPair;

use crate::matching::Matching;
use crate::matching_entry::MatchingEntry;

#[derive(Debug, Clone)]
pub struct Matchings<'a> {
    matching_entries: HashMap<UnorderedPair<&'a CSTNode<'a>>, MatchingEntry>,
    individual_matchings: HashMap<&'a CSTNode<'a>, &'a CSTNode<'a>>,
}

impl<'a> Matchings<'a> {
    pub fn with_capacity(capacity: usize) -> Self {
        Matchings {
            matching_entries: HashMap::with_capacity(capacity),
            individual_matchings: HashMap::with_capacity(capacity * 2),
        }
    }

    pub fn empty() -> Self {
        Matchings {
            matching_entries: HashMap::new(),
            individual_matchings: HashMap::new(),
        }
    }

    pub fn from_single(left: &'a CSTNode<'a>, right: &'a CSTNode<'a>, score: usize) -> Self {
        Matchings {
            matching_entries: HashMap::from([(
                UnorderedPair(left, right),
                MatchingEntry::new(left, right, score),
            )]),
            individual_matchings: HashMap::from([(left, right), (right, left)]),
        }
    }

    pub fn new(matching_entries: HashMap<UnorderedPair<&'a CSTNode<'a>>, MatchingEntry>) -> Self {
        let mut individual_matchings = HashMap::with_capacity(matching_entries.len() * 2);
        for key in matching_entries.keys() {
            individual_matchings.insert(key.0, key.1);
            individual_matchings.insert(key.1, key.0);
        }

        Matchings {
            individual_matchings,
            matching_entries,
        }
    }

    pub fn find_matching_for(&'a self, a_node: &'a CSTNode) -> Option<Matching<'a>> {
        let matching_node = self.individual_matchings.get(a_node)?;
        let matching_entry = self
            .matching_entries
            .get(&UnorderedPair(a_node, matching_node))?;
        Some(Matching {
            matching_node,
            score: matching_entry.score,
            is_perfect_match: matching_entry.is_perfect_match,
        })
    }

    pub fn get_matching_entry(
        &'a self,
        left: &'a CSTNode<'a>,
        right: &'a CSTNode<'a>,
    ) -> Option<&'a MatchingEntry> {
        self.matching_entries.get(&UnorderedPair(left, right))
    }

    pub fn extend(&mut self, matchings: Matchings<'a>) {
        self.individual_matchings.reserve(matchings.len() * 2);
        self.matching_entries.reserve(matchings.len());
        for (UnorderedPair(left, right), entry) in matchings.matching_entries.iter() {
            self.push(left, right, entry.score);
        }
    }

    fn len(&self) -> usize {
        self.matching_entries.len()
    }

    pub fn push(&mut self, left: &'a CSTNode<'a>, right: &'a CSTNode<'a>, score: usize) {
        if self
            .get_matching_entry(left, right)
            .is_some_and(|existing| existing.score > score)
        {
            log::debug!("Early returning because a matching with higher score already exists");
        } else {
            self.individual_matchings.insert(left, right);
            self.individual_matchings.insert(right, left);
            self.matching_entries.insert(
                UnorderedPair(left, right),
                MatchingEntry::new(left, right, score),
            );
        }
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
            leading_white_space: None,
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
            leading_white_space: None,
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
