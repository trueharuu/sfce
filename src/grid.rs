use std::{collections::HashSet, fmt::Display, hash::Hash};

use fumen::Fumen;
use itertools::Itertools;

use crate::{fumen::grid_to_fumen, page::Page, traits::CollectVec};
/// Three-dimensional array, first layer is page, 2nd layer is row, 3rd layer is column
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Grid(pub Vec<Page>);

impl Grid {
    pub fn to_gray(self) -> Self {
        Self(self.0.into_iter().map(|x| x.to_gray()).collect())
    }

    pub fn new(str: impl Display) -> Self {
        let s = str.to_string();
        let z = s.split(';').map(Page::new).collect();
        Self(z)
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self(vec![Page::empty(width, height)])
    }

    pub fn optimized(self) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|x| x.optimized())
                .collect::<Vec<_>>(),
        )
    }

    pub fn pages(&self) -> &Vec<Page> {
        &self.0
    }

    pub fn pages_mut(&mut self) -> &mut Vec<Page> {
        &mut self.0
    }

    pub fn fumen(&self) -> Fumen {
        grid_to_fumen(self.clone())
    }

    pub fn width(&self) -> usize {
        self.pages().iter().map(|x| x.width()).max().unwrap_or(0)
    }

    pub fn height(&self) -> usize {
        self.pages().iter().map(|x| x.height()).max().unwrap_or(0)
    }

    pub fn add_page(&mut self, page: Page) {
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
        dedup_by(&mut self.0, |x, y| x.0 == y.0)
    }

    pub fn dedup_by_comments(&mut self) {
        dedup_by(&mut self.0, |x, y| x.1 == y.1);
    }

    pub fn dedup(&mut self) {
      dedup_by(&mut self.0, |x, y| x == y);
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
            self.pages().iter().map(|x| x.to_string()).vec().join(";")
        )
    }
}

impl Extend<Page> for Grid {
    fn extend<T: IntoIterator<Item = Page>>(&mut self, iter: T) {
        for i in iter {
            self.add_page(i);
        }
    }
}
