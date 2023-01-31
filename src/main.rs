
mod parsers;
mod utils;

use std::{fs::{self, ReadDir}, collections::{HashSet, HashMap}};

enum FileExtension {
    M,
    Header,
    Unknown,
}

impl FileExtension {
    fn new(name: &str) -> FileExtension {
        if name.contains(".m") {
            FileExtension::M
        } else if name.contains(".h") {
            FileExtension::Header
        } else {
            FileExtension::Unknown
        } 
    }
}

fn main() {
    println!("File parsing started");
    // let path = "/Users/m.chizhavko/Documents/Development/FileInjection/FileInjection.xcodeproj/project.pbxproj";
    // let path = "/Users/m.chizhavko/Documents/Development/vkclient/VKClient/VKClient.xcodeproj/project.pbxproj";
    // parsers::project_parser::parse_files_tree(path);
    // println!("Done");

    let mut all_files: HashMap<String, String> = HashMap::new();
    // let paths = fs::read_dir("/Users/m.chizhavko/Documents/Development/FileInjection").unwrap();
    let paths = fs::read_dir("/Users/m.chizhavko/Documents/Development/vkclient/VKClient").unwrap();
    collect_all_files(paths, &mut all_files);

    let mut dependencies: HashSet<String> = HashSet::new();

    println!("============== ALL_FILES ============");
    println!();
    for file in &all_files {
        println!("File: name: {}, path: {}", file.0, file.1);
    }

    println!("============== DEPENDENCY_LIST ============");
    println!();

    dfs("VKMNavDelegate.h".to_string(), &all_files, &mut dependencies);

    for dependency in &dependencies {
        println!("Dep file: {}", dependency);
    }

    println!("TOTAL NUMBER OF FILES: {}", all_files.len());
    println!("TOTAL DEPENDENCY FILES: {}", dependencies.len());
    // wrong: VKMNavDelegate.h
}

// Fetch all files 

fn collect_all_files(dir: ReadDir, files: &mut HashMap<String, String>) {
    for path in dir {
        if let Ok(dir) = path {
            if dir.path().is_dir() {
                if let Ok(dir) = fs::read_dir(dir.path()) {
                    collect_all_files(dir, files);
                }
            } else {
                let file_name = dir.file_name().into_string().unwrap();
                let path_str = dir.path().as_os_str().to_os_string().into_string().unwrap();
                files.insert(file_name, path_str);
            }
        }
    }
}

// Find dependencies 

fn dfs(file_name: String, all_files: &HashMap<String, String>, dependencies: &mut HashSet<String>) {
    let file_extension = FileExtension::new(&file_name);
    let mut imports: HashSet<String> = HashSet::new();

    match file_extension {
        FileExtension::Header => {
            let h_file = file_name;
            let m_file = h_file.replace(".h", ".m");

            if !dependencies.contains(&h_file) {
                let imports_from_h_file = imports_from_file(&h_file, all_files);
                dependencies.insert(h_file);
                imports.extend(imports_from_h_file);
            }

            if !dependencies.contains(&m_file) {
                let imports_from_m_file = imports_from_file(&m_file, all_files);
                dependencies.insert(m_file);
                imports.extend(imports_from_m_file);
            }
        },
        FileExtension::M => {
            let m_file = file_name;
            let h_file = m_file.replace(".m", ".h");

            if !dependencies.contains(&h_file) {
                let imports_from_h_file = imports_from_file(&h_file, all_files);
                dependencies.insert(h_file);
                imports.extend(imports_from_h_file);
            }

            if !dependencies.contains(&m_file) {
                let imports_from_m_file = imports_from_file(&m_file, all_files);
                dependencies.insert(m_file);
                imports.extend(imports_from_m_file);
            }
        },
        FileExtension::Unknown => {
            println!("Wrong file type: {}", file_name);
        }
    }
    
    for import in imports {
        dfs(import, all_files, dependencies)
    }
}

fn imports_from_file(file_name: &str, all_files: &HashMap<String, String>) -> HashSet<String> {
    let mut did_reach_import_section = false;
    let mut result: HashSet<String> = HashSet::new();
    println!("Start file with dep {}", file_name);

    if let Some(path) = all_files.get(file_name) {
        if let Ok(file) = fs::read_to_string(path) {
            for line in file.lines() {
                if line.is_empty() {
                    continue;
                }

                let is_import_line = line.contains("#import");
                
                if is_import_line && !did_reach_import_section {
                    did_reach_import_section = true;
                }
    
                if did_reach_import_section && !is_import_line {
                    return result;
                }
    
                if is_import_line {
                    let dependency_file = utils::string_utils::string_slice_from_pattern("\"", "\"", line);
                    if dependency_file != "" {
                        result.insert(dependency_file.to_string());
                    } else {
                        println!("Wrong file in DFS: {} from line: {}", dependency_file, line);
                    }
                }
            }
        }
    } else {
        println!("Can't find file {}", file_name);
    }

    result
}