use crate::{java::get_default_java_parsing_handlers, ParsingHandlers};
use model::Language;

impl From<Language> for ParsingHandlers {
    fn from(language: Language) -> Self {
        match language {
            Language::Java => get_default_java_parsing_handlers(),
            Language::CSharp => ParsingHandlers::new(vec![]),
        }
    }
}
