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

    pub fn is_doable(&self, board: Board, spawn: (usize, usize), max: usize) -> bool {
        self.inputs(board, spawn, max, true).is_some()
    }
}

impl Placement {
    pub fn inputs(
        self,
        board: Board,
        spawn: (usize, usize),
        max: usize,
        stop_at_first: bool,
    ) -> Option<Vec<Key>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(Vec::new());

        let possible_moves = [
            Key::MoveLeft,
            Key::MoveRight,
            Key::DasLeft,
            Key::DasRight,
            Key::RotateCW,
            Key::RotateCCW,
            Key::Flip,
            Key::SoftDrop,
            Key::SonicDrop,
        ];

        let mut v2 = HashSet::new();

        let mut candidate: Option<Vec<Key>> = None;
        while let Some(current_seq) = queue.pop_front() {
            if v2.contains(&current_seq) {
                continue;
            }
            v2.insert(current_seq.clone());
            let i = Input::new(board.clone(), self.piece, spawn);
            let cs = i.remove_all_noops(&current_seq);
            v2.insert(cs.clone());
            // if i.has_noops(&cs) {
            //     continue;
            // }
            if let Some(ref c) = candidate
                && c.len() < cs.len()
            {
                continue;
            }

            if cs.len() > max {
                continue;
            }

            if self.check_inputs(board.clone(), &cs, spawn) {
                candidate = Some(cs.clone());

                if stop_at_first {
                    return Some(cs);
                }
            }

            for next_move in possible_moves.iter() {
                let mut ns = cs.clone();
                ns.push(*next_move);

                if !visited.contains(&ns) {
                    visited.insert(ns.clone());
                    queue.push_back(ns);
                }
            }
        }

        candidate
    }
}
