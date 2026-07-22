use std::process::ExitCode;

use clap::Parser;
use cli_args::{CliArgs, CliSubCommands, DiffCliArgs, MergeCliArgs};
use tracing_chrome::ChromeLayerBuilder;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod cli_args;
mod cli_exit_codes;
mod control;
mod language;

fn main() -> std::process::ExitCode {
    let args = CliArgs::parse();

    let (chrome_layer, _guard) = ChromeLayerBuilder::new().include_args(true).build();
    tracing_subscriber::registry()
        // .with(fmt::layer())
        .with(chrome_layer)
        .with(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting last Merge tool execution");
    tracing::debug!("Parsed arguments: {:?}", args);

    let result = match args.command {
        CliSubCommands::Diff(args) => run_diff(args),
        CliSubCommands::Merge(args) => run_merge(args),
    };

    match result {
        Ok(exit_code) => ExitCode::from(exit_code),
        Err(exit_code) => ExitCode::from(exit_code),
    }
}

fn run_merge(args: MergeCliArgs) -> Result<u8, u8> {
    let base = std::fs::read_to_string(&args.base_path).map_err(|error| {
        log::error!("Error while reading base file: {}", error);
        cli_exit_codes::READING_FILE_ERROR
    })?;
    let left = std::fs::read_to_string(&args.left_path).map_err(|error| {
        log::error!("Error while reading left file: {}", error);
        cli_exit_codes::READING_FILE_ERROR
    })?;
    let right = std::fs::read_to_string(&args.right_path).map_err(|error| {
        log::error!("Error while reading right file: {}", error);
        cli_exit_codes::READING_FILE_ERROR
    })?;

    let language = match args.language {
        Some(language) => language::get_language_from_name(&language),
        None => language::get_language_by_file_path(&args.base_path),
    }
    .map_err(|error| {
        log::error!("Error while retrieving language configuration: {}", error);
        cli_exit_codes::INVALID_LANGUAGE_ERROR
    })?;

    let result =
        control::run_tool_on_merge_scenario(language, &base, &left, &right, args.print_chunks)
            .map_err(|error| {
                log::error!("Error while running tool: {}", error);
                cli_exit_codes::INTERNAL_EXECUTION_ERROR
            })?;

    std::fs::write(args.merge_path, result.to_string()).map_err(|error| {
        log::error!("Error while writing output file: {}", error);
        cli_exit_codes::WRITING_FILE_ERROR
    })?;

    match result {
        control::ExecutionResult::WithConflicts(_) => {
            log::info!("Execution finished with conflicts");
            Ok(cli_exit_codes::SUCCESS_WITH_CONFLICTS)
        }
        control::ExecutionResult::WithoutConflicts(_) => {
            log::info!("Execution finished without conflicts");
            Ok(cli_exit_codes::SUCCESS_WITHOUT_CONFLICTS)
        }
    }
}

fn run_diff(args: DiffCliArgs) -> Result<u8, u8> {
    let left = std::fs::read_to_string(&args.left_path).map_err(|error| {
        log::error!("Error while reading left file: {}", error);
        cli_exit_codes::READING_FILE_ERROR
    })?;
    let right = std::fs::read_to_string(&args.right_path).map_err(|error| {
        log::error!("Error while reading right file: {}", error);
        cli_exit_codes::READING_FILE_ERROR
    })?;

    let language = match args.language {
        Some(language) => language::get_language_from_name(&language),
        None => language::get_language_by_file_path(&args.left_path),
    }
    .map_err(|error| {
        log::error!("Error while retrieving language configuration: {}", error);
        cli_exit_codes::INVALID_LANGUAGE_ERROR
    })?;

    let result = control::run_diff_on_files(language, &left, &right).map_err(|error| {
        log::error!("Error while running tool: {}", error);
        cli_exit_codes::INTERNAL_EXECUTION_ERROR
    })?;

    log::info!("{:?}", result);
    match result.is_perfect_match {
        true => {
            log::info!("Both files are equivalent");
            Ok(cli_exit_codes::SUCCESS_FILES_FULLY_MATCH)
        }
        false => {
            log::info!("Both files are different");
            Ok(cli_exit_codes::SUCCESS_FILES_DO_NOT_FULLY_MATCH)
        }
    }
}
