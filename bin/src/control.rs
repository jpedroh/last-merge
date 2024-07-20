use std::{
    error::Error,
    fmt::{self, Display},
};

use matching::{matching_configuration, MatchingEntry};
use parsing::ParserConfiguration;

#[derive(Debug)]
pub enum ExecutionError {
    ParsingError(&'static str),
    MergeError(merge::MergeError),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::ParsingError(error) => write!(f, "Parsing error occurred: {}", error),
            ExecutionError::MergeError(error) => write!(f, "Merge error occurred: {}", error),
        }
    }
}

impl Error for ExecutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum ExecutionResult {
    WithConflicts(String),
    WithoutConflicts(String),
}

impl Display for ExecutionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionResult::WithConflicts(value) => write!(f, "{}", value),
            ExecutionResult::WithoutConflicts(value) => write!(f, "{}", value),
        }
    }
}

pub fn run_tool_on_merge_scenario(
    language: model::Language,
    base: &str,
    left: &str,
    right: &str,
) -> Result<ExecutionResult, ExecutionError> {
    if base == left {
        return Ok(ExecutionResult::WithoutConflicts(right.to_string()));
    }

    if base == right {
        return Ok(ExecutionResult::WithoutConflicts(left.to_string()));
    }

    let parser_configuration = ParserConfiguration::from(language);

    log::info!("Started parsing base file");
    let base_tree =
        parsing::parse_string(base, &parser_configuration).map_err(ExecutionError::ParsingError)?;
    log::info!("Finished parsing base file");
    log::info!("Started parsing left file");
    let left_tree =
        parsing::parse_string(left, &parser_configuration).map_err(ExecutionError::ParsingError)?;
    log::info!("Finished parsing left file");
    log::info!("Started parsing right file");
    let right_tree = parsing::parse_string(right, &parser_configuration)
        .map_err(ExecutionError::ParsingError)?;
    log::info!("Finished parsing right file");

    let matching_configuration = matching_configuration::MatchingConfiguration::from(language);
    log::info!("Started calculation of matchings between left and base");
    let matchings_left_base =
        matching::calculate_matchings(&left_tree, &base_tree, &matching_configuration);
    log::info!("Finished calculation of matchings between left and base");
    log::info!("Started calculation of matchings between right and base");
    let matchings_right_base =
        matching::calculate_matchings(&right_tree, &base_tree, &matching_configuration);
    log::info!("Finished calculation of matchings between right and base");
    log::info!("Started calculation of matchings between left and right");
    let matchings_left_right =
        matching::calculate_matchings(&left_tree, &right_tree, &matching_configuration);
    log::info!("Finished calculation of matchings between left and right");

    log::info!("Starting merge of the trees");
    let result = merge::merge(
        &base_tree,
        &left_tree,
        &right_tree,
        &matchings_left_base,
        &matchings_right_base,
        &matchings_left_right,
    )
    .map_err(ExecutionError::MergeError)?;
    log::info!("Finished merge of the trees");

    match result.has_conflict() {
        true => Ok(ExecutionResult::WithConflicts(result.to_string())),
        false => Ok(ExecutionResult::WithoutConflicts(result.to_string())),
    }
}

pub fn run_diff_on_files(
    language: model::Language,
    left: &str,
    right: &str,
) -> Result<MatchingEntry, ExecutionError> {
    let parser_configuration = ParserConfiguration::from(language);

    log::info!("Started parsing left file");
    let left_tree_root =
        parsing::parse_string(left, &parser_configuration).map_err(ExecutionError::ParsingError)?;
    log::info!("Finished parsing left file");
    log::info!("Started parsing right file");
    let right_tree_root = parsing::parse_string(right, &parser_configuration)
        .map_err(ExecutionError::ParsingError)?;
    log::info!("Finished parsing right file");

    let matching_configuration = matching_configuration::MatchingConfiguration::from(language);
    log::info!("Started calculation of matchings between left and right");
    let matchings_left_right =
        matching::calculate_matchings(&left_tree_root, &right_tree_root, &matching_configuration);
    log::info!("Finished calculation of matchings between left and right");

    Ok(matchings_left_right
        .get_matching_entry(&left_tree_root, &right_tree_root)
        .unwrap_or_default()
        .to_owned())
}
