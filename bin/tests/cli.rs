use std::process::Command;

use assert_cmd::prelude::*;

#[test]
fn it_does_not_crash_and_produces_the_expected_output() {
    let mut cmd = Command::cargo_bin("generic-merge").unwrap();
    cmd.arg("--base-path=tests/samples/smoke_java/base.java")
        .arg("--left-path=tests/samples/smoke_java/left.java")
        .arg("--right-path=tests/samples/smoke_java/right.java")
        .arg("--merge-path=tests/samples/smoke_java/merge.output.java")
        .assert()
        .code(0);

    assert_eq!(
        std::fs::read_to_string("tests/samples/smoke_java/merge.expected.java").unwrap(),
        std::fs::read_to_string("tests/samples/smoke_java/merge.output.java").unwrap()
    )
}

#[test]
fn if_left_equals_base_then_output_right_as_result() {
    let mut cmd = Command::cargo_bin("generic-merge").unwrap();
    cmd.arg("--base-path=tests/samples/one_parent_equals_base/base.java")
        .arg("--left-path=tests/samples/one_parent_equals_base/base.java")
        .arg("--right-path=tests/samples/one_parent_equals_base/changed_parent.java")
        .arg("--merge-path=tests/samples/one_parent_equals_base/merge.output.right.java")
        .assert()
        .code(0);

    assert_eq!(
        std::fs::read_to_string("tests/samples/one_parent_equals_base/changed_parent.java")
            .unwrap(),
        std::fs::read_to_string("tests/samples/one_parent_equals_base/merge.output.right.java")
            .unwrap()
    )
}

#[test]
fn if_right_equals_base_then_output_left_as_result() {
    let mut cmd = Command::cargo_bin("generic-merge").unwrap();
    cmd.arg("--base-path=tests/samples/one_parent_equals_base/base.java")
        .arg("--left-path=tests/samples/one_parent_equals_base/base.java")
        .arg("--right-path=tests/samples/one_parent_equals_base/changed_parent.java")
        .arg("--merge-path=tests/samples/one_parent_equals_base/merge.output.left.java")
        .assert()
        .code(0);

    assert_eq!(
        std::fs::read_to_string("tests/samples/one_parent_equals_base/changed_parent.java")
            .unwrap(),
        std::fs::read_to_string("tests/samples/one_parent_equals_base/merge.output.left.java")
            .unwrap()
    )
}

#[test]
fn it_works_on_semi_structured_merge() {
    let mut cmd = Command::cargo_bin("generic-merge").unwrap();
    cmd.arg("--base-path=tests/samples/semi_structured/base.java")
        .arg("--left-path=tests/samples/semi_structured/left.java")
        .arg("--right-path=tests/samples/semi_structured/right.java")
        .arg("--merge-path=tests/samples/semi_structured/merge.output.java")
        .assert()
        .code(0);

    assert_eq!(
        std::fs::read_to_string("tests/samples/semi_structured/merge.expected.java").unwrap(),
        std::fs::read_to_string("tests/samples/semi_structured/merge.output.java").unwrap()
    )
}

#[test]
fn it_works_on_node_reordering() {
    let mut cmd = Command::cargo_bin("generic-merge").unwrap();
    cmd.arg("--base-path=tests/samples/node_reordering/base.java")
        .arg("--left-path=tests/samples/node_reordering/left.java")
        .arg("--right-path=tests/samples/node_reordering/right.java")
        .arg("--merge-path=tests/samples/node_reordering/merge.output.java")
        .assert()
        .code(0);

    assert_eq!(
        std::fs::read_to_string("tests/samples/node_reordering/merge.expected.java").unwrap(),
        std::fs::read_to_string("tests/samples/node_reordering/merge.output.java").unwrap()
    )
}

#[test]
fn it_works_on_no_conflicts() {
    let mut cmd = Command::cargo_bin("generic-merge").unwrap();
    cmd.arg("--base-path=tests/samples/no_conflicts/base.java")
        .arg("--left-path=tests/samples/no_conflicts/left.java")
        .arg("--right-path=tests/samples/no_conflicts/right.java")
        .arg("--merge-path=tests/samples/no_conflicts/merge.output.java")
        .assert()
        .code(0);

    assert_eq!(
        std::fs::read_to_string("tests/samples/no_conflicts/merge.expected.java").unwrap(),
        std::fs::read_to_string("tests/samples/no_conflicts/merge.output.java").unwrap()
    )
}