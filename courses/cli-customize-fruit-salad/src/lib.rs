use rand::seq::SliceRandom;
use rand::rng;

pub fn create_fruit_salad(mut fruits: Vec<String>) -> Vec<String> {
    let mut thread_rng = rng();
    fruits.shuffle(&mut thread_rng);
    fruits
}