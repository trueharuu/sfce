use std::{fmt::Write, str::FromStr};

use fumen::Fumen;

use crate::{
    board_parser::Tetfu,
    grid::Grid,
    program::{FumenCli, Sfce},
};

pub fn fumen_command(f: &mut Sfce, l: FumenCli) -> anyhow::Result<()> {
    match l {
        FumenCli::Encode { grid } => write!(f.buf, "{}", {
            if f.args.link_type.is_none() {
                f.args.link_type = Some('v');
            }
            f.tetfu(Grid::new(grid.contents()))
        })?,
        FumenCli::Decode { fumen } => write!(
            f.buf,
            "{}",
            f.resize(crate::fumen::fumen_to_grid(Fumen::decode(&fumen)?))
        )?,

        FumenCli::Glue { fumen } => {
            let grids = fumen
                .contents()
                .split(",")
                .map(Tetfu::from_str)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|x| anyhow::anyhow!("{x}"))?;

            let mut fum = Grid::default();
            for grid in grids {
                for page in grid.pages() {
                    fum.add_page(page.clone());
                }
            }

            write!(f.buf, "{}", f.tetfu(fum))?;
        }
    };
    Ok(())
}
