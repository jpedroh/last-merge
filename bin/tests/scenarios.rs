use std::{
    assert_eq,
    path::{Path, PathBuf},
};

use model::Language;
use rstest::rstest;

#[rstest]
fn java_samples(#[files("tests/scenarios/java/**/merge.java")] path: PathBuf) {
    run_sample(path.parent().unwrap(), Language::Java, "java");
}

#[rstest]
fn csharp_samples(#[files("tests/scenarios/csharp/**/merge.cs")] path: PathBuf) {
    run_sample(path.parent().unwrap(), Language::CSharp, "cs");
}

#[rstest]
fn javascript_samples(#[files("tests/scenarios/javascript/**/merge.js")] path: PathBuf) {
    run_sample(path.parent().unwrap(), Language::JavaScript, "js");
}

#[rstest]
fn go_samples(#[files("tests/scenarios/go/**/merge.go")] path: PathBuf) {
    run_sample(path.parent().unwrap(), Language::Go, "go");
}

fn run_sample(path: &Path, language: Language, extension: &str) {
    use std::fs::read_to_string;
    let base =
        read_to_string(path.join(format!("base.{}", extension))).expect("Could not read base");
    let left =
        read_to_string(path.join(format!("left.{}", extension))).expect("Could not read left");
    let right =
        read_to_string(path.join(format!("right.{}", extension))).expect("Could not read right");
    let expected =
        read_to_string(path.join(format!("merge.{}", extension))).expect("Could not read merge");

    let result = bin::run_tool_on_merge_scenario(language, &base, &left, &right, false)
        .expect("Unknown error during merge");

    assert_eq!(expected.trim(), result.to_string().trim());
}
