use crate::parsers::pbx_parser;

pub fn parse_files_tree(absolute_path: &str) {
    let files = pbx_parser::parse_all_raw_files(absolute_path);
    let folders = pbx_parser::parse_all_raw_folders(absolute_path);

    println!("Files amount: {}", files.len());
    println!("Folders amount: {}", folders.len());
}
