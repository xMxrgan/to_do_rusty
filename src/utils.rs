use rand::prelude::*;
use rand::rng;

pub fn random_id_creator() -> u8 {
    let mut rng = rng();
    let random_id: u8 = rng.random(); // ID casuale tra 1 e 1000
    return random_id;
}
