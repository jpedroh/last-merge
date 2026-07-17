use crate::ParsingHandlers;

mod tweak_source_file;

pub fn get_default_go_parsing_handlers() -> ParsingHandlers {
    ParsingHandlers::new(vec![tweak_source_file::tweak_source_file])
}
