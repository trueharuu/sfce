use std::{collections::VecDeque, fmt::Display, str::FromStr};

use chumsky::Parser;

use crate::{piece::Piece, traits::CollectVec};

#[derive(Clone, Debug)]
pub struct Pattern {
    pub parts: Vec<PatternPart>,
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

    pub fn queues(&self) -> Vec<Vec<Piece>> {
      self.clone().into_iter().vec()
    }
}

impl FromStr for Pattern {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
      Self::new(s)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PatternPart {
    All(Box<Self>),
    Count(Box<Self>, usize),
    Single(Piece),
    Bag(Vec<Self>),
    Except(Vec<Self>),
    Wildcard,
}

pub struct PatternIterator {
    queue: VecDeque<Vec<Piece>>,
}

impl PatternIterator {
    pub fn new(pattern: &Pattern) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(vec![]); // Start with an empty sequence
        let mut iter = Self { queue };
        iter.expand_pattern(pattern);
        iter
    }

    fn expand_pattern(&mut self, pattern: &Pattern) {
        for part in &pattern.parts {
            let mut new_queue = VecDeque::new();

            while let Some(current) = self.queue.pop_front() {
                match part {
                    PatternPart::All(inner) => {
                        // Expand the inner pattern to collect all pieces
                        let inner_iter = PatternIterator::new(&Pattern {
                            parts: vec![*inner.clone()],
                        });

                        let results: Vec<Vec<Piece>> = inner_iter.collect();
                        if !results.is_empty() {
                            // Generate all permutations of the entire collection of pieces
                            let all_pieces = results.concat(); // Flatten all results
                            for perm in permutations(&all_pieces, all_pieces.len()) {
                                let mut combined = current.clone();
                                combined.extend(perm);
                                new_queue.push_back(combined);
                            }
                        }
                    }
                    PatternPart::Count(inner, count) => {
                        let inner_iter = PatternIterator::new(&Pattern {
                            parts: vec![*inner.clone()],
                        });

                        
                        let results: Vec<Vec<Piece>> = inner_iter.collect();
                        if *count > results.len() {
                          panic!("cannot take {count} pieces from a bag that only has {}", results.len());
                        }
                        for combination in permutations(&results, *count) {
                            let mut combined = current.clone();
                            combined.extend(combination.into_iter().flatten());
                            new_queue.push_back(combined);
                        }
                    }
                    PatternPart::Single(piece) => {
                        let mut combined = current.clone();
                        combined.push(*piece);
                        new_queue.push_back(combined);
                    }
                    PatternPart::Bag(parts) => {
                        for sub_part in parts {
                            let sub_iter = PatternIterator::new(&Pattern {
                                parts: vec![sub_part.clone()],
                            });

                            for sub_result in sub_iter {
                                let mut combined = current.clone();
                                combined.extend(sub_result);
                                new_queue.push_back(combined);
                            }
                        }
                    }

                    PatternPart::Except(exclusions) => {
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
                        .map(PatternPart::Single);

                        // Compute the set difference: all pieces except the exclusions
                        let filtered_pieces: Vec<PatternPart> = all_pieces
                            .iter()
                            .filter(|piece| !exclusions.contains(piece))
                            .cloned()
                            .collect();

                        // Add each filtered piece to the current queue
                        for sub_part in filtered_pieces {
                            let sub_iter = PatternIterator::new(&Pattern {
                                parts: vec![sub_part.clone()],
                            });

                            for sub_result in sub_iter {
                                let mut combined = current.clone();
                                combined.extend(sub_result);
                                new_queue.push_back(combined);
                            }
                        }
                    }

                    PatternPart::Wildcard => {
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
                            combined.push(piece);
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

impl Iterator for PatternIterator {
    type Item = Vec<Piece>;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

impl IntoIterator for Pattern {
    type Item = Vec<Piece>;
    type IntoIter = PatternIterator;
    fn into_iter(self) -> Self::IntoIter {
        PatternIterator::new(&self)
    }
}

pub mod parse {
    use std::str::FromStr;

    use chumsky::{
        prelude::{choice, group, just, one_of},
        text, IterParser, Parser,
    };

    use crate::piece::Piece;

    use super::{Pattern, PatternPart};

    pub fn parser<'a>() -> impl Parser<'a, &'a str, Pattern> {
        let piece = one_of("IJOLZSTijolzst")
            .map(|x: char| Piece::from_str(&x.to_string()))
            .unwrapped()
            .map(PatternPart::Single);
        let wildcard = just("*").to(PatternPart::Wildcard);
        let bag_except = just("^")
            .ignore_then(piece.repeated().at_least(1).collect())
            .delimited_by(just("["), just("]"))
            .map(PatternPart::Except);
        let bag = piece
            .repeated()
            .at_least(1)
            .collect()
            .delimited_by(just("["), just("]"))
            .map(PatternPart::Bag);
        let repeatable = choice((bag_except, bag, wildcard, piece));
        let count = group((repeatable.clone(), text::int(10).from_str().unwrapped()))
            .map(|(x, y)| PatternPart::Count(Box::new(x), y));
        let all =
            group((repeatable.clone(), just("!"))).map(|(x, _)| PatternPart::All(Box::new(x)));
        let part = choice((all, count, repeatable));
        part.separated_by(just(",").or_not())
            .allow_trailing()
            .collect()
            .map(|x| Pattern { parts: x })
    }
}
