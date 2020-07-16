use std::collections::HashMap;

extern crate clap;
use clap::{App, load_yaml};

extern crate chrono;
use chrono::{Datelike, Timelike, Local, DateTime};

use schedule::{Task, load_tasks};

mod list;
use crate::list::list_tasks;

fn main() {
    // Store current time
    let now: DateTime<Local> = Local::now();

    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Gets a value for tasks file if supplied by user, or defaults to "tasks.yml"
    let tasks_file: &str = matches.value_of("tasks").unwrap_or("tasks.yml");
    let tasks: HashMap<String, Task> = load_tasks(tasks_file);

    if matches.occurrences_of("v") > 0 {
        println!("Value for tasks file: {}", tasks_file);
        println!("{:#?}", tasks);
    }


    // println!("{:?}", tasks["tasks"]["reading"]["type"].as_str().unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'schedule -v -v -v' or 'schedule -vvv' vs 'schedule -v')
    // match matches.occurrences_of("v") {
    //     0 => println!("No verbose info"),
    //     1 => println!("Some verbose info"),
    //     2 => println!("Tons of verbose info"),
    //     3 | _ => println!("Don't be crazy"),
    // }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }

        println!("Running tests...");
    }

    if let Some(matches) = matches.subcommand_matches("list") {
        let matches = matches.value_of("TYPE").unwrap_or("all");
        // List different amounts of below data based on verbosity
        // When listing tasks, include number of completions | days since started | last completed | default duration | average duration
        list_tasks(&tasks, &matches);
        // match matches {
        //     "required" => println!("Listing required tasks..."),
        //     "optional" => println!("Listing optional tasks..."),
        //     "incompleted" => println!("Listing incompleted tasks..."),
        //     "completed" => println!("Listing completed tasks..."),
        //     "all" | _ => println!("Listing all tasks..."),
        // }
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let task = matches.value_of("TASK").unwrap();
        // When listing tasks, include number of completions | days since started | last completed
        println!("Adding task with name: {}", task);
        let duration = matches.value_of("duration").unwrap_or("60").parse::<i32>();
        let duration = match duration {
            Ok(num) => num,
            Err(error) => panic!("duration must be an integer: {:?}", error),
        };
        println!("    Task duration: {} minutes", duration.to_string());
        if matches.is_present("required") {
            println!("    Task is required.");
        } else {
            println!("    Task is optional.");
        }
    } else if let Some(matches) = matches.subcommand_matches("complete") {
        let task = matches.value_of("TASK").unwrap();
        println!("{} was completed at {:02}:{:02}:{:02} on {} {}/{}/{}.",
            task,
            now.hour(),
            now.minute(),
            now.second(),
            now.weekday(),
            now.day(),
            now.month(),
            now.year());
        // Need to change 60 to the task default
        let duration = matches.value_of("duration").unwrap_or("60").parse::<i32>();
        let duration = match duration {
            Ok(num) => num,
            Err(error) => panic!("duration must be an integer - {:?}", error),
        };
        println!("    Task ran for {} minutes.", duration.to_string());
    } else if let Some(matches) = matches.subcommand_matches("choose") {
        let matches = matches.value_of("TYPE").unwrap_or("any");
        // When listing tasks, include number of completions | days since started | last completed
        match matches {
            "required" => println!("Choosing a required task..."),
            "optional" => println!("Choosing an optional task..."),
            "all" | _ => println!("Choosing a task..."),
        }
    }
}