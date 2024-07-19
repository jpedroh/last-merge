use matching_handlers::MatchingHandlers;
use model::Language;

pub struct MatchingConfiguration<'a> {
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
                handlers: MatchingHandlers::from(Language::Java),
            },
        }
    }
}
