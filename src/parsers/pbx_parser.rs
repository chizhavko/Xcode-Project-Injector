#[derive(PartialEq, Eq, Hash, Debug)]
pub enum FileType {
    OBJC,
    SWIFT,
    FOLDER,
    NONE // Another type of file. Could be .js/.png/.framework etc
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct FolderRaw {
    isa: String,
    file_type: FileType,
    name: String,
    childs: Vec<String>,
}
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct FileRaw {
    isa: String,
    file_type: FileType,
    name: String,
}

pub fn parse_all_raw_folders(absolute_file_path: &str) -> Vec<FolderRaw> {
    let project_structure_list = parse_project_plist_file(absolute_file_path);
    parse_folders_structure(&project_structure_list)
}

pub fn parse_all_raw_files(absolute_file_path: &str) -> Vec<FileRaw> {
    let project_structure_list = parse_project_plist_file(absolute_file_path);
    parse_project_files_as_raw_string_list(&project_structure_list)
}

// Folders
fn parse_folders_structure(lines: &Vec<String>) -> Vec<FolderRaw> {
    let mut i = 0;
    let mut did_reach_section = false;
    let mut did_reach_group = false;
    let mut result = vec![];

    while i < lines.len() {
        let line = &lines[i];

        if line.contains("/* Begin PBXGroup section */") {
            did_reach_section = true;
            i += 1;
        }

        if line.contains("/* End PBXGroup section */") {
            return result;
        }

        if line.contains("*/ = {") {
            did_reach_group = true;
        }

        if did_reach_section && did_reach_group {
            if let Some(folder) = parse_single_group(lines, i) {
                result.push(folder);
            }
            did_reach_group = false;
        }

        i += 1;
    }

    result
}

fn parse_single_group(lines: &Vec<String>, start_index: usize) -> Option<FolderRaw> {
    let mut name = String::new();
    let mut isa = String::new();
    let mut childs = vec![];

    let mut i = start_index;
    
    while i < lines.len() {
        let line = lines[i].trim();

        if line.contains("}") {
            break;
        }

        if i == start_index {
            // parse ISA 
            let parsed_isa = string_slice_from_start(" /", &line);
            if parsed_isa.len() != line.len() {
                isa = parsed_isa;
            } else {
                return None;
            }
        }
        
        if line.contains("children") {
            // parse childrens
            childs = parse_group_childs(lines, i);
        }

        if line.contains("path") {
            // parse name
            let parsed_name = string_slice_from_pattern(" = ",";", &line);
            if parsed_name.len() != line.len() {
                name = parsed_name;
            } else {
                return None;
            }
        }

        i += 1;
    }

    if name.is_empty() || isa.is_empty() {
        return None;
    }

    Some(
        FolderRaw {
            isa: isa,
            file_type: FileType::FOLDER,
            name: name,
            childs: childs,
        }
    )

}

fn parse_group_childs(lines: &Vec<String>, start_index: usize) -> Vec<String> {
    let mut i = start_index + 1;
    let mut container: Vec<String> = vec![];

    loop {
        let line = &lines[i];

        if line.contains(")") {
            return container;
        } else {
            let item_id = string_slice_from_start(" /*", line);
            container.push(item_id.trim().to_string());
        }
        
        i += 1;
    }
}

// Files

fn parse_project_files_as_raw_string_list(lines: &Vec<String>) -> Vec<FileRaw> {
    let mut result = vec![];
    let mut did_reach_section = false;

    for i in 0..lines.len() {
        let line = &lines[i];

        if line.contains("/* Begin PBXBuildFile section */") {
            did_reach_section = true;
            continue;
        }

        if line.contains("/* End PBXBuildFile section */") {
            return result;
        }

        if did_reach_section {
            if let Some(file) = parse_single_file(line.trim()) {
                result.push(file);
            }
        }
    }
    result
}

fn parse_single_file(line: &str) -> Option<FileRaw> {
    let isa = string_slice_from_start(" /* ", line);
    let name = string_slice_from_pattern(" /* ", " in", line);
    let file_type = if name.contains(".m") || name.contains(".h") { 
        FileType::OBJC 
    } else if name.contains(".swift") {
        FileType::SWIFT 
    } else {
        FileType::NONE
    };

    if file_type == FileType::NONE {
        return None;
    }

    let line_len = line.len();

    if isa.len() == line_len || isa.is_empty() && 
        name.len() == line_len  || name.is_empty() {
        return None;
    }

    Some(
        FileRaw {
            isa: isa, 
            file_type: file_type,
            name: name
        }
    )
}

// Getting file

fn parse_project_plist_file(absolute_file_path: &str) -> Vec<String> {
    let result = std::fs::read_to_string(&absolute_file_path).expect("msg");
    let mut lines = vec![];
    for line in result.lines() {
        lines.push(line.to_string());
    }
    lines
}

/// UTILS

fn string_slice_from_pattern(from: &str, to: &str, line: &str) -> String {
    let start_bytes = line.find(from).unwrap_or(0) + from.len();
    let end_bytes = line.find(to).unwrap_or(line.len());
    line[start_bytes..end_bytes].to_string()
}
fn string_slice_from_start(to: &str, line: &str) -> String {
    let end_bytes = line.find(to).unwrap_or(line.len());
    line[0..end_bytes].to_string()
}