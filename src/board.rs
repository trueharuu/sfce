use std::fmt::Display;

use chumsky::Parser;
use fumen::Fumen;
use serde::{Deserialize, Serialize};

use crate::{
    data::placements::PLACEMENTS,
    grid::Grid,
    piece::Piece,
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
                            trials.push(Piece::D)
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

    pub fn skim_place(&mut self, placement: Placement) {
        let binding = self.clone();
        let removed_lines = binding
            .data
            .iter()
            .enumerate()
            .filter(|(_, x)| !x.contains(&Piece::E));
        self.skim();
        self.place(placement);

        for (i, l) in removed_lines {
            self.data.insert(i, l.clone());
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
    pub fn fast(&self) -> Bits {
        let w = self.width();
        let h = self.height();
        let x = self
            .clone()
            .as_deoptimized()
            .data
            .concat()
            .iter()
            .map(|x| x.is_filled())
            .vec();
        Bits(w, h, x)
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
pub struct Bits(usize, usize, Vec<bool>);
