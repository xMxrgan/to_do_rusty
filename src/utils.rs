use rand::prelude::*;
use rand::rng;

pub fn random_id_creator() -> i32 {
    let mut rng = rng();
    let random_id: i32 = rng.random(); // ID casuale tra 1 e 1000
    return random_id;
}

pub fn format_user_input(s: String) -> String {
    s.trim().to_string()
}
