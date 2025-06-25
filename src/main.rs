pub mod actions;
pub mod database;
pub mod tasks;
pub mod utils;

use crate::actions::Action;
use crate::database::create;
use crate::tasks::{Task, create_task};
use crate::utils::read_user_input;
use std::io;

fn main() {
    let mut command: String = String::new();
    println!("Welcome to TUTU application!\nWrite if you want to CREATE / EDIT or DELETE a todo");
    io::stdin()
        .read_line(&mut command)
        .expect("Error in the reading phase");

    let action = read_user_input(command.trim().to_lowercase());

    match action.as_str() {
        "create" | "c" | "new" => {
            println!("Choose a name for your task:")
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
