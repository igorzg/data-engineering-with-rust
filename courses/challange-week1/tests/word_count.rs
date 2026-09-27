//! Task 2 tests: word frequencies with capitalisation/punctuation normalisation.

use challange_week1::word_count::{normalize, word_frequencies};

#[test]
fn combines_case_and_punctuation() {
    let frequencies = word_frequencies("The, the THE! the. (the)");
    assert_eq!(frequencies["the"], 5);
    assert_eq!(frequencies.len(), 1);
}

#[test]
fn ignores_punctuation_only_tokens() {
    let frequencies = word_frequencies("... --- ?! hello");
    assert_eq!(frequencies.len(), 1);
    assert_eq!(frequencies["hello"], 1);
}

#[test]
fn keeps_inner_apostrophes() {
    let frequencies = word_frequencies("don't Don't!");
    assert_eq!(frequencies["don't"], 2);
}

#[test]
fn lowercases_unicode() {
    assert_eq!(normalize("Café"), Some("café".to_string()));
}

#[test]
fn rejects_tokens_without_letters() {
    assert_eq!(normalize("..."), None);
    assert_eq!(normalize(""), None);
}

#[test]
fn empty_text_yields_empty_map() {
    assert!(word_frequencies("").is_empty());
}
