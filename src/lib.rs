use std::fs;
use std::collections::HashMap;
// use std::collections::BTreeMap;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskOccurrence {
    pub timestamp: i64,
    pub duration: i64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub duration: i64,
    pub required: bool, // true for required, false for optional
    pub status: String,
    pub occurrences: Vec<TaskOccurrence>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskYaml {
    pub tasks: HashMap<String, Task>,
}

fn load_yaml(file: &str) -> Vec<Yaml> {
    let yaml_str = match fs::read_to_string(file) {
        Ok(data) => data,
        Err(_) => panic!("Unable to read file: {}", &file),
    };

    match YamlLoader::load_from_str(&yaml_str) {
        Ok(data) => data,
        Err(_) => panic!("Invalid yaml file: {}", file),
    }
}

// pub fn load_tasks(file: &str) -> HashMap<String, Task> {
//     let mut all_tasks: HashMap<String, Task> = HashMap::new();

//     let yaml = &load_yaml(file)[0];

//     let yaml = yaml.as_hash().unwrap().values().next().unwrap().as_hash().unwrap();
//     for (name, task) in yaml {
//         let mut task_occurrence: Vec<TaskOccurrence> = Vec::new();
//         let t_type = task.as_hash().unwrap().get(&YamlLoader::load_from_str("type").unwrap()[0]).unwrap().as_str().unwrap();
//         let duration = task.as_hash().unwrap().get(&YamlLoader::load_from_str("duration").unwrap()[0]).unwrap().as_i64().unwrap();
//         let occurrence = task.as_hash().unwrap().get(&YamlLoader::load_from_str("occurrence").unwrap()[0]);
//         let status = task.as_hash().unwrap().get(&YamlLoader::load_from_str("status").unwrap()[0]);
//         let name = name.as_str().unwrap();
//         // println!("{}:", name);
//         // println!("    type: {}", t_type);
//         // println!("    duration: {}", duration.to_string());
//         if let Some(occurrence) = occurrence {
//             let occurrence = occurrence.as_vec().unwrap();
//             // println!("    occurrence tasks:");
//             for occurrence_task in occurrence {
//                 let timestamp: i64 = occurrence_task.as_hash().unwrap().get(&YamlLoader::load_from_str("timestamp").unwrap()[0]).unwrap().as_i64().unwrap();
//                 let duration: i64 = occurrence_task.as_hash().unwrap().get(&YamlLoader::load_from_str("duration").unwrap()[0]).unwrap().as_i64().unwrap();
//                 // println!("        timestamp: {:#?}", timestamp);
//                 // println!("        duration: {:#?}", duration);
//                 task_occurrence.push(
//                     TaskOccurrence {
//                         timestamp,
//                         duration,
//                     }
//                 )
//             }
//         }
//         all_tasks.insert(
//             String::from(name),
//             Task {
//                 duration,
//                 status,
//                 required: t_type == "required",
//                 occurrences: task_occurrence,
//             },
//         );
//     }

//     all_tasks
// }

pub fn load_yaml_serde(file: &str) -> TaskYaml {
    let yaml_str = match fs::read_to_string(file) {
        Ok(data) => data,
        Err(error) => panic!("Unable to read file: {} : {:?}", &file, error),
    };

    let deserialized_map: TaskYaml = match serde_yaml::from_str(&yaml_str) {
        Ok(data) => data,
        Err(error) => panic!("Unable to serde_yaml: {} : {:?}", &yaml_str, error)
    };
    // println!("{:#?}", deserialized_map);
    deserialized_map
}