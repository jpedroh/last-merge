mod class_like_declaration;
mod field_declaration;
mod import_declaration;
mod method_declaration;
mod utils;

use crate::MatchingHandlers;

use self::{
    class_like_declaration::compute_matching_score_for_class_like_declaration,
    field_declaration::compute_matching_score_for_field_declaration,
    import_declaration::compute_matching_score_for_import_declaration,
    method_declaration::compute_matching_score_for_method_declaration,
};

pub fn get_default_java_matching_handlers<'a>() -> MatchingHandlers<'a> {
    let mut matching_handlers: MatchingHandlers<'a> = MatchingHandlers::new();
    
    matching_handlers.register(
        "constructor_declaration",
        compute_matching_score_for_method_declaration,
    );
    matching_handlers
}
