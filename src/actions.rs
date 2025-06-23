use rand::prelude::*;
use rand::rng;

fn actions() {
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
}

pub fn random_creator() {
    let mut rng = rng();
    let random_id: u8 = rng.random(); // ID casuale tra 1 e 1000
    println!("{random_id}");
}
