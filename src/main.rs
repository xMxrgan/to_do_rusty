pub mod database;
pub mod tasks;
pub mod utils;

use crate::database::*;
use crate::tasks::*;
use crate::utils::format_user_input;
use std::io;

fn main() {
    println!("Welcome to TUTU application!");

    let connection = init_db().expect("Error during the connection!");

    loop {
        let mut command: String = String::new();
        println!("Write if you want to CREATE / EDIT or DELETE a todo");
        io::stdin()
            .read_line(&mut command)
            .expect("Error in the reading phase");

        let action = format_user_input(command.trim().to_lowercase());

        match action.as_str() {
            "create" | "c" | "new" => {
                let mut name = String::new();
                let mut description = String::new();
                let mut priority_str = String::new();
                let mut status = String::new();

                println!("Choose a name for your task:");
                io::stdin().read_line(&mut name).expect("Name is invalid!");
                let name = name.trim().to_string();

                println!("Choose a description for your task:");
                io::stdin()
                    .read_line(&mut description)
                    .expect("Description is invalid!");
                let description = description.trim().to_string();

                println!("Choose a priority for your task (number):");
                io::stdin()
                    .read_line(&mut priority_str)
                    .expect("Priority is invalid!");
                let priority: u8 = match priority_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid priority, setting to 0.");
                        0
                    }
                };

                println!("Choose a status for your task:");
                io::stdin()
                    .read_line(&mut status)
                    .expect("Status is invalid!");
                let status = status.trim().to_string();

                let new_task = Task {
                    id: 0,
                    name,
                    description,
                    priority,
                    status,
                };
            }
            "edit" | "e" | "modify" => {
                println!("Choose the task to edit:")
            }
            "delete" | "d" | "del" | "cancel" => {
                println!("Choose the task to delete:")
            }
            _ => println!("Command not valid!"),
        }
    }
}
