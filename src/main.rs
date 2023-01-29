
mod parsers;
mod utils;

fn main() {
    println!("File parsing started");
    let path = "/Users/m.chizhavko/Documents/Development/FileInjection/FileInjection.xcodeproj/project.pbxproj";
    parsers::project_parser::parse_files_tree(path);
    println!("Done");
}