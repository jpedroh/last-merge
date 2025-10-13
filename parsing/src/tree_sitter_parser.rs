use model::Language;
use parsing_handlers::ParsingHandlers;
use std::collections::{HashMap, HashSet};

use crate::identifier_extractor::{IdentifierExtractor, TreeSitterQuery};

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
                language: tree_sitter_java::LANGUAGE.into(),
                stop_compilation_at: ["string_literal"].into(),
                kinds_with_unordered_children: [
                    "interface_body",
                    "class_body",
                    "modifiers",
                    "enum_body_declarations",
                ]
                .into(),
                block_end_delimiters: ["}"].into(),
                handlers: ParsingHandlers::from(Language::Java),
                identifier_extractors: {
                    let mut map: HashMap<&'static str, Box<dyn IdentifierExtractor>> =
                        HashMap::new();
                    map.insert(
                        "constructor_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(constructor_declaration name: (identifier) @method_name parameters: (formal_parameters ([ (formal_parameter type: _@parameter_type) (spread_parameter (type_identifier) @parameter_type "..." @parameter_type _) ] "," ?) *))"#,
                            tree_sitter_java::LANGUAGE.into(),
                        )),
                    );
                    map.insert(
                        "method_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(method_declaration name: (identifier) @method_name parameters: (formal_parameters ([ (formal_parameter type: _@parameter_type) (spread_parameter (type_identifier) @parameter_type "..." @parameter_type _) ] "," ?) *))"#,
                            tree_sitter_java::LANGUAGE.into(),
                        )),
                    );
                    map.insert(
                        "field_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(variable_declarator name: _ @name)"#,
                            tree_sitter_java::LANGUAGE.into(),
                        )),
                    );
                    map.insert(
                        "import_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(import_declaration "import" "static"? @resource (scoped_identifier) @resource (asterisk)? @resource)"#,
                            tree_sitter_java::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "class_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(class_declaration (identifier) @class_name)"#,
                            tree_sitter_java::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "enum_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(enum_declaration (identifier) @class_name)"#,
                            tree_sitter_java::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "interface_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(interface_declaration (identifier) @class_name)"#,
                            tree_sitter_java::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "variable_declarator",
                        Box::new(TreeSitterQuery::new(
                            r#"(variable_declarator (identifier) @name)"#,
                            tree_sitter_java::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "object_creation_expression",
                        Box::new(TreeSitterQuery::new(
                            r#"(object_creation_expression (type_identifier) @type_identifier)"#,
                            tree_sitter_java::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "marker_annotation",
                        Box::new(TreeSitterQuery::new(
                            r#"(marker_annotation name: _ @name)"#,
                            tree_sitter_java::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "annotation",
                        Box::new(TreeSitterQuery::new(
                            r#"(annotation name: _ @name)"#,
                            tree_sitter_java::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "method_invocation",
                        Box::new(TreeSitterQuery::new(
                            r#"(method_invocation object: (identifier) @object name: (identifier) @method)"#,
                            tree_sitter_java::LANGUAGE.into(),
                        )),
                    );

                    map
                },
            },
            Language::CSharp => Self {
                language: tree_sitter_c_sharp::LANGUAGE.into(),
                stop_compilation_at: HashSet::new(),
                kinds_with_unordered_children: ["declaration_list", "enum_member_declaration_list"]
                    .into(),
                block_end_delimiters: ["}"].into(),
                handlers: ParsingHandlers::new(vec![]),
                identifier_extractors: {
                    let mut map: HashMap<&'static str, Box<dyn IdentifierExtractor>> =
                        HashMap::new();
                    map.insert(
                        "constructor_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(constructor_declaration name: (identifier) @method_name parameters: (parameter_list ([ (parameter type: _@parameter_type) ] "," ?) *))"#,
                            tree_sitter_c_sharp::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "method_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(method_declaration name: (identifier) @method_name parameters: (parameter_list ([ (parameter type: _@parameter_type) ] "," ?) *))"#,
                            tree_sitter_c_sharp::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "class_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(class_declaration (identifier) @class_name)"#,
                            tree_sitter_c_sharp::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "enum_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(enum_declaration (identifier) @class_name)"#,
                            tree_sitter_c_sharp::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "interface_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(interface_declaration (identifier) @class_name)"#,
                            tree_sitter_c_sharp::LANGUAGE.into(),
                        )),
                    );

                    map.insert(
                        "variable_declaration",
                        Box::new(TreeSitterQuery::new(
                            r#"(variable_declarator (identifier) @name)"#,
                            tree_sitter_c_sharp::LANGUAGE.into(),
                        )),
                    );

                    map
                },
            },
        }
    }
}
