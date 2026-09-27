use std::sync::{Mutex, Arc};
use std::thread;
use rayon::prelude::*;

fn main() {
    let vals = vec![1, 2, 3];
    let squared = vals
        .par_iter() // Rayon parallel iterator
        .map(|x| x * x)
        .collect::<Vec<_>>();

    println!("{:?}", squared);

    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let handles: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[i] += 1;
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", data);
}
