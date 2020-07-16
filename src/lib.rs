use std::fs;
use std::collections::HashMap;

extern crate yaml_rust;
use yaml_rust::{YamlLoader, Yaml};

#[derive(Debug)]
pub struct TaskCompleted {
    pub timestamp: i64,
    pub duration: i64,
}

#[derive(Debug)]
pub struct Task {
    pub duration: i64,
    pub required: bool, // true for required, false for optional
    pub completed: Vec<TaskCompleted>
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

pub fn load_tasks(file: &str) -> HashMap<String, Task> {
    let mut all_tasks: HashMap<String, Task> = HashMap::new();

    let yaml = &load_yaml(file)[0];

    let yaml = yaml.as_hash().unwrap().values().next().unwrap().as_hash().unwrap();
    for (name, task) in yaml {
        let mut task_completed: Vec<TaskCompleted> = Vec::new();
        let t_type = task.as_hash().unwrap().get(&YamlLoader::load_from_str("type").unwrap()[0]).unwrap().as_str().unwrap();
        let duration = task.as_hash().unwrap().get(&YamlLoader::load_from_str("duration").unwrap()[0]).unwrap().as_i64().unwrap();
        let completed = task.as_hash().unwrap().get(&YamlLoader::load_from_str("completed").unwrap()[0]);
        let name = name.as_str().unwrap();
        // println!("{}:", name);
        // println!("    type: {}", t_type);
        // println!("    duration: {}", duration.to_string());
        if let Some(completed) = completed {
            let completed = completed.as_vec().unwrap();
            // println!("    completed tasks:");
            for completed_task in completed {
                let timestamp: i64 = completed_task.as_hash().unwrap().get(&YamlLoader::load_from_str("timestamp").unwrap()[0]).unwrap().as_i64().unwrap();
                let duration: i64 = completed_task.as_hash().unwrap().get(&YamlLoader::load_from_str("duration").unwrap()[0]).unwrap().as_i64().unwrap();
                // println!("        timestamp: {:#?}", timestamp);
                // println!("        duration: {:#?}", duration);
                task_completed.push(
                    TaskCompleted {
                        timestamp,
                        duration,
                    }
                )
            }
        }
        all_tasks.insert(
            String::from(name),
            Task {
                duration,
                required: t_type == "required",
                completed: task_completed,
            },
        );
    }

    all_tasks
}