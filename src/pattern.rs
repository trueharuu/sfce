use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

use chumsky::Parser;
use itertools::Itertools;

use crate::{
    piece::Piece,
    traits::{CollectVec, FullyDedup},
};

#[derive(Clone, Debug)]
pub struct Pattern {
    pub parts: Vec<Part>,
}

impl Pattern {
    pub fn new(s: impl Display) -> Result<Self, String> {
        let t = s.to_string();
        let x = parse::parser()
            .parse(&t)
            .into_output()
            .ok_or("malformed pattern".to_string());
        x
    }

    #[must_use]
    pub fn queues(&self) -> Vec<Queue> {
        self.clone().into_iter().vec()
    }

    pub fn into_iter_with_hold(self) -> impl Iterator<Item = Queue> {
        self.into_iter().fully_dedup_by(Queue::translatable)
    }

    #[must_use]
    pub fn queues_with_hold(&self) -> Vec<Queue> {
        self.clone().into_iter_with_hold().vec()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Queue(Vec<Piece>);

impl Queue {
    #[must_use]
    pub fn empty() -> Self {
        Self(vec![])
    }

    #[must_use]
    pub fn pieces(&self) -> &[Piece] {
        &self.0
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Piece> {
        self.0.iter()
    }

    #[must_use]
    pub fn translatable(&self, b: &Queue) -> bool {
        #[derive(Debug, PartialEq, Eq, Clone, Hash)]
        struct State {
            queue: VecDeque<Piece>,
            hold: Option<Piece>,
            target_index: usize,
        }
        if self.len() != b.len() {
            return false; // Lengths must match
        }

        let initial_queue: VecDeque<Piece> = self.iter().copied().collect();
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        // Initial state: all pieces in the queue, no hold, and starting at target index 0
        stack.push(State {
            queue: initial_queue,
            hold: None,
            target_index: 0,
        });

        while let Some(state) = stack.pop() {
            if state.target_index == b.len() {
                return true; // Successfully matched the entire target sequence
            }

            if visited.contains(&state) {
                continue; // Avoid revisiting the same state
            }

            visited.insert(state.clone());

            let target = b.0[state.target_index];

            // Option 1: Place the front of the queue
            if let Some(&front) = state.queue.front() {
                if front == target {
                    let mut new_queue = state.queue.clone();
                    new_queue.pop_front();
                    stack.push(State {
                        queue: new_queue,
                        hold: state.hold,
                        target_index: state.target_index + 1,
                    });
                }
            }

            // Option 2: Place the hold piece
            if let Some(held_piece) = state.hold {
                if held_piece == target {
                    let mut new_queue = state.queue.clone();
                    let new_hold = new_queue.pop_front();
                    stack.push(State {
                        queue: new_queue,
                        hold: new_hold,
                        target_index: state.target_index + 1,
                    });
                }
            }

            // Option 3: Hold the front of the queue (if hold is empty or hold does not match target)
            if let Some(front) = state.queue.front() {
                if state.hold.is_none() || state.hold != Some(target) {
                    let mut new_queue = state.queue.clone();
                    new_queue.pop_front();
                    stack.push(State {
                        queue: new_queue,
                        hold: Some(*front),
                        target_index: state.target_index,
                    });
                }
            }
        }

        false // No valid sequence found
    }

    #[must_use]
    pub fn hold_queues(&self) -> HashSet<Self> {
        #[derive(Debug, PartialEq, Eq, Clone, Hash)]
        struct State {
            queue: VecDeque<Piece>,
            hold: Option<Piece>,
            sequence: Vec<Piece>,
        }
        let mut result = HashSet::new();
        let mut visited = HashSet::new();
        let mut stack = Vec::new();

        // Initial state: no hold, starting queue, and an empty sequence
        stack.push(State {
            queue: VecDeque::from(self.0.clone()),
            hold: None,
            sequence: vec![],
        });

        while let Some(state) = stack.pop() {
            // If we've already visited this state, skip it
            if !visited.insert((state.queue.clone(), state.hold, state.sequence.clone())) {
                continue;
            }

            // If the queue is empty, finalize the sequence
            if state.queue.is_empty() {
                if let Some(held_piece) = state.hold {
                    // If there's a piece in the hold, it must be placed
                    let mut final_sequence = state.sequence.clone();
                    final_sequence.push(held_piece);
                    result.insert(Self(final_sequence));
                } else {
                    // If no hold piece remains, just add the sequence
                    result.insert(Self(state.sequence.clone()));
                }
                continue;
            }

            // Option 1: Place the front piece from the queue
            if let Some(&front) = state.queue.front() {
                let mut new_queue = state.queue.clone();
                let mut new_sequence = state.sequence.clone();
                new_sequence.push(front); // Add the placed piece to the sequence
                new_queue.pop_front();
                stack.push(State {
                    queue: new_queue,
                    hold: state.hold,
                    sequence: new_sequence,
                });
            }

            // Option 2: Swap the front piece with the hold piece
            if let Some(&front) = state.queue.front() {
                let mut new_queue = state.queue.clone();
                new_queue.pop_front(); // Remove the front piece from the queue

                if let Some(held_piece) = state.hold {
                    // If there's a piece in the hold, swap it with the front
                    let mut swapped_queue = new_queue.clone();
                    swapped_queue.push_front(held_piece); // Add the held piece back to the front
                    stack.push(State {
                        queue: swapped_queue,
                        hold: Some(front),
                        sequence: state.sequence.clone(),
                    });
                } else {
                    // If the hold is empty, store the front piece in the hold
                    stack.push(State {
                        queue: new_queue,
                        hold: Some(front),
                        sequence: state.sequence.clone(),
                    });
                }
            }
        }

        result
    }
}

impl Display for Queue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(std::string::ToString::to_string).join("")
        )
    }
}

impl Extend<Piece> for Queue {
    fn extend<T: IntoIterator<Item = Piece>>(&mut self, iter: T) {
        for i in iter {
            self.0.push(i);
        }
    }
}

impl IntoIterator for Queue {
    type Item = Piece;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        #[allow(clippy::unnecessary_to_owned)]
        self.pieces().to_vec().into_iter()
    }
}

impl<'a> IntoIterator for &'a Queue {
    type Item = &'a Piece;
    type IntoIter = core::slice::Iter<'a, Piece>;
    fn into_iter(self) -> Self::IntoIter {
        self.pieces().iter()
    }
}

impl FromStr for Pattern {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Part {
    All(Box<Self>),
    Count(Box<Self>, usize),
    Single(Piece),
    Bag(Vec<Self>),
    Except(Vec<Self>),
    Wildcard,
}

pub struct Iter {
    queue: VecDeque<Queue>,
}

impl Iter {
    #[must_use]
    pub fn new(pattern: &Pattern) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(Queue::empty()); // Start with an empty sequence
        let mut iter = Self { queue };
        iter.expand_pattern(pattern);
        iter
    }

    #[allow(clippy::too_many_lines)]
    fn expand_pattern(&mut self, pattern: &Pattern) {
        for part in &pattern.parts {
            let mut new_queue = VecDeque::new();

            while let Some(current) = self.queue.pop_front() {
                match part {
                    Part::All(inner) => {
                        // Expand the inner pattern to collect all pieces
                        let inner_iter = Iter::new(&Pattern {
                            parts: vec![*inner.clone()],
                        });

                        let results: Vec<_> = inner_iter.collect();
                        if !results.is_empty() {
                            // Generate all permutations of the entire collection of pieces
                            let all_pieces = results.into_iter().concat(); // Flatten all results
                            for perm in permutations(all_pieces.pieces(), all_pieces.len()) {
                                let mut combined = current.clone();
                                combined.0.extend(perm);
                                new_queue.push_back(combined);
                            }
                        }
                    }
                    Part::Count(inner, count) => {
                        let inner_iter = Iter::new(&Pattern {
                            parts: vec![*inner.clone()],
                        });

                        let results: Vec<_> = inner_iter.collect();
                        assert!(
                            *count <= results.len(),
                            "cannot take {count} pieces from a bag that only has {}",
                            results.len()
                        );
                        for combination in permutations(&results, *count) {
                            let mut combined = current.clone();
                            combined.0.extend(combination.into_iter().flatten());
                            new_queue.push_back(combined);
                        }
                    }
                    Part::Single(piece) => {
                        let mut combined = current.clone();
                        combined.0.push(*piece);
                        new_queue.push_back(combined);
                    }
                    Part::Bag(parts) => {
                        for sub_part in parts {
                            let sub_iter = Iter::new(&Pattern {
                                parts: vec![sub_part.clone()],
                            });

                            for sub_result in sub_iter {
                                let mut combined = current.clone();
                                combined.0.extend(sub_result);
                                new_queue.push_back(combined);
                            }
                        }
                    }

                    Part::Except(exclusions) => {
                        // Define the full set of pieces
                        let all_pieces = [
                            Piece::I,
                            Piece::J,
                            Piece::O,
                            Piece::L,
                            Piece::Z,
                            Piece::S,
                            Piece::T,
                        ]
                        .map(Part::Single);

                        // Compute the set difference: all pieces except the exclusions
                        let filtered_pieces: Vec<Part> = all_pieces
                            .iter()
                            .filter(|piece| !exclusions.contains(piece))
                            .cloned()
                            .collect();

                        // Add each filtered piece to the current queue
                        for sub_part in filtered_pieces {
                            let sub_iter = Iter::new(&Pattern {
                                parts: vec![sub_part.clone()],
                            });

                            for sub_result in sub_iter {
                                let mut combined = current.clone();
                                combined.0.extend(sub_result);
                                new_queue.push_back(combined);
                            }
                        }
                    }

                    Part::Wildcard => {
                        for piece in [
                            Piece::I,
                            Piece::J,
                            Piece::O,
                            Piece::L,
                            Piece::Z,
                            Piece::S,
                            Piece::T,
                        ] {
                            let mut combined = current.clone();
                            combined.0.push(piece);
                            new_queue.push_back(combined);
                        }
                    }
                }
            }

            self.queue = new_queue;
        }
    }
}

fn permutations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }

    let mut result = vec![];
    for (i, item) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(i);
        for mut sub_perm in permutations(&rest, k - 1) {
            let mut perm = vec![item.clone()];
            perm.append(&mut sub_perm);
            result.push(perm);
        }
    }
    result
}

impl Iterator for Iter {
    type Item = Queue;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

impl IntoIterator for Pattern {
    type Item = Queue;
    type IntoIter = Iter;
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(&self)
    }
}

pub mod parse {
    use std::str::FromStr;

    use chumsky::{
        prelude::{choice, group, just, one_of},
        text, IterParser, Parser,
    };

    use crate::piece::Piece;

    use super::{Part, Pattern};

    pub fn parser<'a>() -> impl Parser<'a, &'a str, Pattern> {
        let piece = one_of("IJOLZSTijolzst")
            .map(|x: char| Piece::from_str(&x.to_string()))
            .unwrapped()
            .map(Part::Single);
        let wildcard = just("*").to(Part::Wildcard);
        let bag_except = just("^")
            .ignore_then(piece.repeated().at_least(1).collect())
            .delimited_by(just("["), just("]"))
            .map(Part::Except);
        let bag = piece
            .repeated()
            .at_least(1)
            .collect()
            .delimited_by(just("["), just("]"))
            .map(Part::Bag);
        let repeatable = choice((bag_except, bag, wildcard, piece));
        let count = group((repeatable.clone(), text::int(10).from_str().unwrapped()))
            .map(|(x, y)| Part::Count(Box::new(x), y));
        let all = group((repeatable.clone(), just("!"))).map(|(x, _)| Part::All(Box::new(x)));
        let part = choice((all, count, repeatable));
        part.separated_by(just(",").or_not())
            .allow_trailing()
            .collect()
            .map(|x| Pattern { parts: x })
    }
}
