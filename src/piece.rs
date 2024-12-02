use fumen::CellColor;
use strum::{Display, EnumString};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, EnumString)]
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

    pub fn is_filled(self) -> bool {
        self != Self::E
    }
}

#[derive(Clone, Copy, Debug, strum::EnumIter, PartialEq)]
pub enum Rotation {
    North,
    East,
    South,
    West,
}

impl Rotation {
    pub fn cw(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn ccw(self) -> Self {
        match self {
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::North => Self::West,
        }
    }

    pub fn flip(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Placement {
    pub x: usize,
    pub y: usize,
    pub rotation: Rotation,
    pub piece: Piece,
}
