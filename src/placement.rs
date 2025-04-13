use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;
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
        if Rotation::North.send(keys) != self.rotation() {
            return false;
        }
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

    #[must_use]
    pub fn cells(&self) -> Option<HashSet<(usize, usize)>> {
        self.piece().cells(self.x(), self.y(), self.rotation())
    }
}

impl Display for Placement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{},{}", self.piece(), self.x(), self.y(), self.rotation())
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
        let valid_keys = handling.possible_moves();

        for keys in (1..=handling.max).flat_map(|n| valid_keys.clone().into_iter().permutations(n))
        {
            let c = self.check_inputs(board, &keys, spawn, handling.clone());
            if c {
                // println!("\x1b[32m! {keys:?}\x1b[0m");
                return Some(keys);
            } else {
                // println!("\x1b[31mX {keys:?}\x1b[0m");
            }
        }

        return None;
    }
}
