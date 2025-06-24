pub mod actions;
pub mod tasks;
pub mod utils;

use crate::tasks::{Task, create_task};
use std::io;

fn main() {
    println!("Welcome to TUTU application!");

    create_task(name, description, priority);
}
