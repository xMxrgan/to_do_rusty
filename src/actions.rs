use crate::tasks::Task;
use crate::utils::random_id_creator;

pub enum Action {
    Create,
    Edit,
    Delete,
}
pub enum Status {
    Active,
    Completed,
    Deleted,
}

pub fn create_action(name: String) -> Task {
    let new_id: u8 = random_id_creator();
    // let new_task: Task = Task { id: new_id };
}
