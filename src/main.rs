// #[macro_use]
extern crate clap;
use clap::{App, load_yaml};

extern crate chrono;
use chrono::{Datelike, Timelike, Local};

fn main() {
    // Store current time
    let now = Local::now();

    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    // let config = matches.value_of("config").unwrap_or("default.conf");
    // println!("Value for config: {}", config);

    // Gets a value for tasks file if supplied by user, or defaults to "tasks.yml"
    let tasks = matches.value_of("tasks").unwrap_or("tasks.yml");
    println!("Value for tasks file: {}", tasks);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't)
    // required we could have used an 'if let' to conditionally get the value ()
    // println!("Using input file {}", matches.value_of("INPUT").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v')
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

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
        match matches {
            "required" => println!("Listing required tasks..."),
            "optional" => println!("Listing optional tasks..."),
            "incompleted" => println!("Listing incompleted tasks..."),
            "completed" => println!("Listing completed tasks..."),
            "all" | _ => println!("Listing all tasks..."),
        }
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

    // let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments {}", err);
    //     process::exit(1);
    // });

    // if let Err(e) = schedule::run(config) {
    //     println!("Application error: {}", e);

    //     process::exit(1);
    // }
}