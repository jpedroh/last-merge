use unordered_pair::UnorderedPair;

use crate::{matches::Matches, MatchingEntry, Matchings};

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode,
    right: &'a model::CSTNode,
) -> Matchings<'a> {
    match (left, right) {
        (model::CSTNode::NonTerminal(nt_left), model::CSTNode::NonTerminal(nt_right)) => {
            let root_matching: usize = left.matches(right).into();

            let mut sum = 0;
            let mut result = Matchings::empty();

            for child_left in nt_left.get_children() {
                for child_right in nt_right.get_children() {
                    let is_same_identifier = match (child_left, child_right) {
                        (model::CSTNode::Terminal(left), model::CSTNode::Terminal(right)) => {
                            left.get_identifier() == right.get_identifier()
                        }
                        (model::CSTNode::NonTerminal(left), model::CSTNode::NonTerminal(right)) => {
                            left.get_identifier() == right.get_identifier()
                        }
                        (_, _) => false,
                    };

                    if is_same_identifier {
                        let child_matchings = crate::calculate_matchings(child_left, child_right);

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
