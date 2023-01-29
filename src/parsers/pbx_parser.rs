use crate::utils::string_utils::*;

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
    let project_structure_list = parse_project_file(absolute_file_path);
    parse_folders_raw_list(&project_structure_list)
}

pub fn parse_all_raw_files(absolute_file_path: &str) -> Vec<FileRaw> {
    let project_structure_list = parse_project_file(absolute_file_path);
    parse_files_raw_list(&project_structure_list)
}

// Folders
fn parse_folders_raw_list(lines: &Vec<String>) -> Vec<FolderRaw> {
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

fn combine_group_into_single_line(lines: &Vec<String>, start_index: usize) -> String {
    let mut combined_line = String::new();
    let mut i = start_index;
    
    while i < lines.len() {
        let line = lines[i].trim();
        combined_line += line;

        if line.contains("};") {
            return combined_line;
        }

        i += 1;
    }

    combined_line
}

fn parse_single_group(lines: &Vec<String>, start_index: usize) -> Option<FolderRaw> {
    let group_string = combine_group_into_single_line(lines, start_index);
    let isa = string_slice_from_start(" = {", &group_string);
    let name = string_slice_from_pattern("path = ", ";", &group_string);
    let childs_string = string_slice_from_pattern("children = (", ");", &group_string);
    let childs = childs_string
        .split(",")
        .map(|item| string_slice_from_start(" /* ", item).to_string())
        .filter(|item| !item.is_empty())
        .collect::<Vec<String>>();

    let line_len = group_string.len();

    if isa.len() == line_len || isa.is_empty() && 
        name.len() == line_len  || name.is_empty() {
        return None;
    }

    Some(
        FolderRaw {
            isa: isa.to_string(),
            name: name.to_string(),
            file_type: FileType::FOLDER,
            childs:childs
        }
    )
}

// Files
fn parse_files_raw_list(lines: &Vec<String>) -> Vec<FileRaw> {
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
            isa: isa.to_string(), 
            file_type: file_type,
            name: name.to_string()
        }
    )
}

// Getting file

fn parse_project_file(absolute_file_path: &str) -> Vec<String> {
    let result = std::fs::read_to_string(&absolute_file_path).expect("msg");
    let mut lines = vec![];
    for line in result.lines() {
        lines.push(line.to_string());
    }
    lines
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine_group_into_single_line() {
        let lines: Vec<String> = group_lines();
        let expected_string = "7BD23B3E2981AE9F00C0ADAE = {isa = PBXGroup;children = (7BD23B4A2981AE9F00C0ADAE /* FileInjection */,7BD23B492981AE9F00C0ADAE /* Products */,);path = Items;sourceTree = \"<group>\";};";
        let result = combine_group_into_single_line(&lines, 0);

        assert_eq!(result, expected_string);
    }

    #[test]
    fn test_parse_single_group() {
        let lines = group_lines();
        let result = parse_single_group(&lines, 0);
        assert_eq!(
            result, 
            Some(
                FolderRaw {
                    isa: "7BD23B3E2981AE9F00C0ADAE".to_string(),
                    file_type: FileType::FOLDER,
                    name: "Items".to_string(),
                    childs: vec!["7BD23B4A2981AE9F00C0ADAE".to_string(), "7BD23B492981AE9F00C0ADAE".to_string()]
                }
            )
        )
    }

    #[test]
    fn test_parse_single_file() {
        {
            let line = "7B3516B22984049B00348D3A /* ItemObject.m in Sources */ = {isa = PBXBuildFile; fileRef = 7B3516B02984049B00348D3A /* ItemObject.m */; };";
            let result = parse_single_file(line);
            assert_eq!(
                result,
                Some(
                    FileRaw { 
                        isa: "7B3516B22984049B00348D3A".to_string(),
                        file_type: FileType::OBJC, 
                        name: "ItemObject.m".to_string()
                    }
                )
            )
        }
        { 
            let line = "7B3516B22984049B00348D3A /* ItemObject.framework in Sources */ = {isa = PBXBuildFile; fileRef = 7B3516B02984049B00348D3A /* ItemObject.m */; };";
            let result = parse_single_file(line);
            assert_eq!(
                result,
                None
            )
        }
    }

    #[test]
    fn test_parse_folders_structure() {
        let path = "/Users/m.chizhavko/Documents/Development/xcode_project_extractor/test_data/file_structure.xml";
        let lines = parse_project_file(path);
        let folders = parse_folders_raw_list(&lines);
        assert_eq!(folders.len(), 3);
    }

    #[test]
    fn test_parse_project_files_as_raw_string_list() {
        let path = "/Users/m.chizhavko/Documents/Development/xcode_project_extractor/test_data/file_structure.xml";
        let lines = parse_project_file(path);
        let files = parse_files_raw_list(&lines);
        assert_eq!(files.len(), 7);
    }

    #[test]
    fn test_parse_project_file() {
        let path = "/Users/m.chizhavko/Documents/Development/xcode_project_extractor/test_data/file_structure.xml";
        let result = parse_project_file(path);
        assert_eq!(result.len(), 385);
    }

    /*
        7BD23B3E2981AE9F00C0ADAE = {
			isa = PBXGroup;
			children = (
				7BD23B4A2981AE9F00C0ADAE /* FileInjection */,
				7BD23B492981AE9F00C0ADAE /* Products */,
			);
            path = Items;
			sourceTree = "<group>";
		};
    */
    fn group_lines() -> Vec<String> {
        vec!["7BD23B3E2981AE9F00C0ADAE = {".to_string(), "  isa = PBXGroup; ".to_string(), "children = (  ".to_string(), "7BD23B4A2981AE9F00C0ADAE /* FileInjection */, ".to_string(), "7BD23B492981AE9F00C0ADAE /* Products */,".to_string(), ");".to_string(), "path = Items;".to_string(), "sourceTree = \"<group>\";".to_string(), "};".to_string()]
    }
}