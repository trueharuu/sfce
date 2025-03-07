use std::{collections::HashSet, str::FromStr};

use fumen::CellColor;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

use crate::{data::placements::PLACEMENTS, input::Key, traits::GetWith};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, Hash, Serialize, Deserialize, EnumIter,
)]
#[strum(ascii_case_insensitive)]
pub enum Piece {
    I,
    J,
    O,
    L,
    Z,
    S,
    T,
    G,
    D,
    E,
}

impl Piece {
    #[must_use]
    pub fn cell_color(self) -> CellColor {
        match self {
            Piece::D | Piece::G => CellColor::Grey,
            Piece::E => CellColor::Empty,
            Piece::I => CellColor::I,
            Piece::J => CellColor::J,
            Piece::O => CellColor::O,
            Piece::L => CellColor::L,
            Piece::Z => CellColor::Z,
            Piece::S => CellColor::S,
            Piece::T => CellColor::T,
        }
    }

    #[must_use]
    pub fn fum(self) -> fumen::PieceType {
        match self {
            Piece::I => fumen::PieceType::I,
            Piece::J => fumen::PieceType::J,
            Piece::O => fumen::PieceType::O,
            Piece::L => fumen::PieceType::L,
            Piece::Z => fumen::PieceType::Z,
            Piece::S => fumen::PieceType::S,
            Piece::T => fumen::PieceType::T,
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub fn is_filled(self) -> bool {
        self != Self::E
    }

    #[must_use]
    pub fn is_filled_with_piece(self) -> bool {
        self != Self::E && self != Self::G && self != Self::D
    }

    #[must_use]
    pub fn offsets<'a>(self, rotation: Rotation) -> &'a [(isize, isize)] {
        PLACEMENTS
            .get_with(|x| x.0 == self && x.1 == rotation)
            .unwrap()
            .2
    }

    #[must_use]
    pub fn cells(self, x: usize, y: usize, rotation: Rotation) -> Option<HashSet<(usize, usize)>> {
        let mut a = HashSet::new();
        for &(ox, oy) in self.offsets(rotation) {
            let dx = x.checked_add_signed(ox)?;
            let dy = y.checked_add_signed(oy)?;

            a.insert((dx, dy));
        }

        Some(a)
    }
}

#[derive(Clone, Copy, Debug, strum::EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rotation {
    North,
    East,
    South,
    West,
}

impl FromStr for Rotation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "up" | "north" | "n" => Ok(Self::North),
            "right" | "east" | "e" => Ok(Self::East),
            "down" | "south" | "s" => Ok(Self::South),
            "left" | "west" | "w" => Ok(Self::West),
            c => Err(format!("unknown rotation {c}")),
        }
    }
}

impl Rotation {
    #[must_use]
    pub fn cw(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    #[must_use]
    pub fn ccw(self) -> Self {
        match self {
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::North => Self::West,
        }
    }

    #[must_use]
    pub fn flip(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    #[must_use]
    pub fn send_one(self, keys: Key) -> Self {
        match keys {
            Key::CCW => self.ccw(),
            Key::CW => self.cw(),
            Key::Flip => self.flip(),

            _ => self,
        }
    }

    #[must_use]
    pub fn send(mut self, keys: &[Key]) -> Self {
        for key in keys { self = self.send_one(*key ) }
        self
    }
}
