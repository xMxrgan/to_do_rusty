use crate::utils::random_id_creator;

pub struct Task {
    id: u8,
    name: String,
    description: String,
    priority: u8, // From 1 to 5
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
    };

    Ok(new_task)
}

fn priority_check(priority: u8) -> bool {
    (1..=5).contains(&priority)
}
