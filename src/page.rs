use std::fmt::Display;

use chumsky::Parser;
use fumen::Fumen;

use crate::{
    data::placements::PLACEMENTS,
    grid::Grid,
    piece::{Piece, Placement},
    traits::{CollectVec, GetWith},
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Page(pub Vec<Vec<Piece>>, pub Option<String>);

impl Page {
    pub fn fumen(&self) -> Fumen {
        self.grid().fumen()
    }
    pub fn grid(&self) -> Grid {
        Grid(vec![self.clone()])
    }
    pub fn fumen_page(&self) -> fumen::Page {
        self.fumen().pages[0].clone()
    }
    pub fn new(t: impl Display) -> Self {
        let s = t.to_string();

        let x = parse::parser().parse(&s).unwrap();
        x
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self(vec![vec![Piece::E; width]; height], None)
    }

    pub fn to_gray(self) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|x| {
                    x.into_iter()
                        .map(|x| if x == Piece::E { Piece::E } else { Piece::G })
                        .collect()
                })
                .collect(),
            self.1,
        )
    }

    pub fn rows(&self) -> &Vec<Vec<Piece>> {
        &self.0
    }

    pub fn rows_mut(&mut self) -> &mut Vec<Vec<Piece>> {
        &mut self.0
    }

    pub fn optimized(self) -> Self {
        Self(
            self.0
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
            self.1,
        )
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
        self.rows().len()
    }

    pub fn is_valid_placement(&self, placement: Placement) -> bool {
        let s = self.clone().as_deoptimized();
        if let Some(pm) =
            PLACEMENTS.get_with(|x| x.0 == placement.piece && x.1 == placement.rotation)
        {
            let mut trials = vec![];
            for offset in pm.2 {
                let dy = placement.y as isize + offset.1;
                let dx = placement.x as isize + offset.0;

                // if the piece can't fit in the board it's bad
                if !s.is_in_bounds(dx, dy) {
                    return false;
                }

                // if the piece intersects the board it's bad
                if s.0[dy as usize][dx as usize].is_filled() {
                    return false;
                }

                // check the cell below
                if s.is_in_bounds(dx, dy - 1) {
                    let cell_below = s.0[(dy - 1) as usize][dx as usize];
                    trials.push(cell_below);
                } else {
                    trials.push(Piece::D);
                }
            }

            // dbg!(&trials);

            // if the piece is floating (all cells directly below it are empty)
            if !trials.iter().any(|x| x.is_filled()) {
                return false;
            }

            true
        } else {
            false
        }
    }

    pub fn is_in_bounds(&self, x: isize, y: isize) -> bool {
        (0..self.width() as isize).contains(&x) && (0..self.height() as isize).contains(&y)
    }

    pub fn place(&mut self, placement: Placement) {
        self.deoptimize();
        if let Some(pm) =
            PLACEMENTS.get_with(|x| x.0 == placement.piece && x.1 == placement.rotation)
        {
            for (ox, oy) in pm.2 {
                let dx = placement.x as isize + ox;
                let dy = placement.y as isize + oy;
                if self.is_in_bounds(dx, dy) {
                    self.0[dy as usize][dx as usize] = placement.piece;
                }
            }
        }
    }

    pub fn with_placement(&self, placement: Placement) -> Self {
        let mut c = self.clone();
        c.place(placement);
        c
    }

    pub fn comment(&self) -> &Option<String> {
        &self.1
    }

    pub fn set_comment(&mut self, comment: impl Display) {
        self.1 = Some(comment.to_string());
    }

    pub fn with_comment(mut self, comment: impl Display) -> Self {
        self.1 = Some(comment.to_string());
        self
    }

    pub fn set_width(&mut self, width: usize) {
        for row in self.rows_mut() {
            row.resize(width, Piece::E);
        }
    }

    pub fn set_height(&mut self, height: usize) {
        while self.height() > height {
            self.rows_mut().remove(0);
        }

        let w = self.width();
        while self.height() < height {
            self.rows_mut().insert(0, vec![Piece::E; w])
        }
    }

    pub fn skim(&mut self) {
        self.0 = self
            .0
            .clone()
            .into_iter()
            .filter(|x| x.contains(&Piece::E))
            .vec()
    }

    pub fn skimmed(mut self) -> Self {
        self.skim();
        self
    }

    pub fn line_clears(&self) -> usize {
        self.0.iter().filter(|x| !x.contains(&Piece::E)).count()
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
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

    use crate::{page::Page, piece::Piece, traits::CollectVec};

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

    pub fn parser<'a>() -> impl Parser<'a, &'a str, Page, chumsky::extra::Err<Rich<'a, char>>> {
        part_parser()
            .map(|x| x.into_iter().flat_map(|x| x.expand()).vec())
            .separated_by(just("|"))
            .collect()
            .map(|x: Vec<Vec<_>>| Page(x.into_iter().rev().vec(), None))
    }
}
