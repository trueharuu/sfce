use std::str::FromStr;

use crate::piece::{
    Piece::{self, I, J, L, O, S, T, Z},
    Rotation::{self, East as e, North as n, South as s, West as w},
};

// TODO: add kicktables for SRS, SRS+, SRS-X, SRS-jstris
pub type RawKickset<'a> = &'a [(Piece, Rotation, Rotation, &'a [(isize, isize)])];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Kickset<'a> {
    raw: RawKickset<'a>,
}

impl<'a> Kickset<'a> {
    #[must_use] pub fn raw(raw: RawKickset<'a>) -> Self {
        Self { raw }
    }
    #[must_use] pub fn get(
        &self,
        piece: Piece,
        initial_rotation: Rotation,
        final_rotation: Rotation,
    ) -> &'a [(isize, isize)] {
        self.raw
            .iter()
            .find(|x| x.0 == piece && x.1 == initial_rotation && x.2 == final_rotation)
            .map_or(&[(0, 0)], |x| x.3)
    }

    #[must_use] pub fn none() -> Self {
      Self::raw(NONE)
    }

    #[must_use] pub fn srs() -> Self {
      Self::raw(SRS)
    }
}

impl<'a> FromStr for Kickset<'a> {
    type Err = String;
    fn from_str(z: &str) -> Result<Self, Self::Err> {
        match z.to_ascii_lowercase().as_str() {
            "none" => Ok(Kickset::none()),
            "srs" => Ok(Kickset::srs()),

            c => Err(format!("invalid kick table {c}")),
        }
    }
}

macro_rules! kicks {
  ($($p:ident $l:ident $r:ident $(($q:expr, $v:expr))* ;)*) => {
      &[$(($p, $l, $r, &[$(($q, $v),)*]),)*]
  };
}

pub const NONE: RawKickset = &[];
pub const SRS: RawKickset = kicks!(
  J n e (0, 0) (-1, 0) (-1, 1) (0, -2) (-1, -2);
  J e n (0, 0) (1, 0) (1, -1) (0, 2) (1, 2);
  J e s (0, 0) (1, 0) (1, -1) (0, 2) (1, 2);
  J s e (0, 0) (-1, 0) (-1, 1) (0, -2) (1, -2);
  J s w (0, 0) (1, 0) (1, 1) (0, -2) (1, -2);
  J w s (0, 0) (-1, 0) (-1, -1) (0, 2) (-1, 2);
  J w n (0, 0) (-1, 0) (-1, -1) (0, 2) (-1, 2);
  J n w (0, 0) (1, 0) (1, 1) (0, -2) (1, -2);

  L n e (0, 0) (-1, 0) (-1, 1) (0, -2) (-1, -2);
  L e n (0, 0) (1, 0) (1, -1) (0, 2) (1, 2);
  L e s (0, 0) (1, 0) (1, -1) (0, 2) (1, 2);
  L s e (0, 0) (-1, 0) (-1, 1) (0, -2) (1, -2);
  L s w (0, 0) (1, 0) (1, 1) (0, -2) (1, -2);
  L w s (0, 0) (-1, 0) (-1, -1) (0, 2) (-1, 2);
  L w n (0, 0) (-1, 0) (-1, -1) (0, 2) (-1, 2);
  L n w (0, 0) (1, 0) (1, 1) (0, -2) (1, -2);

  S n e (0, 0) (-1, 0) (-1, 1) (0, -2) (-1, -2);
  S e n (0, 0) (1, 0) (1, -1) (0, 2) (1, 2);
  S e s (0, 0) (1, 0) (1, -1) (0, 2) (1, 2);
  S s e (0, 0) (-1, 0) (-1, 1) (0, -2) (1, -2);
  S s w (0, 0) (1, 0) (1, 1) (0, -2) (1, -2);
  S w s (0, 0) (-1, 0) (-1, -1) (0, 2) (-1, 2);
  S w n (0, 0) (-1, 0) (-1, -1) (0, 2) (-1, 2);
  S n w (0, 0) (1, 0) (1, 1) (0, -2) (1, -2);

  T n e (0, 0) (-1, 0) (-1, 1) (0, -2) (-1, -2);
  T e n (0, 0) (1, 0) (1, -1) (0, 2) (1, 2);
  T e s (0, 0) (1, 0) (1, -1) (0, 2) (1, 2);
  T s e (0, 0) (-1, 0) (-1, 1) (0, -2) (1, -2);
  T s w (0, 0) (1, 0) (1, 1) (0, -2) (1, -2);
  T w s (0, 0) (-1, 0) (-1, -1) (0, 2) (-1, 2);
  T w n (0, 0) (-1, 0) (-1, -1) (0, 2) (-1, 2);
  T n w (0, 0) (1, 0) (1, 1) (0, -2) (1, -2);

  Z n e (0, 0) (-1, 0) (-1, 1) (0, -2) (-1, -2);
  Z e n (0, 0) (1, 0) (1, -1) (0, 2) (1, 2);
  Z e s (0, 0) (1, 0) (1, -1) (0, 2) (1, 2);
  Z s e (0, 0) (-1, 0) (-1, 1) (0, -2) (1, -2);
  Z s w (0, 0) (1, 0) (1, 1) (0, -2) (1, -2);
  Z w s (0, 0) (-1, 0) (-1, -1) (0, 2) (-1, 2);
  Z w n (0, 0) (-1, 0) (-1, -1) (0, 2) (-1, 2);
  Z n w (0, 0) (1, 0) (1, 1) (0, -2) (1, -2);

  O n e (0, 0);
  O e n (0, 0);
  O e s (0, 0);
  O s e (0, 0);
  O s w (0, 0);
  O w s (0, 0);
  O w n (0, 0);
  O n w (0, 0);

  I n e (0, 0) (-2, 0) (1, 0) (-2, -1) (1, 2);
  I e n (0, 0) (2, 0) (-1, 0) (2, 1) (-1, 2);
  I e s (0, 0) (-1, 0) (2, 0) (-1, 2) (2, -1);
  I s e (0, 0) (1, 0) (-2, 0) (1, -2) (-2, 1);
  I s w (0, 0) (2, 0) (-1, 0) (2, 1) (-1, 2);
  I w s (0, 0) (-2, 0) (1, 0) (-2, -1) (1, 2);
  I w n (0, 0) (1, 0) (-2, 0) (1, -2) (-2, 1);
  I n w (0, 0) (-1, 0) (2, 0) (-1, 2) (2, -1);
);
