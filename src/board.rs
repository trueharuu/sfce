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
    pub fn fumen(&self) -> Fumen {
        self.grid().fumen()
    }

    pub fn grid(&self) -> Grid {
        Grid(vec![self.clone()])
    }

    pub fn get(&self, x: usize, y: usize) -> Piece {
        self.data
            .get(y)
            .and_then(|y| y.get(x))
            .copied()
            .unwrap_or(Piece::E)
    }

    pub fn fumen_page(&self) -> fumen::Page {
        self.fumen().pages[0].clone()
    }

    pub fn new(t: impl Display) -> Self {
        let s = t.to_string();

        let x = parse::parser().parse(&s).unwrap();
        x
    }

    pub fn spawn(&self) -> (usize, usize) {
        (self.width() / 2, self.total_height() - self.margin)
    }

    pub fn empty(width: usize, height: usize, margin: usize) -> Self {
        Self {
            data: vec![vec![Piece::E; width]; height + margin],
            comment: None,
            margin,
        }
    }

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

    pub fn rows(&self) -> &Vec<Vec<Piece>> {
        &self.data
    }

    pub fn rows_mut(&mut self) -> &mut Vec<Vec<Piece>> {
        &mut self.data
    }

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

    pub fn as_deoptimized(mut self) -> Self {
        let w = self.width();
        for row in self.rows_mut() {
            while row.len() < w {
                row.push(Piece::E);
            }
        }

        self
    }

    pub fn width(&self) -> usize {
        self.rows().iter().map(|x| x.len()).max().unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.rows().len() - self.margin
    }

    pub fn total_height(&self) -> usize {
        self.rows().len()
    }

    pub fn is_valid_placement(&self, placement: Placement, allow_floating: bool) -> bool {
        let s = self.clone().as_deoptimized();
        if let Some(pm) =
            PLACEMENTS.get_with(|x| x.0 == placement.piece() && x.1 == placement.rotation())
        {
            let mut trials = vec![];
            for offset in pm.2 {
                let dy = placement.y() as isize + offset.1;
                let dx = placement.x() as isize + offset.0;

                // if the piece can't fit in the board it's bad
                if !s.is_in_bounds(dx, dy) {
                    return false;
                }

                // if the piece intersects the board it's bad
                if s.data[dy as usize][dx as usize].is_filled() {
                    return false;
                }

                // check the cell below
                if s.is_in_bounds(dx, dy - 1) {
                    let cell_below = s.data[(dy - 1) as usize][dx as usize];
                    trials.push(cell_below);
                } else {
                    trials.push(Piece::D);
                }
            }

            // dbg!(&trials);

            // if the piece is floating (all cells directly below it are empty)
            if !allow_floating && !trials.iter().any(|x| x.is_filled()) {
                return false;
            }

            true
        } else {
            false
        }
    }

    pub fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        (0..self.width() as isize).contains(&x)
            && (0..(self.height() + self.margin) as isize).contains(&y)
    }

    pub fn place(&mut self, placement: Placement) {
        self.deoptimize();
        if let Some(pm) =
            PLACEMENTS.get_with(|x| x.0 == placement.piece() && x.1 == placement.rotation())
        {
            for (ox, oy) in pm.2 {
                let dx = placement.x() as isize + ox;
                let dy = placement.y() as isize + oy;
                if self.is_in_bounds(dx, dy) {
                    self.data[dy as usize][dx as usize] = placement.piece();
                }
            }
        }
    }

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

    pub fn with_skimmed_placement(&self, placement: Placement) -> Self {
        let mut s = self.clone();
        s.skim_place(placement);
        s
    }

    pub fn with_many_placements(&self, placements: &[Placement]) -> Self {
        let mut s = self.clone();
        for p in placements {
            s.skim_place(*p);
        }

        s
    }

    pub fn comment(&self) -> &Option<String> {
        &self.comment
    }

    pub fn set_comment(&mut self, comment: impl Display) {
        self.comment = Some(comment.to_string());
    }

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
            self.rows_mut().insert(h, vec![Piece::E; w])
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

    pub fn skimmed(mut self) -> Self {
        self.skim();
        self
    }

    pub fn line_clears(&self) -> usize {
        self.data.iter().filter(|x| !x.contains(&Piece::E)).count()
    }

    pub fn fast(&self) -> BitBoard {
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
        BitBoard(w, h, x)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data
                .iter()
                .map(|x| x.iter().map(|y| y.to_string()).vec().join(""))
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
                Self::Group(t) => t.into_iter().flat_map(|x| x.expand()).vec(),
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
            .map(|x| x.into_iter().flat_map(|x| x.expand()).vec())
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
pub struct BitBoard(usize, usize, Vec<bool>);
