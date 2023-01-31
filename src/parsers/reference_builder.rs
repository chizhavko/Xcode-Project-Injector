use std::collections::{HashMap, HashSet};
use super::pbx_parser::{self, FileRaw, FolderRaw};
use core::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
struct File {
    isa: String,
    name: String,
    path: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Folder {
    isa: String, 
    name: String, 
    path: String,
    files: HashSet<File>,
    subfolders: HashSet<Folder>,
}

impl Hash for Folder {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.isa.hash(state);
    }
}


impl Hash for File {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.isa.hash(state);
    }
}

pub fn parse_files_tree(absolute_path: &str, root_folder_name: &str) {
    let files = pbx_parser::parse_all_raw_files(absolute_path);
    let folders = pbx_parser::parse_all_raw_folders(absolute_path);
    if let Some(root_folder_isa) = find_root_folder_isa(root_folder_name, &folders) {
        if let Some(root_raw_folder) = folders.get(&root_folder_isa) {
            let mut root_folder = Folder {
                isa: root_folder_isa,
                name: root_raw_folder.name.clone(),
                path: root_raw_folder.name.clone(), 
                files: HashSet::new(),
                subfolders: HashSet::new()
            };

            let root_folder = build_project_hierarhy_recursively(&files, &folders, &mut root_folder);

            print_hierarchy(root_folder);

        } else {
            println!("NO ROOT RAW FOLDER");
        }
    } else {
        println!("NO ROOT FOLDER ISA");
    }
}

fn print_hierarchy(folder: &Folder) { 
    println!("-Folder {}", folder.name);

    for file in &folder.files {
        println!("File: {}", file.name);
    }

    for folder in &folder.subfolders {
        println!("Folder: {}", folder.name);
    }

    for folder in &folder.subfolders {
        print_hierarchy(folder);
    }
}

fn find_root_folder_isa(root_folder_name: &str, folders: &HashMap<String, FolderRaw>) -> Option<String> {
    for (key, value) in folders {
        if value.name == root_folder_name {
            return Some(key.clone());
        }
    }

    None
}

fn build_project_hierarhy_recursively<'a>(
    files: &HashMap<String, FileRaw>, 
    folders: &HashMap<String, FolderRaw>, 
    current_folder: &'a mut Folder,
) -> &'a Folder {
    if let Some(raw_folder) = folders.get(&current_folder.isa) {
        for child_isa in &raw_folder.childs {
            if let Some(subfolder_ref) = folders.get(child_isa) {
                let path = current_folder.path.clone() + "/" + &subfolder_ref.name.clone();
                let mut subfolder = Folder {
                    isa: subfolder_ref.isa.to_string(),
                    name: subfolder_ref.name.to_string(),
                    path: path.clone(),
                    files: HashSet::new(),
                    subfolders: HashSet::new()
                };
                build_project_hierarhy_recursively(files, folders, &mut subfolder);
                current_folder.subfolders.insert(subfolder);

            } else if let Some(file_ref) = files.get(child_isa) {
                let path = current_folder.path.clone() + "/" + &file_ref.name.clone();  
                let file = File {
                    isa: file_ref.isa.to_string(),
                    name: file_ref.name.to_string(),
                    path: path,
                };

                current_folder.files.insert(file);
            } else {
                println!("Missing folder with isa {}", child_isa);
            }
        }

    }

    current_folder
}