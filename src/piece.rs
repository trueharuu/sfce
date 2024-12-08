use fumen::CellColor;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString, Hash, Serialize, Deserialize)]
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
    #[must_use] pub fn cell_color(self) -> CellColor {
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

    #[must_use] pub fn fum(self) -> fumen::PieceType {
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

    #[must_use] pub fn is_filled(self) -> bool {
        self != Self::E
    }

    #[must_use] pub fn is_filled_with_piece(self) -> bool {
        self != Self::E && self != Self::G && self != Self::D
    }
}

#[derive(Clone, Copy, Debug, strum::EnumIter, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rotation {
    North,
    East,
    South,
    West,
}

impl Rotation {
    #[must_use] pub fn cw(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    #[must_use] pub fn ccw(self) -> Self {
        match self {
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::North => Self::West,
        }
    }

    #[must_use] pub fn flip(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}
