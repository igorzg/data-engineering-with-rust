//! Task 3: build a singly linked list holding the most common words of a
//! text, sort the list alphabetically, and print the top 10 entries.

use std::collections::HashMap;
use std::fmt;

/// One node of a [`WordList`].
#[derive(Debug)]
pub struct WordNode {
    /// The normalised word.
    pub word: String,
    /// Occurrences of the word in the source text.
    pub count: usize,
    next: Option<Box<WordNode>>,
}

/// A singly linked list of `(word, count)` pairs.
#[derive(Debug, Default)]
pub struct WordList {
    head: Option<Box<WordNode>>,
}

impl WordList {
    /// Creates an empty list.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends `(word, count)` to the tail of the list. `O(n)` in the length
    /// of the list.
    pub fn push(&mut self, word: &str, count: usize) {
        let node = Box::new(WordNode {
            word: word.to_string(),
            count,
            next: None,
        });
        push_to_tail(&mut self.head, node);
    }

    /// Removes and returns the head node.
    pub fn pop(&mut self) -> Option<Box<WordNode>> {
        let mut head = self.head.take()?;
        self.head = head.next.take();
        Some(head)
    }

    /// Iterates over the `(word, count)` pairs in list order.
    #[must_use]
    pub fn iter(&self) -> WordIter<'_> {
        WordIter {
            current: self.head.as_deref(),
        }
    }

    /// Sorts the list in place, alphabetically by `word`, using insertion sort.
    ///
    /// Nodes are taken off the list one by one and spliced into their sorted
    /// position in a fresh prefix — pure pointer surgery, no intermediate
    /// collection.
    pub fn sort_alphabetically(&mut self) {
        let mut sorted = WordList::new();
        while let Some(node) = self.pop() {
            insert_sorted(&mut sorted.head, node);
        }
        self.head = sorted.head;
    }

    /// Builds a list of the `n` most common words from `frequencies`,
    /// ordered by count (descending), ties broken alphabetically.
    #[must_use]
    pub fn top_n(frequencies: &HashMap<String, usize>, n: usize) -> WordList {
        let mut ranked: Vec<(&String, &usize)> = frequencies.iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
        let mut list = WordList::new();
        for (word, count) in ranked.into_iter().take(n) {
            list.push(word, *count);
        }
        list
    }
}

/// Walks to the tail of the chain rooted at `slot` and links `node` there.
fn push_to_tail(slot: &mut Option<Box<WordNode>>, node: Box<WordNode>) {
    match slot {
        Some(current) => push_to_tail(&mut current.next, node),
        None => *slot = Some(node),
    }
}

/// Splices `node` into `slot`'s chain at its alphabetical position.
fn insert_sorted(slot: &mut Option<Box<WordNode>>, mut node: Box<WordNode>) {
    match slot {
        Some(current) if node.word < current.word => {
            node.next = slot.take();
            *slot = Some(node);
        }
        Some(current) => {
            insert_sorted(&mut current.next, node);
        }
        None => *slot = Some(node),
    }
}

/// Iterator over the `(word, count)` pairs of a [`WordList`].
pub struct WordIter<'a> {
    current: Option<&'a WordNode>,
}

impl<'a> Iterator for WordIter<'a> {
    type Item = (&'a str, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.current?;
        self.current = node.next.as_deref();
        Some((node.word.as_str(), node.count))
    }
}

impl<'a> IntoIterator for &'a WordList {
    type Item = (&'a str, usize);
    type IntoIter = WordIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl fmt::Display for WordList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut separated = false;
        for (word, count) in self {
            if separated {
                write!(f, " -> ")?;
            }
            write!(f, "{word} ({count})")?;
            separated = true;
        }
        Ok(())
    }
}
