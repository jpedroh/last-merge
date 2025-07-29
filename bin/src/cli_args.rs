use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliSubCommands,

    /// The minimum log level to be displayed in output
    #[arg(long, global=true, default_value_t = log::LevelFilter::Info)]
    pub log_level: log::LevelFilter,
}

#[derive(Subcommand, Debug)]
pub enum CliSubCommands {
    #[command(about = "Runs only the diffing step on both input files")]
    Diff(DiffCliArgs),
    #[command(about = "Runs structured merge on the scenario provided")]
    Merge(MergeCliArgs),
}

#[derive(Parser, Debug)]
pub struct DiffCliArgs {
    /// Path to file in left revision
    #[arg(short, long)]
    pub(crate) left_path: std::path::PathBuf,

    /// Path to file in right revision
    #[arg(short, long)]
    pub(crate) right_path: std::path::PathBuf,

    /// The language that the files being diffed are written in.
    /// If not provided the language will try to be inferred by the extension.
    #[arg(long)]
    pub(crate) language: Option<String>,
}

#[derive(Parser, Debug)]
pub struct MergeCliArgs {
    /// Path to file in base revision
    #[arg(short, long)]
    pub(crate) base_path: std::path::PathBuf,

    /// Path to file in left revision
    #[arg(short, long)]
    pub(crate) left_path: std::path::PathBuf,

    /// Path to file in right revision
    #[arg(short, long)]
    pub(crate) right_path: std::path::PathBuf,

    /// Path where the merged file should be written
    #[arg(short, long)]
    pub(crate) merge_path: std::path::PathBuf,

    /// The language that the files being diffed are written in.
    /// If not provided the language will try to be inferred by the extension.
    #[arg(long)]
    pub(crate) language: Option<String>,

    /// Prints a detailed log of the merge process in the CST nodes
    #[arg(long)]
    pub print_chunks: bool,
}
