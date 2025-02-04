use std::{
    io::Write,
    str::FromStr,
};

use crate::{
    board::Board, grid::Grid, pattern::Pattern, program::Sfce, ranged::Ranged,
};

impl Sfce {
    pub fn test_command(&mut self) -> anyhow::Result<()> {
        let mut file = std::fs::File::create("./res3.txt").unwrap();

        let p = Pattern::new("*").unwrap();
        let r = Ranged::from_str("1").unwrap();
        let mut i = 1;
        for d in Self::gen_residuals(4, 3) {
            let mut board = Board {
                data: d,
                margin: 0,
                ..Default::default()
            };
            board.set_margin(2);
            board.set_height(board.height() + 1);
            let mut g = self.move_placements(&Grid::from_pages([board]), &p, r, false);

            if g.pages().len() > 1 {
                for p in g.pages_mut() {
                    p.comment = Some(format!("{} #{}", p.comment.clone().unwrap_or_default(), i));
                }
                i += 1;
                write!(file, "{},", self.tetfu(&g)).unwrap();
            }
        }
        Ok(())
    }
}
