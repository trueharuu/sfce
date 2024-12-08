use std::{collections::VecDeque, fmt::Display, str::FromStr};

use chumsky::Parser;
use itertools::Itertools;

use crate::{piece::Piece, traits::CollectVec};

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

    use super::{Pattern, Part};

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
        let all =
            group((repeatable.clone(), just("!"))).map(|(x, _)| Part::All(Box::new(x)));
        let part = choice((all, count, repeatable));
        part.separated_by(just(",").or_not())
            .allow_trailing()
            .collect()
            .map(|x| Pattern { parts: x })
    }
}
