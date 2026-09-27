use std::collections::HashMap;

use decoder_ring::gen_counts;

const EPS: f32 = 1e-3;

fn approx(a: f32, b: f32) -> bool {
    (a - b).abs() < EPS
}

/// `HashMap<char, _>` indexing requires an explicit `&char` operand.
fn char_get<V: Copy>(m: &HashMap<char, V>, c: char) -> V {
    m[&c]
}

#[test]
fn covers_the_ten_most_common_english_letters() {
    assert_eq!(gen_counts().len(), 10);
}

#[test]
fn keys_are_lowercase_letters() {
    for key in gen_counts().keys() {
        assert!(key.is_ascii_lowercase(), "key '{key}' should be lowercase");
    }
}

#[test]
fn spot_check_known_frequencies() {
    let m = gen_counts();
    assert!(approx(char_get(&m, 'e'), 12.7));
    assert!(approx(char_get(&m, 't'), 9.1));
    assert!(approx(char_get(&m, 'a'), 8.2));
    assert!(approx(char_get(&m, 'd'), 4.3));
}

#[test]
fn rare_letters_are_absent() {
    let m = gen_counts();
    assert!(!m.contains_key(&'z'));
    assert!(!m.contains_key(&'q'));
}

#[test]
fn all_frequencies_are_positive() {
    assert!(gen_counts().values().all(|&v| v > 0.0));
}
