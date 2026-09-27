//! Task 3 tests: linked list of top words, alphabetical sort, top-N.

use std::collections::HashMap;

use challange_week1::word_list::WordList;

#[test]
fn push_preserves_insertion_order() {
    let mut list = WordList::new();
    for word in ["banana", "apple", "cherry"] {
        list.push(word, 1);
    }
    let order: Vec<&str> = list.iter().map(|(word, _)| word).collect();
    assert_eq!(order, ["banana", "apple", "cherry"]);
}

#[test]
fn pop_removes_from_the_head() {
    let mut list = WordList::new();
    list.push("one", 1);
    list.push("two", 2);
    assert_eq!(list.pop().unwrap().word, "one");
    assert_eq!(list.pop().unwrap().word, "two");
    assert!(list.pop().is_none());
}

#[test]
fn sort_alphabetically_reorders_nodes() {
    let mut list = WordList::new();
    for word in ["delta", "alpha", "charlie", "bravo"] {
        list.push(word, 1);
    }
    list.sort_alphabetically();
    let order: Vec<&str> = list.iter().map(|(word, _)| word).collect();
    assert_eq!(order, ["alpha", "bravo", "charlie", "delta"]);
}

#[test]
fn sort_alphabetically_on_empty_list_is_a_noop() {
    let mut list = WordList::new();
    list.sort_alphabetically();
    assert_eq!(list.to_string(), "");
}

#[test]
fn top_n_orders_by_count_then_word() {
    let mut frequencies: HashMap<String, usize> = HashMap::new();
    for (word, count) in [("zebra", 5usize), ("apple", 5usize), ("mango", 2usize)] {
        frequencies.insert(word.to_string(), count);
    }
    let list = WordList::top_n(&frequencies, 2);
    let order: Vec<(&str, usize)> = list.iter().collect();
    assert_eq!(order, [("apple", 5), ("zebra", 5)]);
}

#[test]
fn display_joins_nodes() {
    let mut list = WordList::new();
    list.push("one", 3);
    list.push("two", 1);
    assert_eq!(list.to_string(), "one (3) -> two (1)");
}
