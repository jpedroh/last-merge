use std::process::Command;

use assert_cmd::prelude::*;

#[test]
fn if_there_is_a_conflict_it_returns_valid_exit_code() {
    let mut cmd = Command::cargo_bin("last-merge").unwrap();
    cmd.arg("merge")
        .arg("--base-path=tests/scenarios/smoke_java/base.java")
        .arg("--left-path=tests/scenarios/smoke_java/left.java")
        .arg("--right-path=tests/scenarios/smoke_java/right.java")
        .arg("--merge-path=tests/scenarios/smoke_java/merge.output.java")
        .arg("--language=java")
        .assert()
        .code(bin::SUCCESS_WITH_CONFLICTS);
}

#[test]
fn if_there_is_no_conflict_it_returns_valid_exit_code() {
    let mut cmd = Command::cargo_bin("last-merge").unwrap();
    cmd.arg("merge")
        .arg("--base-path=tests/scenarios/no_conflicts/base.java")
        .arg("--left-path=tests/scenarios/no_conflicts/left.java")
        .arg("--right-path=tests/scenarios/no_conflicts/right.java")
        .arg("--merge-path=tests/scenarios/no_conflicts/merge.output.java")
        .arg("--language=java")
        .assert()
        .code(bin::SUCCESS_WITHOUT_CONFLICTS);
}

#[test]
fn if_i_am_running_on_diff_mode_and_files_fully_match_it_returns_zero() {
    let mut cmd = Command::cargo_bin("last-merge").unwrap();
    cmd.arg("diff")
        .arg("--left-path=tests/diff_scenarios/java_files_full_match/left.java")
        .arg("--right-path=tests/diff_scenarios/java_files_full_match/right.java")
        .arg("--language=java")
        .assert()
        .code(bin::SUCCESS_FILES_FULLY_MATCH);
}

#[test]
fn if_i_am_running_on_diff_mode_and_files_do_not_fully_match_it_returns_one() {
    let mut cmd = Command::cargo_bin("last-merge").unwrap();
    cmd.arg("diff")
        .arg("--left-path=tests/diff_scenarios/java_files_not_fully_matching/left.java")
        .arg("--right-path=tests/diff_scenarios/java_files_not_fully_matching/right.java")
        .arg("--language=java")
        .assert()
        .code(bin::SUCCESS_FILES_DO_NOT_FULLY_MATCH);
}
