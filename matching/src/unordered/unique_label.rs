use model::{cst_node::NonTerminal, CSTNode};
use unordered_pair::UnorderedPair;

use crate::{matching_configuration::MatchingConfiguration, MatchingEntry, Matchings};

pub fn calculate_matchings<'a>(
    left: &'a CSTNode,
    right: &'a CSTNode,
    config: &'a MatchingConfiguration<'a>,
) -> crate::Matchings<'a> {
    match (left, right) {
        (
            CSTNode::NonTerminal(NonTerminal {
                kind: kind_left,
                children: children_left,
                ..
            }),
            CSTNode::NonTerminal(NonTerminal {
                kind: kind_right,
                children: children_right,
                ..
            }),
        ) => {
            let root_matching: usize = (kind_left == kind_right).into();

            let mut sum = 0;
            let mut result = Matchings::empty();

            for child_left in children_left {
                for child_right in children_right {
                    let is_same_identifier = child_left.get_identifier().is_some() && child_left.get_identifier() == child_right.get_identifier();
                
                    if is_same_identifier {
                        let child_matchings =
                            crate::calculate_matchings(child_left, child_right, config);

                        if let Some(matching_entry) =
                            child_matchings.get_matching_entry(child_left, child_right)
                        {
                            if matching_entry.score >= 1 {
                                sum += matching_entry.score;
                                result.extend(child_matchings);
                            }
                        }
                    }
                }
            }

            result.extend(Matchings::from_single(
                UnorderedPair(left, right),
                MatchingEntry::new(left, right, sum + root_matching),
            ));

            result
        }
        _ => unreachable!("Unordered matching is only supported for non-terminals."),
    }
}
