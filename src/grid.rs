use std::fmt::Display;

use fumen::Fumen;

use crate::{board::Board, fumen::grid_to_fumen, traits::CollectVec};
/// Three-dimensional array, first layer is page, 2nd layer is row, 3rd layer is column
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Grid(pub Vec<Board>);

impl Grid {
    #[must_use]
    pub fn to_gray(self) -> Self {
        Self(
            self.0
                .into_iter()
                .map(super::board::Board::to_gray)
                .collect(),
        )
    }

    pub fn new(str: impl Display) -> Self {
        let s = str.to_string();
        let z = s.split(';').map(Board::new).collect();
        Self(z)
    }

    #[must_use]
    pub fn empty(width: usize, height: usize) -> Self {
        Self(vec![Board::empty(width, height, 0)])
    }

    #[must_use]
    pub fn optimized(self) -> Self {
        Self(
            self.0
                .into_iter()
                .map(super::board::Board::optimized)
                .collect::<Vec<_>>(),
        )
    }

    #[must_use]
    pub fn as_deoptimized(mut self) -> Self {
        let w = self.width();
        let h = self.height();
        for page in self.pages_mut() {
            page.set_height(h);
            page.set_width(w);
        }

        self
    }

    #[must_use]
    pub fn pages(&self) -> &Vec<Board> {
        &self.0
    }

    pub fn pages_mut(&mut self) -> &mut Vec<Board> {
        &mut self.0
    }

    #[must_use]
    pub fn fumen(&self) -> Fumen {
        grid_to_fumen(self)
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.pages()
            .iter()
            .map(super::board::Board::width)
            .max()
            .unwrap_or(0)
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.pages()
            .iter()
            .map(super::board::Board::height)
            .max()
            .unwrap_or(0)
    }

    pub fn add_page(&mut self, page: Board) {
        self.0.push(page);
    }

    pub fn set_width(&mut self, width: usize) {
        for page in self.pages_mut() {
            page.set_width(width);
        }
    }

    pub fn set_height(&mut self, width: usize) {
        for page in self.pages_mut() {
            page.set_height(width);
        }
    }

    pub fn dedup_by_board(&mut self) {
        dedup_by(&mut self.0, |x, y| x.data == y.data);
    }

    pub fn dedup_by_comments(&mut self) {
        dedup_by(&mut self.0, |x, y| x.comment == y.comment);
    }

    pub fn dedup(&mut self) {
        dedup_by(&mut self.0, |x, y| x == y);
    }

    pub fn set_margin(&mut self, margin: usize) {
        for page in self.pages_mut() {
            page.set_margin(margin);
        }
    }
}

fn dedup_by<T>(v: &mut Vec<T>, by: impl Fn(&T, &T) -> bool) {
    let len = v.len();

    if len < 2 {
        return;
    }

    for i in (1..len).rev() {
        if v[0..i].iter().any(|x| by(x, &v[i])) {
            v.remove(i);
        }
    }
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.pages()
                .iter()
                .map(std::string::ToString::to_string)
                .vec()
                .join(";")
        )
    }
}

impl Extend<Board> for Grid {
    fn extend<T: IntoIterator<Item = Board>>(&mut self, iter: T) {
        for i in iter {
            self.add_page(i);
        }
    }
}
