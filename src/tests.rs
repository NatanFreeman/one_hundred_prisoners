#![cfg(test)]

use std::collections::HashSet;
use std::hash::Hash;

use crate::generate_numbers;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

#[test]
///makes sure `generate_numbers` gives unique values
fn unique_numbers() {
    let boxes = generate_numbers();
    assert!(has_unique_elements(boxes.0));
    assert!(has_unique_elements(boxes.1));
}
