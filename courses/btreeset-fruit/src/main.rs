//! A simple Rust program to generate a fruit salad using a binary heap.
//!
//! # Enum `Fruit`
//!
//! The `Fruit` enum represents different types of fruits that can be included in the salad.
//! It has two variants:
//! - `Fig`: Represents a fig.
//! - `Other(String)`: Represents any other type of fruit, identified by its name as a string.
//!
//! # Implementations
//!
//! ## `Ord` for `Fruit`
//!
//! The `Ord` implementation defines the ordering between fruits in the salad. Figs are always
//! considered greater than other fruits, and other fruits are considered equal to each other.
//!
//! ## `PartialOrd` for `Fruit`
//!
//! The `PartialOrd` implementation simply wraps the `Ord` implementation, ensuring that all
//! comparisons return a valid ordering.
//!
//! # Function `generate_fruit_salad`
//!
//! Generates a fruit salad by creating a binary heap of `Fruit` items. The function ensures that
//! at least two figs are included in the salad. It uses the `rand::seq::IndexedRandom` trait to
//! randomly select fruits from a predefined list.
//!
//! # Returns
//!
//! A `BinaryHeap<Fruit>` containing at least two figs and other randomly selected fruits.
//!
//! # Example
//!
//! ```
//! let salad = generate_fruit_salad();
//! println!("Salads: {:?}", salad);
//! ```
//!
//! This example will print a binary heap of fruits, with at least two figs present.
//!
//! # Main Function
//!
//! The `main` function calls `generate_fruit_salad` to create a fruit salad and then prints it.
//! The output will show the fruits in the heap, following the defined ordering where figs are
//! prioritized over other fruits.
use std::cmp::Ord;
use rand::rng;
use std::collections::{BinaryHeap};
use rand::seq::IndexedRandom;

#[derive(Eq, PartialEq, Debug)]
enum Fruit {
    Fig,
    Other(String)
}

impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut  _rng = rng();
    let fruits = vec![
        "Apple",
        "Orange",
        "Pear",
        "Peach",
        "Banana",
        "Fig",
        "Fig",
        "Fig",
        "Fig"
    ];
    let mut fruit_salat = BinaryHeap::new();
    let mut figs_count = 0;
    while figs_count < 2 {
        let fruit = fruits.choose(&mut _rng).unwrap();
        if *fruit == "Fig" {
            figs_count += 1;
            fruit_salat.push(Fruit::Fig);
        } else {
            fruit_salat.push(Fruit::Other(fruit.to_string()));
        }
    }
    fruit_salat
}

fn main() {
    let salad = generate_fruit_salad();
    println!("Salads: {:?}", salad);
}
