use rand::rng;
use rand::seq::SliceRandom;

pub fn create_fruit_salad(mut fruits: Vec<String>) -> Vec<String> {
    let mut thread_rng = rng();
    fruits.shuffle(&mut thread_rng);
    fruits
}
