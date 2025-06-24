use rand::prelude::*;
use rand::rng;

use crate::utils;

pub fn actions() {
    let id: u8 = utils::random_id_creator();
    println!("{id}");
}

pub fn create() {}
