use std::collections::{HashSet, VecDeque};

use serde::{Deserialize, Serialize};

use crate::{
    board::Board,
    input::{DropType, Input, Key},
    piece::{Piece, Rotation},
    program::Handling,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Placement(Piece, usize, usize, Rotation);

impl Placement {
    #[must_use] pub fn new(piece: Piece, x: usize, y: usize, rotation: Rotation) -> Self {
        Placement(piece, x, y, rotation)
    }

    #[must_use] pub fn piece(&self) -> Piece {
        self.0
    }

    #[must_use] pub fn x(&self) -> usize {
        self.1
    }

    #[must_use] pub fn y(&self) -> usize {
        self.2
    }

    #[must_use] pub fn location(&self) -> (usize, usize) {
        (self.1, self.2)
    }

    #[must_use] pub fn rotation(&self) -> Rotation {
        self.3
    }

    #[must_use] pub fn at(mut self, (x, y): (usize, usize)) -> Self {
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

    #[must_use] pub fn check_inputs(
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

    #[must_use] pub fn is_input_useful(
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

    #[must_use] pub fn is_doable(&self, board: &Board, spawn: (usize, usize), mut handling: Handling) -> bool {
        handling.finesse = false;
        self.inputs(board, spawn, handling).is_some()
    }
}

impl Placement {
    #[must_use] pub fn finesse(
        self,
        board: &Board,
        spawn: (usize, usize),
        mut handling: Handling,
    ) -> Option<Vec<Key>> {
        handling.finesse = true;
        self.inputs(board, spawn, handling)
    }
    #[must_use] pub fn inputs(
        self,
        board: &Board,
        spawn: (usize, usize),
        handling: Handling,
    ) -> Option<Vec<Key>> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back(Vec::new());

        let mut possible_moves = vec![Key::MoveLeft, Key::MoveRight, Key::CW, Key::CCW];

        if handling.das {
            possible_moves.insert(0, Key::DasRight);
            possible_moves.insert(0, Key::DasLeft);
        }

        if handling.use_180 {
            possible_moves.push(Key::Flip);
        }

        if handling.drop_type == DropType::Sonic || handling.drop_type == DropType::Soft {
            possible_moves.push(Key::SonicDrop);
        }

        if handling.drop_type == DropType::Soft {
            possible_moves.push(Key::SoftDrop);
        }

        let mut v2 = HashSet::new();

        let mut candidate: Option<Vec<Key>> = None;
        while let Some(current_seq) = queue.pop_front() {
            if v2.contains(&current_seq) {
                continue;
            }
            v2.insert(current_seq.clone());
            // let i = Input::new(board, self.piece, spawn, Rotation::North, handling);
            // let cs = i.remove_all_noops(&current_seq);
            let cs = current_seq;
            // v2.insert(cs.clone());
            // if i.has_noops(&cs) {
            //     continue;
            // }
            if let Some(ref c) = candidate
                && c.len() < cs.len()
            {
                // println!("{cs:?} is longer than {c:?}!");
                continue;
            }

            if cs.len() > handling.max {
                continue;
            }

            if self.check_inputs(board, &cs, spawn, handling) {
                candidate = Some(cs.clone());

                if handling.finesse {
                    // println!("found first!");
                    return Some(cs);
                }
            }

            for next_move in &possible_moves {
                if !self.is_input_useful(board, &cs, *next_move, spawn, handling) {
                    continue;
                    // println!("deemed {next_move:?} unhelpful with {cs:?}")
                }
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
