use std::collections::HashMap;

fn init_languages() -> HashMap<String, i32> {
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("SQL".to_string(), 1974);
    languages.insert("TypeScript".to_string(), 2012);
    languages.insert("Bash/Shell".to_string(), 1989);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(), 2000);
    languages.insert("C++".to_string(), 1985);
    languages.insert("C".to_string(), 1972);
    languages.insert("PHP".to_string(), 1995);
    languages.insert("PowerShell".to_string(), 2006);
    languages.insert("Go".to_string(), 2007);
    languages.insert("Rust".to_string(), 2010);
    languages
}

fn calculate_weights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    for year in years_active.values_mut() {
        *year = 2026 - *year;
    }
    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights = HashMap::new();
    for (language, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1;
        weights.insert(language.to_string(), weight);
    }
    weights
}

fn main() {
    let mut languages = init_languages();
    let weights = calculate_weights(&mut languages);

    println!("Language weighing from 1-100 by age (1 is newest and 100 ist oldest)");
    for (language, weight) in weights {
        println!("{}: {}", language, weight);
    }

    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");
    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    println!("\t\tVecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html");
    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html");

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");

    // Sets
    println!("\n\tSets:");
    println!("\t\tHashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html");
    println!("\t\tBTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html");

    // Misc
    println!("\n\tMisc:");
    println!("\t\tBinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html");
}
