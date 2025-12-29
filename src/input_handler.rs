use std::io;
use std::io::Write;
use std::str::SplitWhitespace;
use crate::to_do_list::ToDoList;
use tokio::time::{sleep, Duration};

pub async fn input_handler() {
    // Declare and initialize
    let mut to_do_list = ToDoList::new();
    let mut buffer= String::new();

    // Load the file
    to_do_list.load();

    // Loop infinitely
    loop {
        // Clear the buffer
        buffer.clear();

        // Add a line to signify input
        print!(" >>> ");

        // Flush
        io::stdout().flush().unwrap();

        // Fetch the input to the buffer
        let _ = io::stdin().read_line(&mut buffer);

        // Split the input into different parts (command and arguments)
        let mut parts = buffer.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        let args = parts;

        // Check the command
        match command {
            "add" => add_command(args, &mut to_do_list),                // Add command
            "delete" => delete_command(args, &mut to_do_list),          // Delete command
            "remind" => remind_command(args, &mut to_do_list).await,    // Remind command
            "complete" => complete_command(args, &mut to_do_list),      // Complete command
            "list" => list_command(&to_do_list),                        // List command
            "help" => help_command(),                                   // Help command
            "save" => save_command(&to_do_list),                        // Save command
            "quit" => { save_command(&to_do_list); break },             // Quit command
            _ => {
                println!("[INVALID INPUT]: {command}. Type 'help' to see available commands.");
            }
        }
    }
}

fn add_command(args: SplitWhitespace, to_do_list: &mut ToDoList) {
    // Add the arguments to satisfy the title parameter
    let title = args.collect::<Vec<&str>>().join(" ");

    // Perform the operation
    println!("{}", to_do_list.add(title));
}

fn delete_command(mut args: SplitWhitespace, to_do_list: &mut ToDoList) {
    // Unwrap the id parameter
    let id_param = args.next().unwrap_or("").parse::<i32>().unwrap_or(-1);

    // Perform the operation and check for the result
    match to_do_list.delete(id_param) {
        Ok(_) => println!("[DELETE]: [{id_param}] deleted."),
        Err(_) => println!("[ERROR]: [{id_param}] not found.")
    }
}

async fn remind_command (mut args: SplitWhitespace<'_>, to_do_list: &ToDoList) {
    // Split the arguments into id_param (task_id) and time (seconds)
    let id_param = args.next().unwrap_or("").parse::<i32>().unwrap_or(-1);
    let time = args.next().unwrap_or("").parse::<i32>().unwrap_or(-1);

    // Check if the task exists
    match to_do_list.search(id_param) {
        Ok(title) => {
            // Reminder created
            println!("[REMINDER]: Reminder for '{title}' is created with a duration of {time} seconds.");

            // Create an async task
            tokio::spawn(
                async move {
                    // Wait for time
                    sleep(Duration::from_secs(time as u64)).await;

                    // Reminder finished
                    println!("\n[REMINDER]: {}", title);

                    // Clean flush
                    print!(" >>> ");
                    let _ = io::stdout().flush();
                }
            );
        },
        Err(_) => println!("[ERROR]: [{id_param}] not found.")
    }
}

fn complete_command(mut args: SplitWhitespace, to_do_list: &mut ToDoList) {
    // Unwrap the id parameter
    let id_param = args.next().unwrap_or("").parse::<i32>().unwrap_or(-1);

    // Perform the operation and check for the result
    match to_do_list.complete(id_param) {
        Ok(message) => println!("{message}"),
        Err(error_message) => println!("{error_message}")
    }
}

fn list_command(to_do_list: &ToDoList) {
    to_do_list.list();
}

fn help_command() {
    println!("\t======================: HELP :=======================");
    println!("\t >>> add <task_name>");
    println!("\t >>> delete <task_number>");
    println!("\t >>> remind <task_number> <seconds>");
    println!("\t >>> complete <task_number>");
    println!("\t >>> list");
    println!("\t >>> help");
    println!("\t >>> save");
    println!("\t >>> quit");
    println!("\t=====================================================");
}

fn save_command(to_do_list: &ToDoList) {
    // Save the file only
    match to_do_list.save() {
        Ok(message) => println!("{message}"),
        Err(error_message) => println!("{error_message}")
    };
}