use crate::piece::{
    Piece::{self, *},
    Rotation::{self, East as e, North as n, South as s, West as w},
};

macro_rules! placements {
    ($($p:ident $r:ident $(($q:expr, $v:expr))* ;)*) => {
        &[$(($p, $r, &[$(($q, $v),)*]),)*]
    };
}
/// List of grid-filling locations for any given placement of a piece and a rotation.
/// This is defined as the list of cells filled, written as the offsets from the center.
#[allow(clippy::type_complexity)]
pub const PLACEMENTS: &[(Piece, Rotation, &[(isize, isize)])] = placements!(
  I n (0, 0) (-1, 0) (1, 0) (2, 0);
  I e (0, 0) (0, 1) (0, -1) (0, -2);
  I s (0, 0) (-2, 0) (-1, 0) (1, 0);
  I w (0, 0) (0, 2) (0, 1) (0, -1);
  J n (0, 0) (-1, 1) (-1, 0) (1, 0);
  J e (0, 0) (1, 1) (0, 1) (0, -1);
  J s (0, 0) (-1, 0) (1, 0) (1, -1);
  J w (0, 0) (0, 1) (0, -1) (-1, -1);
  O n (0, 0) (0, 1) (1, 0) (1, 1);
  O e (0, 0) (0, -1) (1, 0) (1, -1);
  O s (0, 0) (0, -1) (-1, 0) (-1, -1);
  O w (0, 0) (0, 1) (-1, 0) (-1, 1);
  L n (0, 0) (-1, 0) (1, 0) (1, 1);
  L e (0, 0) (0, 1) (0, -1) (1, -1);
  L s (0, 0) (1, 0) (-1, 0) (-1, -1);
  L w (0, 0) (-1, 1) (0, 1) (0, -1);
  Z n (0, 0) (-1, 1) (0, 1) (1, 0);
  Z e (0, 0) (1, 1) (1, 0) (0, -1);
  Z s (0, 0) (-1, 0) (0, -1) (1, -1);
  Z w (0, 0) (-1, 0) (-1, -1) (0, 1);
  S n (0, 0) (0, 1) (1, 1) (-1, 0);
  S e (0, 0) (0, 1) (1, 0) (1, -1);
  S s (0, 0) (1, 0) (0, -1) (-1, -1);
  S w (0, 0) (-1, 1) (-1, 0) (0, -1);
  T n (0, 0) (0, 1) (-1, 0) (1, 0);
  T e (0, 0) (1, 0) (0, -1) (0, 1);
  T s (0, 0) (0, -1) (-1, 0) (1, 0);
  T w (0, 0) (-1, 0) (0, -1) (0, 1);
);
