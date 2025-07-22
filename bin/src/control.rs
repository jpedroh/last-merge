use std::{
    error::Error,
    fmt::{self, Display},
    time::Instant,
};

use merge::log_structures::{LogState, MergeChunk};
use matching::MatchingEntry;
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
    print_chunks: bool,
) -> Result<ExecutionResult, ExecutionError> {
    if base == left {
        log::info!("Early returning because base equals left");
        return Ok(ExecutionResult::WithoutConflicts(right.to_string()));
    }

    if base == right {
        log::info!("Early returning because base equals right");
        return Ok(ExecutionResult::WithoutConflicts(left.to_string()));
    }

    let parser_configuration = ParserConfiguration::from(language);
    
    let mut log_state = if print_chunks {
        Some(LogState::default())
    } else {
        None
    };

    let start = Instant::now();
    log::info!("Started parsing base file");
    let base_tree =
        parsing::parse_string(base, &parser_configuration).map_err(ExecutionError::ParsingError)?;
    log::info!("Finished parsing base file in {:?}", start.elapsed());

    let start = Instant::now();
    log::info!("Started parsing left file");
    let left_tree =
        parsing::parse_string(left, &parser_configuration).map_err(ExecutionError::ParsingError)?;
    log::info!("Finished parsing left file in {:?}", start.elapsed());

    let start = Instant::now();
    log::info!("Started parsing right file");
    let right_tree = parsing::parse_string(right, &parser_configuration)
        .map_err(ExecutionError::ParsingError)?;
    log::info!("Finished parsing right file in {:?}", start.elapsed());

    let start = Instant::now();
    log::info!("Started calculation of matchings between left and base");
    let matchings_left_base = matching::calculate_matchings(&left_tree, &base_tree);
    log::info!(
        "Finished calculation of matchings between left and base in {:?}",
        start.elapsed()
    );

    let start = Instant::now();
    log::info!("Started calculation of matchings between right and base");
    let matchings_right_base = matching::calculate_matchings(&right_tree, &base_tree);
    log::info!(
        "Finished calculation of matchings between right and base in {:?}",
        start.elapsed()
    );

    let start = Instant::now();
    log::info!("Started calculation of matchings between left and right");
    let matchings_left_right = matching::calculate_matchings(&left_tree, &right_tree);
    log::info!(
        "Finished calculation of matchings between left and right in {:?}",
        start.elapsed()
    );

    let start = Instant::now();
    log::info!("Starting merge of the trees");
    let result = merge::merge(
        &base_tree,
        &left_tree,
        &right_tree,
        &matchings_left_base,
        &matchings_right_base,
        &matchings_left_right,
        &mut log_state,
    )
    .map_err(ExecutionError::MergeError)?;
    log::info!("Finished merge of the trees in {:?}", start.elapsed());

    if let Some(final_log_state) = log_state {
        let format_node_list_detailed = |nodes: &Vec<&model::CSTNode>| -> String {
            if nodes.is_empty() {
                return "-".to_string();
            }
            let first_node = nodes.first().unwrap();
            let last_node = nodes.last().unwrap();
            
            let start_line = first_node.start_position().row + 1;
            let end_line = last_node.end_position().row + 1;
            
            let range = if start_line == end_line {
                format!("(L{})", start_line)
            } else {
                format!("(L{}-L{})", start_line, end_line)
            };
            
            const MAX_NODES_TO_SHOW: usize = 3;
            const MAX_CONTENT_LEN: usize = 25;
            let descriptions: Vec<String> = nodes.iter().map(|n| {
                let mut content = n.contents().replace(['\n', '\r'], " ").trim().to_string();
                if content.len() > MAX_CONTENT_LEN {
                    content.truncate(MAX_CONTENT_LEN - 3);
                    content.push_str("...");
                }
                format!("{}: '{}'", n.kind(), content)
            }).take(MAX_NODES_TO_SHOW).collect();
            let mut summary = format!("[{}]", descriptions.join(", "));
            if nodes.len() > MAX_NODES_TO_SHOW {
                summary.push_str("...");
            }
            
            format!("{} nós {} {}", nodes.len(), range, summary)
        };

        println!("\n--- LASTMERGE CHUNK DEBUG LOG ---");
        println!("===========================================================");
        for (i, chunk) in final_log_state.log.iter().enumerate() {
            match chunk {
                MergeChunk::Stable(data) => {
                    println!("-- stable chunk #{} --", i + 1);
                    println!("    Left (L):  {}", format_node_list_detailed(&data.left_nodes));
                    println!("    Base (B):  {}", format_node_list_detailed(&data.base_nodes));
                    println!("    Right (R): {}", format_node_list_detailed(&data.right_nodes));
                }
                MergeChunk::Unstable(data) => {
                    println!("-- unstable chunk #{} --", i + 1);
                    println!("    Left (L):  {}", format_node_list_detailed(&data.left_nodes));
                    println!("    Base (B):  {}", format_node_list_detailed(&data.base_nodes));
                    println!("    Right (R): {}", format_node_list_detailed(&data.right_nodes));
                }
            }
            println!("-----------------------------------------------------------"); 
        }
        println!("--- END LASTMERGE CHUNK DEBUG LOG ---\n");
    }

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

    log::info!("Left tree size: {}", left_tree_root.get_tree_size());
    log::info!("Right tree size: {}", right_tree_root.get_tree_size());

    log::info!("Started calculation of matchings between left and right");
    let matchings_left_right = matching::calculate_matchings(&left_tree_root, &right_tree_root);
    log::info!("Finished calculation of matchings between left and right");

    Ok(matchings_left_right
        .get_matching_entry(&left_tree_root, &right_tree_root)
        .unwrap_or_default()
        .to_owned())
}
