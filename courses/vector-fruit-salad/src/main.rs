use rand::rng;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

fn main() {
    // let mut fruit = vec![
    //     "Orange",
    //     "Fig",
    //     "Pomegranate",
    //     "Cherry",
    //     "Apple",
    //     "Pear",
    //     "Peach",
    // ];
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    let mut rng = rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();
    fruit.push_back("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");

    println!("VecDeque Fruit salad: ");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            println!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
