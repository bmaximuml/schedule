use std::collections::HashMap;
use schedule::Task;

extern crate chrono;
use chrono::Local;

fn print_task(name: &String, task: &Task) {
    println!(
        "| {0: <12} | {1: <11} | {2: <8} | {3: <8} | {4: <11} |", name, task.status, task.duration, 
        if task.required {"required"} else {"optional"}, task.occurrences.len()
    );
}

fn if_task_required(name: &String, task: &Task) {
    // Print task if task is required
    if task.required {
        print_task(name, task);
    }
}

fn if_task_optional(name: &String, task: &Task) {
    // Print task if task is optional
    if !task.required {
        print_task(name, task);
    }
}

fn if_task_started(name: &String, task: &Task) {
    // Print task if task is started
    if task.occurrences.len() > 0 {
        print_task(name, task);
    }
}

fn if_task_unstarted(name: &String, task: &Task) {
    // Print task if task is unstarted
    if task.occurrences.len() == 0 {
        print_task(name, task);
    }
}

pub fn list_tasks(tasks: &HashMap<String, Task>, task_type: &str) {
    println!(
        "| {0: <12} | {1: <11} | {2: <8} | {3: <8} | {4: <11} |",
        "name", "status", "duration", "type", "occurrences"
    );
    println!(
        "| {0: <12} | {1: <11} | {2: <8} | {3: <8} | {4: <11} |",
        "====", "======", "========", "====", "==========="
    );
    // println!("task_type: {}", task_type);
    for (name, task) in tasks {
        match task_type {
            "required" => if_task_required(name, task),
            "optional" => if_task_optional(name, task),
            "started" => if_task_started(name, task),
            "unstarted" => if_task_unstarted(name, task),
            "all" | _ => print_task(name, task),
        }
    }
}