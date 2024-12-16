use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use dashmap::DashSet;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};

use crate::{
    board::Board,
    input::{Input, Key},
    piece::{Piece, Rotation},
    program::Handling,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Placement(Piece, usize, usize, Rotation);

impl Placement {
    #[must_use]
    pub fn new(piece: Piece, x: usize, y: usize, rotation: Rotation) -> Self {
        Placement(piece, x, y, rotation)
    }

    #[must_use]
    pub fn piece(&self) -> Piece {
        self.0
    }

    #[must_use]
    pub fn x(&self) -> usize {
        self.1
    }

    #[must_use]
    pub fn y(&self) -> usize {
        self.2
    }

    #[must_use]
    pub fn location(&self) -> (usize, usize) {
        (self.1, self.2)
    }

    #[must_use]
    pub fn rotation(&self) -> Rotation {
        self.3
    }

    #[must_use]
    pub fn at(mut self, (x, y): (usize, usize)) -> Self {
        self.move_to((x, y));
        self
    }

    pub fn move_to(&mut self, (x, y): (usize, usize)) {
        self.1 = x;
        self.2 = y;
    }

    pub fn set_rotation(&mut self, rotation: Rotation) {
        self.3 = rotation;
    }

    pub fn set_x(&mut self, x: usize) {
        self.1 = x;
    }

    pub fn set_y(&mut self, y: usize) {
        self.2 = y;
    }

    #[must_use]
    pub fn check_inputs(
        self,
        board: &Board,
        keys: &[Key],
        spawn: (usize, usize),
        handling: Handling,
    ) -> bool {
        // println!("trying {keys:?}");
        let mut input = Input::new(board, self.piece(), spawn, Rotation::North, handling);
        input.send_keys(keys);
        self == input.placement()
    }

    #[must_use]
    pub fn is_input_useful(
        self,
        board: &Board,
        orig_keys: &[Key],
        key: Key,
        spawn: (usize, usize),
        handling: Handling,
    ) -> bool {
        let mut i = Input::new(board, self.piece(), spawn, Rotation::North, handling);
        i.send_keys(orig_keys);
        // println!("currently in state");
        i.can(key)
    }

    #[must_use]
    pub fn is_doable(&self, board: &Board, spawn: (usize, usize), mut handling: Handling) -> bool {
        handling.finesse = false;
        self.inputs(board, spawn, handling).is_some()
    }
}

impl Placement {
    #[must_use]
    pub fn finesse(
        self,
        board: &Board,
        spawn: (usize, usize),
        mut handling: Handling,
    ) -> Option<Vec<Key>> {
        handling.finesse = true;
        self.inputs(board, spawn, handling)
    }

    #[must_use]
    pub fn inputs(
        self,
        board: &Board,
        spawn: (usize, usize),
        handling: Handling,
    ) -> Option<Vec<Key>> {
        let visited = Arc::new(DashSet::new());
        let candidate: Arc<Mutex<Option<Vec<Key>>>> = Arc::new(Mutex::new(None));
        let possible_moves = handling.possible_moves();

        let mut queue = VecDeque::new();
        queue.push_back(Vec::new());

        while !queue.is_empty() {
            let batch: Vec<Vec<Key>> = queue.drain(0..queue.len()).collect();

            let new_sequences: Vec<Vec<Key>> = batch
                .into_par_iter()
                .flat_map(|current_seq| {
                    let mut local_new_sequences = Vec::new();

                    if visited.contains(&current_seq) {
                        return local_new_sequences;
                    }

                    visited.insert(current_seq.clone());

                    if let Some(ref c) = *candidate.lock().unwrap() {
                        if c.len() <= current_seq.len() {
                            return local_new_sequences;
                        }
                    }

                    if current_seq.len() > handling.max {
                        return local_new_sequences;
                    }

                    if self.check_inputs(board, &current_seq, spawn, handling) {
                        let mut candidate_guard = candidate.lock().unwrap();
                        if candidate_guard.is_none()
                            || current_seq.len() < candidate_guard.as_ref().unwrap().len()
                        {
                            *candidate_guard = Some(current_seq.clone());
                            if !handling.finesse {
                                return vec![];
                            }
                        }
                    }

                    for next_move in &possible_moves {
                        let mut new_seq = current_seq.clone();
                        new_seq.push(*next_move);

                        if !visited.contains(&new_seq) {
                            local_new_sequences.push(new_seq);
                        }
                    }

                    local_new_sequences
                })
                .collect();

            queue.extend(new_sequences);

            if candidate.lock().unwrap().is_some() {
                break;
            }
        }

        Arc::try_unwrap(candidate)
            .ok()
            .unwrap()
            .into_inner()
            .unwrap()
    }
}
