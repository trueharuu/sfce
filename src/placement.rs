use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{HashSet, VecDeque},
    sync::{Arc, Mutex},
};

use crate::{
    board::Board,
    input::{Input, Key},
    piece::{Piece, Rotation},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placement {
    pub x: usize,
    pub y: usize,
    pub rotation: Rotation,
    pub piece: Piece,
}

impl Placement {
    pub fn check_inputs(self, board: Board, keys: &[Key], spawn: (usize, usize)) -> bool {
        // println!("trying {keys:?}");
        let mut input = Input::new(board, self.piece, spawn);
        input.send_keys(keys);
        self == input.placement()
    }

    pub fn remove_noop(self, board: Board, keys: &[Key], spawn: (usize, usize)) -> Vec<Key> {
        let mut i = Input::new(board, self.piece, spawn);
        let mut t = vec![];
        for &key in keys {
            if i.is_useful(key) {
                i.send_key(key);
                t.push(key);
            }
        }

        let mut t2 = vec![];
        let mut i = t.iter().peekable();
        // rudimentary noop detection
        while let Some(c) = i.next() {
            if c == &Key::Flip && i.peek() == Some(&&Key::Flip) {
                i.next();
            } else {
                t2.push(*c);
            }
        }

        t2
    }

    pub fn is_doable(&self, board: Board, spawn: (usize, usize), max: usize) -> bool {
        self.inputs(board, spawn, max).is_some()
    }
}

impl Placement {
    pub fn inputs(self, board: Board, spawn: (usize, usize), max: usize) -> Option<Vec<Key>> {
        let visited = Arc::new(Mutex::new(HashSet::new()));
        let queue = Arc::new(Mutex::new(VecDeque::new()));

        queue.lock().unwrap().push_back(Vec::new());

        let possible_moves = vec![
            Key::MoveLeft,
            Key::MoveRight,
            Key::DasLeft,
            Key::DasRight,
            Key::RotateCW,
            Key::RotateCCW,
            Key::Flip,
            Key::SoftDrop,
            Key::SonicDrop,
            // Key::HardDrop,
        ];

        let candidate = Arc::new(Mutex::new(Option::<Vec<Key>>::None));

        loop {
            // Extract all current tasks in the queue.
            let tasks = {
                let mut q = queue.lock().unwrap();
                q.drain(..).collect::<Vec<_>>()
            };

            if tasks.is_empty() {
                break;
            }

            // Process tasks in parallel
            tasks.par_iter().for_each(|current_seq| {
                let cs = self.remove_noop(board.clone(), current_seq, spawn);

                {
                    let current_candidate = candidate.lock().unwrap();
                    if let Some(ref c) = *current_candidate {
                        if c.len() < cs.len() {
                            return;
                        }
                    }
                }

                if cs.len() > max {
                    return;
                }

                if self.check_inputs(board.clone(), &cs, spawn) {
                    let mut current_candidate = candidate.lock().unwrap();
                    *current_candidate = Some(cs.clone());
                }

                let mut local_visited = Vec::new();

                for next_move in &possible_moves {
                    let mut new_sequence = cs.clone();
                    new_sequence.push(*next_move);

                    {
                        let mut visited_lock = visited.lock().unwrap();
                        if visited_lock.contains(&new_sequence) {
                            continue;
                        }
                        visited_lock.insert(new_sequence.clone());
                    }

                    local_visited.push(new_sequence);
                }

                // Add new sequences to the queue
                let mut q = queue.lock().unwrap();
                for seq in local_visited {
                    q.push_back(seq);
                }
            });
        }

        Arc::try_unwrap(candidate)
            .ok()
            .unwrap()
            .into_inner()
            .unwrap()
    }
}
