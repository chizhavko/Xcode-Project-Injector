
use crate::parsers::reference_builder;

pub fn parse_files_tree(absolute_path: &str) {
    reference_builder::parse_files_tree(absolute_path, "VKClient");
}
