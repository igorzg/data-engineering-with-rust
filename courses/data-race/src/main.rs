use rayon::prelude::*;

fn main() {
    let vals = vec![1, 2, 3];
    let squared = vals
        .par_iter() // Rayon parallel iterator
        .map(|x| x * x)
        .collect::<Vec<_>>();

    println!("{:?}", squared);
}
