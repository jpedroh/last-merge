use model::Language;
use parsing_handlers::ParsingHandlers;
use std::collections::{HashMap, HashSet};

use crate::identifier_extractor::{
    IdentifierExtractor, RegularExpressionIdentifierExtractor, TreeSitterQueryIdentifierExtractor,
};

pub struct ParserConfiguration {
    pub(crate) language: tree_sitter::Language,
    pub(crate) stop_compilation_at: HashSet<&'static str>,
    pub(crate) kinds_with_unordered_children: HashSet<&'static str>,
    pub(crate) block_end_delimiters: HashSet<&'static str>,
    pub(crate) handlers: ParsingHandlers,
    pub(crate) identifier_extractors: HashMap<&'static str, Box<dyn IdentifierExtractor>>,
}

impl From<Language> for ParserConfiguration {
    fn from(language: Language) -> Self {
        match language {
            Language::Java => ParserConfiguration {
                language: tree_sitter_java::language(),
                stop_compilation_at: [].into(),
                kinds_with_unordered_children: [
                    "interface_body",
                    "class_body",
                    "enum_body_declarations",
                ]
                .into(),
                block_end_delimiters: ["}"].into(),
                handlers: ParsingHandlers::from(Language::Java),
                identifier_extractors: {
                    let mut map: HashMap<&'static str, Box<dyn IdentifierExtractor>> =
                        HashMap::new();
                    map.insert("constructor_declaration", Box::new(TreeSitterQueryIdentifierExtractor::new(r#"(constructor_declaration name: (identifier) @method_name [parameters: (formal_parameters [ (formal_parameter type: (_) @argument_type) (spread_parameter (type_identifier) @spread_parameter "..." @spread_indicator) ]) _ ])"#)));
                    map.insert("method_declaration", Box::new(TreeSitterQueryIdentifierExtractor::new(r#"(method_declaration name: (identifier) @method_name [parameters: (formal_parameters [ (formal_parameter type: (_) @argument_type) (spread_parameter (type_identifier) @spread_parameter "..." @spread_indicator) ]) _ ])"#)));
                    map.insert(
                        "field_declaration",
                        Box::new(TreeSitterQueryIdentifierExtractor::new(
                            r#"(variable_declarator name: _ @name)"#,
                        )),
                    );
                    map.insert(
                        "import_declaration",
                        Box::new(TreeSitterQueryIdentifierExtractor::new(
                            r#"(import_declaration ( scoped_identifier ) @namespace)"#,
                        )),
                    );

                    map.insert(
                        "class_declaration",
                        Box::new(RegularExpressionIdentifierExtractor::new(
                            r#"class [A-Za-z_][A-Za-z0-9_]*"#,
                        )),
                    );

                    map.insert(
                        "enum_declaration",
                        Box::new(RegularExpressionIdentifierExtractor::new(
                            r#"enum [A-Za-z_][A-Za-z0-9_]*"#,
                        )),
                    );

                    map.insert(
                        "interface_declaration",
                        Box::new(RegularExpressionIdentifierExtractor::new(
                            r#"interface [A-Za-z_][A-Za-z0-9_]*"#,
                        )),
                    );
                    map
                },
            },
        }
    }
}
