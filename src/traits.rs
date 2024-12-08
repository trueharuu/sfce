pub trait CollectVec {
    type Item;
    fn vec(self) -> Vec<Self::Item>;
}

impl<T> CollectVec for T
where
    T: Iterator,
{
    type Item = T::Item;
    fn vec(self) -> Vec<Self::Item> {
        // dumb
        self.collect()
    }
}

pub trait GetWith: IntoIterator {
    fn get_with<F>(self, f: F) -> Option<Self::Item>
    where
        F: FnMut(&Self::Item) -> bool,
        Self: Sized,
    {
        self.into_iter().find(f)
    }
}

use std::collections::HashSet;
use std::hash::Hash;

pub trait FullyDedup: Iterator {
    /// Adapts the iterator to yield only the first instance of each unique item (based on equality).
    fn fully_dedup(self) -> FullyDedupIter<Self>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        FullyDedupIter {
            iter: self,
            seen: HashSet::new(),
        }
    }

    /// Adapts the iterator to yield only the first instance of each unique item,
    /// determined by a provided predicate function.
    fn fully_dedup_by<F>(self, pred: F) -> FullyDedupByIter<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    {
        FullyDedupByIter {
            iter: self,
            seen: Vec::new(),
            pred,
        }
    }

    /// Adapts the iterator to yield only the first instance of each unique item,
    /// determined by a key function.
    fn fully_dedup_by_key<K, F>(self, key_fn: F) -> FullyDedupByKeyIter<Self, K, F>
    where
        Self: Sized,
        K: Eq + Hash,
        F: FnMut(&Self::Item) -> K,
    {
        FullyDedupByKeyIter {
            iter: self,
            seen: HashSet::new(),
            key_fn,
        }
    }
}

impl<I: Iterator> FullyDedup for I {}

/// Iterator adapter for `fully_dedup`.
pub struct FullyDedupIter<I: Iterator> {
    iter: I,
    seen: HashSet<I::Item>,
}

impl<I> Iterator for FullyDedupIter<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            if self.seen.insert(item.clone()) {
                return Some(item);
            }
        }
        None
    }
}

/// Iterator adapter for `fully_dedup_by`.
pub struct FullyDedupByIter<I, F>
where
    I: Iterator,
{
    iter: I,
    seen: Vec<I::Item>,
    pred: F,
}

impl<I, F> Iterator for FullyDedupByIter<I, F>
where
    I: Iterator,
    F: FnMut(&I::Item, &I::Item) -> bool,
    I::Item: Clone
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            if !self
                .seen
                .iter()
                .any(|seen_item| (self.pred)(&item, seen_item))
            {
                self.seen.push(item.clone());
                return Some(item);
            }
        }
        None
    }
}

/// Iterator adapter for `fully_dedup_by_key`.
pub struct FullyDedupByKeyIter<I, K, F>
where
    I: Iterator,
{
    iter: I,
    seen: HashSet<K>,
    key_fn: F,
}

impl<I, K, F> Iterator for FullyDedupByKeyIter<I, K, F>
where
    I: Iterator,
    K: Eq + Hash,
    F: FnMut(&I::Item) -> K,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        for item in self.iter.by_ref() {
            let key = (self.key_fn)(&item);
            if self.seen.insert(key) {
                return Some(item);
            }
        }
        None
    }
}

impl<T> GetWith for T where T: IntoIterator {}

pub fn contiguous_subsequences<I, T>(input: I) -> Vec<Vec<T>>
where
    I: IntoIterator<Item = T>,
    T: Clone,
{
    let input: Vec<T> = input.into_iter().collect(); // Collect the input into a Vec
    let mut out = vec![];

    for start in 0..input.len() {
        for end in start..input.len() {
            out.push(input[start..=end].to_vec());
        }
    }

    out
}

// TODO: make this really fast
pub fn contiguous_cut_seqs<I, T>(input: I) -> Vec<(Vec<T>, Vec<T>, Vec<T>)>
where
    I: IntoIterator<Item = T>,
    T: Clone,
{
    let input: Vec<T> = input.into_iter().collect(); // Collect the input into a Vec
    let mut out = vec![];

    for start in 0..input.len() {
        for end in start..input.len() {
            out.push((
                input[..start].to_vec(),
                input[start..=end].to_vec(),
                input[end + 1..].to_vec(),
            ));
        }
    }

    out
}

pub fn do_until_same<T>(mut value: T, mut func: impl FnMut(T) -> T) -> T
where
    T: PartialEq + Clone,
{
    loop {
        let new_value = func(value.clone());
        if new_value == value {
            break; // Exit the loop if the value didn't change
        }
        value = new_value; // Update the value with the new value
    }
    value // Return the final value
}
