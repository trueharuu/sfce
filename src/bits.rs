use std::{collections::HashSet, ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not}};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{board::Board, piece::{Piece, Rotation}, placement::Placement};


#[derive(Hash, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Bits {
    pub width: usize,
    pub height: usize,
    pub bits: Vec<bool>,
}

impl Not for Bits {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for i in self.bits.as_mut_slice() {
            *i = !*i;
        }

        self
    }
}

impl BitAnd for Bits {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        assert_eq!(self.dimensions(), rhs.dimensions());
        let mut m = self.clone();

        for (a, b) in m.bits.iter_mut().zip(rhs.bits.iter()) {
            *a &= b;
        }

        m
    }
}

impl BitOr for Bits {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.dimensions(), rhs.dimensions());
        let mut m = self.clone();

        for (a, b) in m.bits.iter_mut().zip(rhs.bits.iter()) {
            *a |= b;
        }

        m
    }
}

impl BitAndAssign for Bits {
    fn bitand_assign(&mut self, rhs: Self) {
        assert_eq!(self.dimensions(), rhs.dimensions());

        for (a, b) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *a &= b;
        }
    }
}

impl BitOrAssign for Bits {
    fn bitor_assign(&mut self, rhs: Self) {
        assert_eq!(self.dimensions(), rhs.dimensions());

        for (a, b) in self.bits.iter_mut().zip(rhs.bits.iter()) {
            *a |= b;
        }
    }
}

impl Bits {
    #[must_use]
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> bool {
        self.bits.get(self.index(x, y)).copied().unwrap_or(false)
    }

    #[must_use]
    pub fn has(&self, x: usize, y: usize) -> bool {
        (..self.width).contains(&x) && (..self.height).contains(&y)
    }

    #[must_use]
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut bool {
        &mut self.bits[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, b: bool) {
        self.bits[y * self.width + x] = b;
    }

    #[must_use]
    pub const fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[must_use]
    pub fn shift_up(&self) -> Self {
        let mut s = self.clone();
        s.bits.drain((self.height - 1) * self.width..);
        s.bits.splice(0..0, vec![true; self.width]);
        s
    }

    #[must_use]
    pub fn shift_right(&self) -> Self {
        let mut s = self.clone();
        for row in (0..self.height).rev() {
            s.bits.remove((row + 1) * self.width - 1);
            s.bits.insert(row * self.width, true);
        }

        s
    }

    #[must_use]
    pub fn shift_down(&self) -> Self {
        let mut s = self.clone();
        s.bits.drain(0..self.width);
        s.bits.extend(vec![true; self.width]);
        s
    }

    #[must_use]
    pub fn shift_left(&self) -> Self {
        let mut s = self.clone();
        for row in 0..s.height {
            s.bits.remove(row * s.width);
            s.bits.insert((row + 1) * s.width - 1, true);
        }

        s
    }

    #[must_use]
    pub fn shift_for(&self, mut x: isize, mut y: isize) -> Self {
        let mut s = self.clone();
        while x < 0 {
            x += 1;
            s = s.shift_right();
        }

        while x > 0 {
            x -= 1;
            s = s.shift_left();
        }

        while y < 0 {
            y += 1;
            s = s.shift_up();
        }

        while y > 0 {
            y -= 1;
            s = s.shift_down();
        }

        s
    }

    #[must_use]
    pub fn tint(self, piece: Piece) -> Board {
        Board {
            margin: 0,
            comment: None,
            data: self
                .bits
                .iter()
                .chunks(self.width)
                .into_iter()
                .map(|x| x.map(|y| if *y { piece } else { Piece::E }).collect())
                .collect(),
        }
    }

    #[must_use]
    pub fn board(self) -> Board {
        self.tint(Piece::G)
    }

    #[must_use]
    pub fn possible_placements(&self, piece: Piece, rotation: Rotation) -> Self {
        let mut s = self.clone();
        let rm = s.removed_lines();
        s.skim();

        let o = piece.offsets(rotation);
        let mut m = !o
            .iter()
            .map(|&(x, y)| s.shift_for(x, y))
            .fold(s.clone(), |x, y| x | y);

        for x in 0..s.width {
            for y in 0..s.height {
                if !o.iter().map(|(ox, oy)| (*ox, oy - 1)).any(|(ox, oy)| {
                    let zx = x.checked_add_signed(ox);
                    let zy = y.checked_add_signed(oy);
                    if zy.is_none() {
                        return true;
                    }
                    if zx.is_none() {
                        return false;
                    }
                    !s.has(zx.unwrap(), zy.unwrap()) || s.get(zx.unwrap(), zy.unwrap())
                }) {
                    m.set(x, y, false);
                }
            }
        }

        m.add_back(&rm.into_iter().collect_vec(), false);

        m
    }

    

    #[must_use]
    pub fn all_placements_of_piece(&self, piece: Piece) -> Vec<Placement> {
        // dbg!(self.height);
        let mut m = vec![];
        for rotation in Rotation::iter() {
            m.extend(
                self.possible_placements(piece, rotation)
                    .filled_cells()
                    .iter()
                    .map(|&(x, y)| Placement::new(piece, x, y, rotation)),
            );
        }

        m
    }

    #[must_use]
    pub fn filled_cells(&self) -> HashSet<(usize, usize)> {
        self.bits
            .iter()
            .enumerate()
            .filter(|(_, x)| **x)
            .map(|(x, _)| (x % self.width, x / self.width))
            .collect()
    }

    #[must_use]
    pub fn empty_cells(&self) -> HashSet<(usize, usize)> {
        self.bits
            .iter()
            .enumerate()
            .filter(|(_, x)| !**x)
            .map(|(x, _)| (x % self.width, x / self.width))
            .collect()
    }

    #[must_use]
    pub fn removed_lines(&self) -> HashSet<usize> {
        self.bits
            .chunks_exact(self.width)
            .enumerate()
            .filter(|(_, x)| !x.contains(&false))
            .map(|x| x.0)
            .collect()
    }

    pub fn skim(&mut self) {
        let lcs = self.removed_lines().len();
        self.height -= lcs;

        self.bits = self
            .bits
            .chunks(self.width)
            .filter(|x| x.contains(&false))
            .flatten()
            .copied()
            .collect();
    }

    pub fn add_back(&mut self, l: &[usize], a: bool) {
        self.height += l.len();
        for i in l {
            let len = i * self.width;
            self.bits.splice(len..len, vec![a; self.width]);
        }
    }
}
