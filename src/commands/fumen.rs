use std::{fmt::Write, str::FromStr};

use crate::{
    board_parser::Tetfu,
    grid::Grid,
    program::{FumenCli, Sfce},
};

impl Sfce {
    pub fn fumen_command(&mut self, l: FumenCli) -> anyhow::Result<()> {
        match l {
            FumenCli::Encode { grid } => writeln!(self.buf, "{}", {
                if self.program.args.link_type.is_none() {
                    self.program.args.link_type = Some('v');
                }
                self.tetfu(&Grid::new(grid.contents().grid()))
            })?,
            FumenCli::Decode { fumen } => {
                writeln!(self.buf, "{}", self.resize(fumen.contents().grid()))?;
            }

            FumenCli::Glue { fumen } => {
                let grids = fumen
                    .contents()
                    .split(',')
                    .filter(|x| !x.is_empty())
                    .map(Tetfu::from_str)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|x| anyhow::anyhow!("{x}"))?;

                let mut fum = Grid::default();
                for grid in grids {
                    for page in grid.pages() {
                        fum.add_page(page.clone());
                    }
                }

                writeln!(self.buf, "{}", self.tetfu(&fum))?;
            }

            FumenCli::Optimize { .. } => {
                // todo
            }
        };
        Ok(())
    }
}
