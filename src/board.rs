use std::{
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

use chumsky::Parser;
use fumen::Fumen;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{
    data::placements::PLACEMENTS,
    grid::Grid,
    piece::{Piece, Rotation},
    placement::Placement,
    traits::{CollectVec, GetWith},
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Board {
    pub data: Vec<Vec<Piece>>,
    pub comment: Option<String>,
    pub margin: usize,
}

impl Board {
    #[must_use]
    pub fn fumen(&self) -> Fumen {
        self.grid().fumen()
    }

    #[must_use]
    pub fn grid(&self) -> Grid {
        Grid(vec![self.clone()])
    }

    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> Piece {
        self.data
            .get(y)
            .and_then(|y| y.get(x))
            .copied()
            .unwrap_or(Piece::E)
    }

    pub fn set(&mut self, x: usize, y: usize, p: Piece) {
        self.data[y][x] = p;
    }

    #[must_use]
    pub fn fumen_page(&self) -> fumen::Page {
        self.fumen().pages[0].clone()
    }

    pub fn new(t: impl Display) -> Self {
        let s = t.to_string();

        let x = parse::parser().parse(&s).unwrap();
        x
    }

    #[must_use]
    pub fn spawn(&self) -> (usize, usize) {
        (self.spawn_x(), self.spawn_y())
    }

    #[must_use]
    pub fn spawn_x(&self) -> usize {
        self.width() / 2 - 1
    }

    #[must_use]
    pub fn spawn_y(&self) -> usize {
        self.total_height() - self.margin
    }

    #[must_use]
    pub fn empty(width: usize, height: usize, margin: usize) -> Self {
        Self {
            data: vec![vec![Piece::E; width]; height + margin],
            comment: None,
            margin,
        }
    }

    #[must_use]
    pub fn only_gray(self) -> Self {
        Self {
            data: self
                .data
                .into_iter()
                .map(|x| {
                    x.into_iter()
                        .map(|x| {
                            if x == Piece::G || x == Piece::D {
                                x
                            } else {
                                Piece::E
                            }
                        })
                        .vec()
                })
                .vec(),
            comment: self.comment,
            margin: self.margin,
        }
    }

    #[must_use]
    pub fn to_gray(self) -> Self {
        Self {
            data: self
                .data
                .into_iter()
                .map(|x| {
                    x.into_iter()
                        .map(|x| if x == Piece::E { Piece::E } else { Piece::G })
                        .collect()
                })
                .collect(),
            comment: self.comment,
            margin: self.margin,
        }
    }

    #[must_use]
    pub fn rows(&self) -> &Vec<Vec<Piece>> {
        &self.data
    }

    pub fn rows_mut(&mut self) -> &mut Vec<Vec<Piece>> {
        &mut self.data
    }

    #[must_use]
    pub fn optimized(self) -> Self {
        Self {
            data: self
                .data
                .into_iter()
                .skip_while(|x| x.iter().all(|x| *x == Piece::E))
                .map(|x| {
                    x.into_iter()
                        .rev()
                        .skip_while(|x| *x == Piece::E)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .collect::<Vec<_>>()
                })
                .collect(),
            comment: self.comment,
            margin: self.margin,
        }
    }

    pub fn deoptimize(&mut self) {
        let w = self.width();
        for row in self.rows_mut() {
            while row.len() < w {
                row.push(Piece::E);
            }
        }
    }

    #[must_use]
    pub fn as_deoptimized(mut self) -> Self {
        let w = self.width();
        for row in self.rows_mut() {
            while row.len() < w {
                row.push(Piece::E);
            }
        }

        self
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.rows()
            .iter()
            .map(std::vec::Vec::len)
            .max()
            .unwrap_or(0)
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.rows().len() - self.margin
    }

    #[must_use]
    pub fn total_height(&self) -> usize {
        self.rows().len()
    }

    #[must_use]
    pub fn is_valid_placement(&self, placement: Placement, allow_floating: bool) -> bool {
        let s = self.clone().as_deoptimized();
        if let Some(pm) =
            PLACEMENTS.get_with(|x| x.0 == placement.piece() && x.1 == placement.rotation())
        {
            let mut trials = vec![];
            for offset in pm.2 {
                if let (Some(dx), Some(dy)) = (
                    placement.x().checked_add_signed(offset.0),
                    placement.y().checked_add_signed(offset.1),
                ) {
                    // if the piece can't fit in the board it's bad
                    if !s.is_in_bounds(dx, dy) {
                        return false;
                    }

                    // if the piece intersects the board it's bad
                    if s.data[dy][dx].is_filled() {
                        return false;
                    }

                    // check the cell below
                    if let Some(ry) = dy.checked_sub(1) {
                        if s.is_in_bounds(dx, ry) {
                            let cell_below = s.data[dy - 1][dx];
                            trials.push(cell_below);
                        } else {
                            trials.push(Piece::D);
                        }
                    } else {
                        trials.push(Piece::D);
                    }
                } else {
                    return false;
                }
            }

            // dbg!(&trials);

            // if the piece is floating (all cells directly below it are empty)
            if !allow_floating && !trials.iter().any(|x| x.is_filled()) {
                return false;
            }

            if !allow_floating && s.intersects_margin() {
                return false;
            }

            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn is_valid_placement_with_skim(&self, placement: Placement, allow_floating: bool) -> bool {
        self.clone()
            .skimmed()
            .is_valid_placement(placement, allow_floating)
    }

    #[must_use]
    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        (0..self.width()).contains(&x) && (0..self.total_height()).contains(&y)
    }

    #[must_use]
    pub fn is_in_margin(&self, x: usize, y: usize) -> bool {
        (0..self.width()).contains(&x) && (self.height() + 1..).contains(&y)
    }

    pub fn place(&mut self, placement: Placement) {
        self.deoptimize();
        if let Some(pm) =
            PLACEMENTS.get_with(|x| x.0 == placement.piece() && x.1 == placement.rotation())
        {
            for (ox, oy) in pm.2 {
                let dx = placement.x().checked_add_signed(*ox);
                let dy = placement.y().checked_add_signed(*oy);
                if let (Some(dx), Some(dy)) = (dx, dy) {
                    if self.is_in_bounds(dx, dy) {
                        self.data[dy][dx] = placement.piece();
                    }
                }
            }
        }
    }

    #[must_use]
    pub fn with_placement(&self, placement: Placement) -> Self {
        let mut c = self.clone();
        c.place(placement);
        c
    }

    #[must_use]
    pub fn removed_lines(&self) -> Vec<(usize, &Vec<Piece>)> {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, x)| !x.contains(&Piece::E))
            .collect()
    }

    pub fn skim_place(&mut self, placement: Placement) {
        self.deoptimize();
        if let Some(pm) =
            PLACEMENTS.get_with(|x| x.0 == placement.piece() && x.1 == placement.rotation())
        {
            for (ox, oy) in pm.2 {
                let dx = placement.x().checked_add_signed(*ox);
                let dy = placement.y().checked_add_signed(*oy);
                if let (Some(dx), Some(mut dy)) = (dx, dy) {
                    if self.is_in_bounds(dx, dy) {
                        if *oy < 0 {
                            while self.is_cleared(dy) {
                                dy -= 1;
                            }
                        }

                        if *oy > 0 {
                            while self.is_cleared(dy) {
                                dy += 1;
                            }
                        }

                        self.data[dy][dx] = placement.piece();
                    }
                }
            }
        }
    }

    #[must_use]
    pub fn with_skimmed_placement(&self, placement: Placement) -> Self {
        let mut s = self.clone();
        s.skim_place(placement);
        s
    }

    #[must_use]
    pub fn with_many_placements(&self, placements: &[Placement]) -> Self {
        let mut s = self.clone();
        for p in placements {
            s.skim_place(*p);
        }

        s
    }

    #[must_use]
    pub fn intersects_margin(&self) -> bool {
        self.data[self.data.len() - self.margin..]
            .iter()
            .any(|x| x.iter().any(|y| y.is_filled()))
    }

    #[must_use]
    pub fn comment(&self) -> &Option<String> {
        &self.comment
    }

    pub fn set_comment(&mut self, comment: impl Display) {
        self.comment = Some(comment.to_string());
    }

    #[must_use]
    pub fn with_comment(mut self, comment: impl Display) -> Self {
        self.comment = Some(comment.to_string());
        self
    }

    pub fn set_width(&mut self, width: usize) {
        for row in self.rows_mut() {
            row.resize(width, Piece::E);
        }
    }

    pub fn set_height(&mut self, height: usize) {
        while self.height() > height {
            let h = self.height();
            self.rows_mut().remove(h - 1);
        }

        let w = self.width();
        while self.height() < height {
            let h = self.height();
            self.rows_mut().insert(h, vec![Piece::E; w]);
        }
    }

    pub fn set_margin(&mut self, margin: usize) {
        while self.margin > margin {
            self.rows_mut().pop();
            self.margin -= 1;
        }

        let w = self.width();
        while self.margin < margin {
            self.rows_mut().push(vec![Piece::E; w]);
            self.margin += 1;
        }
    }

    pub fn skim(&mut self) {
        let lc = self.line_clears();
        self.data = self
            .data
            .clone()
            .into_iter()
            .filter(|x| x.contains(&Piece::E))
            .vec();

        let mut i = 0;
        let w = self.width();
        while i < lc {
            i += 1;
            self.data.push(vec![Piece::E; w]);
        }
    }

    #[must_use]
    pub fn skimmed(mut self) -> Self {
        self.skim();
        self
    }

    #[must_use]
    pub fn line_clears(&self) -> usize {
        self.data.iter().filter(|x| !x.contains(&Piece::E)).count()
    }

    #[must_use]
    pub fn is_cleared(&self, y: usize) -> bool {
        self.removed_lines().iter().map(|x| x.0).contains(&y)
    }

    #[must_use]
    pub fn fast(&self) -> Bits {
        let width = self.width();
        let height = self.height();
        let bits = self.clone().as_deoptimized().data[..self.height()]
            .concat()
            .iter()
            .map(|x| x.is_filled())
            .vec();
        Bits {
            width,
            height,
            bits,
        }
    }

    #[must_use]
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
}

impl BitOr for Board {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.dimensions(), rhs.dimensions());
        let mut m = self.clone();

        for x in 0..self.width() {
            for y in 0..self.height() {
                let p = rhs.get(x, y);
                if p.is_filled() {
                    m.set(x, y, p);
                }
            }
        }

        m
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|x| x
                    .iter()
                    .map(std::string::ToString::to_string)
                    .vec()
                    .join(""))
                .rev()
                .vec()
                .join("|")
        )
    }
}

mod parse {
    use std::str::FromStr;

    use chumsky::{
        error::Rich,
        prelude::{just, one_of, recursive},
        text, IterParser, Parser,
    };

    use crate::{board::Board, piece::Piece, traits::CollectVec};

    pub enum Part {
        Piece(Piece),
        Group(Vec<Self>),
        Repeat(Box<Self>, usize),
    }

    impl Part {
        pub fn expand(self) -> Vec<Piece> {
            match self {
                Self::Piece(t) => vec![t],
                Self::Group(t) => t.into_iter().flat_map(Part::expand).vec(),
                Self::Repeat(t, n) => t.expand().repeat(n),
            }
        }
    }

    pub fn part_parser<'a>(
    ) -> impl Parser<'a, &'a str, Vec<Part>, chumsky::extra::Err<Rich<'a, char>>> {
        let piece = one_of("IJOLZSTEGDijolzstegd")
            .map(|x: char| Piece::from_str(&x.to_string()))
            .unwrapped()
            .map(Part::Piece);

        let part = recursive(|e| {
            let group = e
                .clone()
                .repeated()
                .at_least(1)
                .collect()
                .delimited_by(just("["), just("]"))
                .map(Part::Group)
                .or(piece);

            let repeat = group
                .or(piece)
                .then(text::int(10).from_str::<usize>().unwrapped())
                .map(|(x, y)| Part::Repeat(Box::new(x), y));

            // repeat.or(group)
            repeat.or(piece)
        });

        let y = part.repeated().collect().boxed();
        y
    }

    pub fn parser<'a>() -> impl Parser<'a, &'a str, Board, chumsky::extra::Err<Rich<'a, char>>> {
        part_parser()
            .map(|x| x.into_iter().flat_map(Part::expand).vec())
            .separated_by(just("|"))
            .collect()
            .map(|x: Vec<Vec<_>>| Board {
                data: x.into_iter().rev().vec(),
                comment: None,
                margin: 0,
            })
    }
}

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

        m.add_back(&rm, false);

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
    pub fn filled_cells(&self) -> Vec<(usize, usize)> {
        self.bits
            .iter()
            .enumerate()
            .filter(|(_, x)| **x)
            .map(|(x, _)| (x % self.width, x / self.width))
            .collect()
    }

    #[must_use]
    pub fn removed_lines(&self) -> Vec<usize> {
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
