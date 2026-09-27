use sha3_dupe_detector::{generate_random_phrases, analyze_duplicates};

fn main() {
    let phrases = generate_random_phrases();
    analyze_duplicates(&phrases);
}
