use std::{ops::Deref, str::FromStr};

use fumen::Fumen;

use crate::{fumen::fumen_to_grid, grid::Grid};

/// Argument type for either a `Fumen` or a direct `Grid`.
#[derive(Clone, Debug)]
pub struct Tetfu(Grid);

impl Deref for Tetfu {
    type Target = Grid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Tetfu {
  #[must_use] pub fn grid(&self) -> Grid {
    self.0.clone()
  }
}

impl FromStr for Tetfu {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "_" {
            return Ok(Self(Grid::empty(10, 23)));
        }

        if s[1..].starts_with("115@") {
            Ok(Self(fumen_to_grid(
                &Fumen::decode(s).map_err(|x| format!("{x}"))?,
            )))
        } else {
            Ok(Self(Grid::new(s)))
        }
    }
}
