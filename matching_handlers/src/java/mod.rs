mod utils;

use crate::MatchingHandlers;

pub fn get_default_java_matching_handlers<'a>() -> MatchingHandlers<'a> {
    let matching_handlers: MatchingHandlers<'a> = MatchingHandlers::new();
    matching_handlers
}
