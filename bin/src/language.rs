pub fn get_language_from_name(name: &str) -> Result<model::Language, String> {
    match name {
        "java" => Ok(model::Language::Java),
        "csharp" | "cs" => Ok(model::Language::CSharp),
        _ => Err(format!("Invalid language provided: {name}")),
    }
}

pub fn get_language_by_file_path(file_path: &std::path::Path) -> Result<model::Language, String> {
    file_path
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .and_then(|extension| match extension {
            "java" => Some(model::Language::Java),
            "cs" => Some(model::Language::CSharp),
            _ => None,
        })
        .ok_or(format!(
            "Could not retrieve parsing configuration for file {}",
            file_path.display()
        ))
}

#[cfg(test)]
mod tests {
    use crate::language::get_language_by_file_path;

    #[test]
    fn if_the_file_extension_has_no_parser_available_it_returns_error() {
        let file_path = std::path::PathBuf::from("/path/without/extension");
        assert!(get_language_by_file_path(&file_path).is_err())
    }

    #[test]
    fn if_the_file_extension_has_a_parser_available_it_returns_a_parser_configuration() {
        let file_path = std::path::PathBuf::from("/path/for/java/file/Example.java");
        assert_eq!(
            get_language_by_file_path(&file_path).unwrap(),
            model::Language::Java
        )
    }
}
