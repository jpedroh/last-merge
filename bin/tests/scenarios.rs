#[test]
fn all_java_samples_work_correctly() -> Result<(), Box<dyn std::error::Error>> {
    let sample_names = get_samples_names(model::Language::Java)?;

    for sample_path in sample_names {
        let base = std::fs::read_to_string(format!("{}/base.java", sample_path.display()))?;
        let left = std::fs::read_to_string(format!("{}/left.java", sample_path.display()))?;
        let right = std::fs::read_to_string(format!("{}/right.java", sample_path.display()))?;

        let expected = std::fs::read_to_string(format!("{}/merge.java", sample_path.display()))?;
        let result = bin::run_tool_on_merge_scenario(model::Language::Java, &base, &left, &right)
            .map_err(|err| format!("Failed on {} with error: {}", sample_path.display(), err));

        assert_eq!(
            expected.trim(),
            result?.to_string().trim(),
            "Failed on {}",
            sample_path.display()
        );
    }

    Ok(())
}

fn get_samples_names(language: model::Language) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let language_directory = match language {
        model::Language::Java => "java",
    };

    std::fs::read_dir(format!("tests/scenarios/{language_directory}"))?
        .filter(|sample| {
            sample
                .as_ref()
                .map(|sample| sample.path().is_dir())
                .unwrap_or(false)
        })
        .map(|sample| sample.map(|sample| sample.path()))
        .collect()
}
