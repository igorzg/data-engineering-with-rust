use rand::rng;
use std::collections::HashSet;
use rand::prelude::IndexedRandom;
fn generate_fruit() -> &'static str {
    let fruits = [
        "Apple",
        "Bannana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Grape",
        "Honeydew"
    ];
    let mut _rnd = rng();
    fruits.choose(&mut _rnd).unwrap()
}

fn main() {
    let mut fruit_set = HashSet::new();
    println!("Generating 100 random fruits...");
    for _ in 0..100 {
        fruit_set.insert(generate_fruit());
    }
    println!("Number of unique fruits generated {}", fruit_set.len());
}
