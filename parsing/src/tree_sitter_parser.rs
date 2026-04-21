use model::{cst_node::Delimiters, Language};
use parsing_handlers::ParsingHandlers;
use std::collections::{HashMap, HashSet};

use crate::identifier_extractor::IdentifierExtractor;
use crate::tree_sitter_queries_identifier_extractors;

pub struct ParserConfiguration {
    pub(crate) language: tree_sitter::Language,
    pub(crate) stop_compilation_at: HashSet<&'static str>,
    pub(crate) kinds_with_unordered_children: HashSet<&'static str>,
    pub(crate) delimiters: HashMap<&'static str, Delimiters<'static>>,
    pub(crate) handlers: ParsingHandlers,
    pub(crate) identifier_extractors: HashMap<&'static str, Box<dyn IdentifierExtractor>>,
}

impl From<Language> for ParserConfiguration {
    fn from(language: Language) -> Self {
        match language {
            Language::Java => ParserConfiguration {
                language: tree_sitter_java::LANGUAGE.into(),
                stop_compilation_at: ["string_literal"].into(),
                kinds_with_unordered_children: [
                    "interface_body",
                    "class_body",
                    "modifiers",
                    "enum_body_declarations",
                ]
                .into(),
                delimiters: HashMap::from([
                    ("interface_body", Delimiters::new("{", "}")),
                    ("class_body", Delimiters::new("{", "}")),
                ]),
                handlers: ParsingHandlers::from(Language::Java),
                identifier_extractors: tree_sitter_queries_identifier_extractors! {
                    language: tree_sitter_java::LANGUAGE,
                    queries: {
                        "constructor_declaration": r#"(constructor_declaration name: (identifier) @method_name parameters: (formal_parameters ([ (formal_parameter type: _@parameter_type) (spread_parameter (type_identifier) @parameter_type "..." @parameter_type _) ] "," ?) *))"#,
                        "method_declaration": r#"(method_declaration name: (identifier) @method_name parameters: (formal_parameters ([ (formal_parameter type: _@parameter_type) (spread_parameter (type_identifier) @parameter_type "..." @parameter_type _) ] "," ?) *))"#,
                        "field_declaration": r#"(variable_declarator name: _ @name)"#,
                        "import_declaration": r#"(import_declaration "import" "static"? @resource (scoped_identifier) @resource (asterisk)? @resource)"#,
                        "class_declaration": r#"(class_declaration (identifier) @class_name)"#,
                        "enum_declaration": r#"(enum_declaration (identifier) @class_name)"#,
                        "interface_declaration": r#"(interface_declaration (identifier) @class_name)"#,
                        "variable_declarator": r#"(variable_declarator (identifier) @name)"#,
                        "object_creation_expression": r#"(object_creation_expression (type_identifier) @type_identifier)"#,
                        "marker_annotation": r#"(marker_annotation name: _ @name)"#,
                        "annotation": r#"(annotation name: _ @name)"#,
                        "method_invocation": r#"(method_invocation object: (identifier) @object name: (identifier) @method)"#,
                    }
                },
            },
            Language::CSharp => Self {
                language: tree_sitter_c_sharp::LANGUAGE.into(),
                stop_compilation_at: HashSet::new(),
                kinds_with_unordered_children: ["declaration_list", "enum_member_declaration_list"]
                    .into(),
                delimiters: HashMap::from([("declaration_list", Delimiters::new("{", "}"))]),
                handlers: ParsingHandlers::new(vec![]),
                identifier_extractors: tree_sitter_queries_identifier_extractors! {
                    language: tree_sitter_c_sharp::LANGUAGE,
                    queries: {
                        "constructor_declaration": r#"(constructor_declaration name: (identifier) @method_name parameters: (parameter_list ([ (parameter type: _@parameter_type) ] "," ?) *))"#,
                        "method_declaration": r#"(method_declaration name: (identifier) @method_name parameters: (parameter_list ([ (parameter type: _@parameter_type) ] "," ?) *))"#,
                        "class_declaration": r#"(class_declaration (identifier) @class_name)"#,
                        "enum_declaration": r#"(enum_declaration (identifier) @class_name)"#,
                        "interface_declaration": r#"(interface_declaration (identifier) @class_name)"#,
                        "variable_declaration": r#"(variable_declarator (identifier) @name)"#,
                    }
                },
            },
        }
    }
}
