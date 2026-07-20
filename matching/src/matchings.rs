use std::collections::HashMap;

use model::CSTNode;
use unordered_pair::UnorderedPair;

use crate::matching::Matching;
use crate::matching_entry::MatchingEntry;
use rustc_hash::FxBuildHasher;

#[derive(Debug, Clone)]
pub struct Matchings<'a> {
    matching_entries: HashMap<UnorderedPair<&'a CSTNode<'a>>, MatchingEntry, FxBuildHasher>,
    individual_matchings: HashMap<&'a CSTNode<'a>, &'a CSTNode<'a>, FxBuildHasher>,
}

impl<'a> Matchings<'a> {
    pub fn with_capacity(capacity: usize) -> Self {
        Matchings {
            matching_entries: HashMap::with_capacity_and_hasher(capacity, FxBuildHasher),
            individual_matchings: HashMap::with_capacity_and_hasher(capacity * 2, FxBuildHasher),
        }
    }

    pub fn empty() -> Self {
        Matchings {
            matching_entries: HashMap::default(),
            individual_matchings: HashMap::default(),
        }
    }

    // Safe because Hash/Eq are based exclusively on the immutable node id.
    // Cached fields are not considered for identity.
    #[allow(clippy::mutable_key_type)]
    pub fn new(
        matching_entries: HashMap<UnorderedPair<&'a CSTNode<'a>>, MatchingEntry, FxBuildHasher>,
    ) -> Self {
        let mut individual_matchings =
            HashMap::with_capacity_and_hasher(matching_entries.len() * 2, FxBuildHasher);
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

    pub fn len(&self) -> usize {
        self.matching_entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, left: &'a CSTNode<'a>, right: &'a CSTNode<'a>, score: usize) {
        if let Some(existing) = self
            .get_matching_entry(left, right)
            .filter(|existing| existing.score > score)
        {
            log::debug!("Early returning because a matching with higher score ({:?} vs {:?}) already exists for {:?} and {:?}", existing.score, score, left.contents(), right.contents());
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

        let mut matchings = HashMap::with_hasher(FxBuildHasher);
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

    #[test]
    fn if_there_are_no_matchings_is_empty_returns_true() {
        let matchings = Matchings::empty();
        assert!(matchings.is_empty());
    }

    #[test]
    fn if_there_is_already_a_matching_with_higher_score_push_has_no_action() {
        let mut matchings = Matchings::empty();
        let left = CSTNode::Terminal(Terminal::default());
        let right = CSTNode::Terminal(Terminal::default());

        matchings.push(&left, &right, 20);
        matchings.push(&left, &right, 10);

        let matching = matchings
            .get_matching_entry(&left, &right)
            .expect("matching should exist");

        assert_eq!(20, matching.score);
    }
}
