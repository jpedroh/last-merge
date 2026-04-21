#[macro_export]
macro_rules! tree_sitter_queries_identifier_extractors {
    (
        language: $language:expr,
        queries: {
            $( $key:literal : $query:literal ),* $(,)?
        }
    ) => {{
        use $crate::identifier_extractor::{IdentifierExtractor, TreeSitterQuery};
        std::collections::HashMap::from([
            $(
                (
                    $key,
                    Box::new(TreeSitterQuery::new($query, $language.into()))
                        as Box<dyn IdentifierExtractor>,
                ),
            )*
        ])
    }};
}
