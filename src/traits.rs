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
        self.iter
            .by_ref()
            .find(|item| self.seen.insert(item.clone()))
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
    I::Item: Clone,
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

use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub trait FullyDedupParallel: ParallelIterator {
    /// Adapts the parallel iterator to yield only the first instance of each unique item (based on equality).
    fn fully_dedup(self) -> FullyDedupParallelIter<Self>
    where
        Self: Sized,
        Self::Item: Eq + Hash + Send + Sync,
    {
        FullyDedupParallelIter {
            iter: self,
            seen: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    /// Adapts the parallel iterator to yield only the first instance of each unique item,
    /// determined by a predicate function.
    fn fully_dedup_by<F>(self, pred: F) -> FullyDedupByParallelIter<Self, F>
    where
        Self: Sized,
        Self::Item: Send + Sync,
        F: Fn(&Self::Item, &Self::Item) -> bool + Send + Sync,
    {
        FullyDedupByParallelIter {
            iter: self,
            seen: Arc::new(Mutex::new(Vec::new())),
            pred,
        }
    }

    /// Adapts the parallel iterator to yield only the first instance of each unique item,
    /// determined by a key function.
    fn fully_dedup_by_key<K, F>(self, key_fn: F) -> FullyDedupByKeyParallelIter<Self, K, F>
    where
        Self: Sized,
        K: Eq + Hash + Send + Sync,
        F: Fn(&Self::Item) -> K + Send + Sync,
    {
        FullyDedupByKeyParallelIter {
            iter: self,
            seen: Arc::new(Mutex::new(HashSet::new())),
            key_fn,
        }
    }
}

impl<I: ParallelIterator> FullyDedupParallel for I {}

/// Parallel iterator adapter for `fully_dedup`.
pub struct FullyDedupParallelIter<I: ParallelIterator> {
    iter: I,
    seen: Arc<Mutex<HashSet<I::Item>>>,
}

impl<I> ParallelIterator for FullyDedupParallelIter<I>
where
    I: ParallelIterator,
    I::Item: Eq + Hash + Send + Sync + Clone,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        let seen = self.seen;
        self.iter
            .filter(move |item| {
                let mut seen_lock = seen.lock().unwrap();
                seen_lock.insert(item.clone())
            })
            .drive_unindexed(consumer)
    }
}

/// Parallel iterator adapter for `fully_dedup_by`.
pub struct FullyDedupByParallelIter<I, F>
where
    I: ParallelIterator,
{
    iter: I,
    seen: Arc<Mutex<Vec<I::Item>>>,
    pred: F,
}

impl<I, F> ParallelIterator for FullyDedupByParallelIter<I, F>
where
    I: ParallelIterator,
    F: Fn(&I::Item, &I::Item) -> bool + Send + Sync,
    I::Item: Send + Sync + Clone,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        let seen = self.seen;
        let pred = self.pred;
        self.iter
            .filter(move |item| {
                let mut seen_lock = seen.lock().unwrap();
                if seen_lock.iter().any(|seen_item| pred(seen_item, item)) {
                    false
                } else {
                    seen_lock.push(item.clone());
                    true
                }
            })
            .drive_unindexed(consumer)
    }
}

/// Parallel iterator adapter for `fully_dedup_by_key`.
pub struct FullyDedupByKeyParallelIter<I, K, F>
where
    I: ParallelIterator,
{
    iter: I,
    seen: Arc<Mutex<HashSet<K>>>,
    key_fn: F,
}

impl<I, K, F> ParallelIterator for FullyDedupByKeyParallelIter<I, K, F>
where
    I: ParallelIterator,
    K: Eq + Hash + Send + Sync,
    F: Fn(&I::Item) -> K + Send + Sync,
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        let seen = self.seen;
        let key_fn = self.key_fn;
        self.iter
            .filter(move |item| {
                let key = key_fn(item);
                let mut seen_lock = seen.lock().unwrap();
                seen_lock.insert(key)
            })
            .drive_unindexed(consumer)
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
