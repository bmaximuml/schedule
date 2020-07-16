use std::collections::HashMap;
use schedule::Task;

extern crate chrono;
use chrono::Local;

fn print_task(name: &String, task: &Task) {
    println!("| {0: <12} | {1: <8} | {2: <8} |", name, task.duration, if task.required {"required"} else {"optional"});
}

fn list_required(name: &String, task: &Task) {
    if task.required {
        print_task(name, task);
    }
}

fn list_optional(name: &String, task: &Task) {
    if !task.required {
        print_task(name, task);
    }
}

fn list_completed(name: &String, task: &Task) {
    // get highest timestamp => highest_timestamp
    let mut highest_timestamp: i64 = 0;
    for completed_task in &task.completed {
        if completed_task.timestamp > highest_timestamp {
            highest_timestamp = completed_task.timestamp;
        }
    }

    // get timestamp for midnight this morning => today_timestamp
    let today_timestamp: i64 = Local::today().and_hms(0, 0, 0).timestamp();

    // print task if highest_timestamp >= today_timestamp
    if highest_timestamp >= today_timestamp {
        print_task(name, task);
    }
}

fn list_incompleted(name: &String, task: &Task) {
    // get highest timestamp => highest_timestamp
    let mut highest_timestamp: i64 = 0;
    for completed_task in &task.completed {
        if completed_task.timestamp > highest_timestamp {
            highest_timestamp = completed_task.timestamp;
        }
    }

    // get timestamp for midnight this morning => today_timestamp
    let today_timestamp: i64 = Local::today().and_hms(0, 0, 0).timestamp();

    // print task if highest_timestamp < today_timestamp
    if highest_timestamp < today_timestamp {
        print_task(name, task);
    }
}

pub fn list_tasks(tasks: &HashMap<String, Task>, task_type: &str) {
    println!(
        "| {0: <12} | {1: <8} | {2: <8} |",
        "name", "duration", "type"
    );
    println!("task_type: {}", task_type);
    for (name, task) in tasks {
        match task_type {
            "required" => list_required(name, task),
            "optional" => list_optional(name, task),
            "completed" => list_completed(name, task),
            "incompleted" => list_incompleted(name, task),
            "all" | _ => print_task(name, task),
        }
    }
}