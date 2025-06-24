use rand::prelude::*;
use rand::rng;

pub fn actions() {
    enum Action {
        Create,
        Edit,
        Delete,
    }
    enum Kanban {
        Active,
        Completed,
        Deleted,
    }
    struct Task {
        id: i32,
        name: String,
        description: String,
        priority: usize, // From 1 to 5
    }

    let id: u8 = random_id_creator();
    println!("{id}");
}

pub fn create() {}

pub fn random_id_creator() -> u8 {
    let mut rng = rng();
    let random_id: u8 = rng.random(); // ID casuale tra 1 e 1000
    return random_id;
}
