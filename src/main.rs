use std::{fs, env, path::PathBuf};
use serde_json::Value;

fn main() {
    let structure_path = env::current_exe().unwrap().parent().unwrap().join("");
    let json_path = &structure_path.join("structure.json");
    let structure = match fs::read_to_string(&json_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Error while reading the json in this path: {:#?}", &structure_path);
            String::new()
        },
    };
    let parsed_structure: Value = serde_json::from_str(&structure).expect("Error while parsing the json");
    create_folders(&structure_path, &parsed_structure);
}

fn create_folders(base_path: &PathBuf, structure: &Value) {
    match structure {
        Value::Object(map) => {
            for (folder_name, substructure) in map {
                let folder_path = base_path.join(folder_name);
                fs::create_dir_all(&folder_path).expect("Error while creating folders");
                create_folders(&folder_path, substructure);
            }
        },
        _ => {},
    }
}