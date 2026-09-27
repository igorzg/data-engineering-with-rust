use std::collections::HashMap;

use decoder_ring::stat_analysis;

const EPS: f32 = 1e-3;

fn approx(a: f32, b: f32) -> bool {
    (a - b).abs() < EPS
}

/// `HashMap<char, _>` indexing requires an explicit `&char` operand.
fn char_get<V: Copy>(m: &HashMap<char, V>, c: char) -> V {
    m[&c]
}

/// Re-index the flat `stat_analysis` output by character, since `HashMap`
/// iteration order (and thus the Vec order) is nondeterministic.
fn stats_by_char(text: &str) -> HashMap<char, (u32, f32, Option<f32>, f32)> {
    stat_analysis(text)
        .into_iter()
        .map(|(c, count, freq, eng, diff)| (c, (count, freq, eng, diff)))
        .collect()
}

#[test]
fn empty_text_yields_no_stats() {
    assert!(stat_analysis("").is_empty());
}

#[test]
fn counts_and_frequencies() {
    let m = stats_by_char("aab");
    assert_eq!(m.len(), 2);

    let (count, freq, ..) = char_get(&m, 'a');
    assert_eq!(count, 2);
    assert!(approx(freq, 200.0 / 3.0));

    let (count, freq, ..) = char_get(&m, 'b');
    assert_eq!(count, 1);
    assert!(approx(freq, 100.0 / 3.0));
}

#[test]
fn non_alphabetic_chars_are_counted_too() {
    let m = stats_by_char("a b-");
    assert_eq!(m.len(), 4);
    for c in ['a', ' ', 'b', '-'] {
        assert!(m.contains_key(&c));
    }
}

#[test]
fn english_freq_lookup_is_case_insensitive() {
    let m = stats_by_char("E");
    let (.., eng, _) = char_get(&m, 'E');
    assert!(approx(eng.unwrap(), 12.7));
}

#[test]
fn letter_outside_top_ten_has_no_english_freq() {
    let m = stats_by_char("z");
    let (.., eng, diff) = char_get(&m, 'z');
    assert!(eng.is_none());
    assert!(approx(diff, 0.0));
}

#[test]
fn english_freq_diff_is_the_absolute_gap() {
    let m = stats_by_char("ee");
    let (.., eng, diff) = char_get(&m, 'e');
    assert!(approx(eng.unwrap(), 12.7));
    assert!(approx(diff, (100.0_f32 - 12.7_f32).abs()));
}

#[test]
fn frequencies_sum_to_hundred() {
    let m = stats_by_char("The quick brown fox");
    let sum: f32 = m.values().map(|(_, freq, _, _)| *freq).sum();
    assert!(approx(sum, 100.0));
}
