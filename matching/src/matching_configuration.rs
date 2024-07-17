use matching_handlers::MatchingHandlers;
use model::Language;
use std::collections::HashSet;

pub struct MatchingConfiguration<'a> {
    pub(crate) delimiters: HashSet<&'static str>,
    pub(crate) kinds_with_label: HashSet<&'static str>,
    pub(crate) handlers: MatchingHandlers<'a>,
}

impl Default for MatchingConfiguration<'_> {
    fn default() -> Self {
        MatchingConfiguration::from(Language::Java)
    }
}

impl From<Language> for MatchingConfiguration<'_> {
    fn from(language: Language) -> Self {
        match language {
            Language::Java => MatchingConfiguration {
                delimiters: ["{", "}", ";"].into(),
                kinds_with_label: [
                    "compact_constructor_declaration",
                    "constructor_declaration",
                    "field_declaration",
                    "method_declaration",
                    "import_declaration",
                    "class_declaration",
                    "interface_declaration",
                    "enum_declaration"
                ]
                .into(),
                handlers: MatchingHandlers::from(Language::Java),
            },
        }
    }
}
