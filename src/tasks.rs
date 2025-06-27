use crate::utils::random_id_creator;

pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub priority: u8, // From 1 to 5
    pub status: String,
}

enum Status {
    ToDo,
    InProgress,
    Done,
}

pub fn create_task(name: String, description: String, priority: u8) -> Result<Task, String> {
    let id = random_id_creator();

    if !priority_check(priority) {
        return Err("The priority is supposed to be between 1 and 5!".to_string());
    }

    let new_task = Task {
        id,
        name,
        description,
        priority,
        status,
        ToDO,
    };

    Ok(new_task)
}

fn priority_check(priority: u8) -> bool {
    (1..=5).contains(&priority)
}

fn edit_task() {}

fn delete_task() {}

fn compare_task(input: String) {}
