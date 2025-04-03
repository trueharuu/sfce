use std::{collections::HashSet, fmt::Write};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    board::Board, board_parser::Tetfu, grid::Grid, pattern::Pattern, piece::Piece,
    placement::Placement, program::Sfce, ranged::Ranged,
};

impl Sfce {
    pub fn move_command(
        &mut self,
        tetfu: &Tetfu,
        pattern: &Pattern,
        clears: Ranged<usize>,
        minimal: bool,
        continuous_clears: Ranged<usize>,
    ) -> anyhow::Result<()> {
        let rs = self.resize(tetfu.grid()).page();

        let mut m = Grid::from_pages([]);
        for q in pattern.queues() {
            for hq in self.hold_queues(&q) {
                let pms = self.all_placements_of_queue(&rs, hq.pieces());

                let mut s = HashSet::new();
                for pm in pms {
                    // println!("{pm:?}");
                    let vv = rs.with_many_placements(&pm);
                    if !clears.contains(&vv.line_clears()) {
                        continue;
                    }
                    if !self.keeps_continuous_clears(&pm, &rs, continuous_clears) {
                        continue;
                    }

                    if !self.is_doable(&rs, &pm) {
                        continue;
                    }

                    // println!("{vv:?}");
                    if minimal && s.contains(&vv.to_string()) {
                        continue;
                    }

                    s.insert(vv.to_string());

                    m.add_page(vv);

                    if self.program.args.raw {
                        writeln!(
                            self.buf,
                            "{}",
                            pm.iter()
                                .map(|x| format!(
                                    "{},{},{},{}",
                                    x.piece(),
                                    x.x(),
                                    x.y(),
                                    x.rotation()
                                ))
                                .join(";")
                        )?;
                    }
                }
            }
        }

        if !self.program.args.raw {
            write!(self.buf, "{}", self.tetfu(&m))?;
        }
        Ok(())
    }

    #[must_use]
    pub fn is_queue_placeable(&self, board: &Board, queue: &[Piece]) -> bool {
        board.fast().empty_cells().len() >= queue.len() * 4
    }

    pub fn keeps_continuous_clears(
        &self,
        placements: &[Placement],
        board: &Board,
        continuous_clears: Ranged<usize>,
    ) -> bool {
        if self.handling().ignore {
            return true;
        }

        let mut bc = board.clone();
        let mut lc = bc.line_clears();

        for pm in placements {
            bc.skim_place(*pm);
            let diff = bc.line_clears() - lc;
            // println!("{diff}");

            if !continuous_clears.contains(&diff) {
                return false;
            }

            lc = bc.line_clears();
        }

        true
    }

    #[must_use]
    pub fn all_placements_of_queue(&self, board: &Board, queue: &[Piece]) -> Vec<Vec<Placement>> {
        if queue.is_empty() {
            return vec![vec![]];
        }

        // i can haz optimizationburger?
        if queue.len() == 1 {
            return self
                .all_placements_of_piece(board, queue[0])
                .iter()
                .map(|x| vec![*x])
                .collect();
        }

        let piece = queue[0];
        let remaining_queue = &queue[1..];
        let placements = self.all_placements_of_piece(board, piece);

        placements
            .into_par_iter()
            .map(|p| {
                let mut s = board.clone();
                s.skim_place(p);

                let sub_placements = self.all_placements_of_queue(&s, remaining_queue);

                sub_placements.into_par_iter().map(move |mut sr| {
                    sr.insert(0, p);
                    sr
                })
            })
            .flatten()
            .collect()
    }

    #[must_use]
    pub fn is_doable(&self, board: &Board, placements: &[Placement]) -> bool {
        if self.handling().ignore {
            return true;
        }

        let mut bc = board.clone();
        // println!("{placements:?}");
        for p in placements {
            let lc = bc.removed_lines().iter().filter(|x| x.0 < p.y()).count();
            let s = bc.clone().skimmed();
            let mut c = *p;
            c.set_y(c.y() - lc);
            if !c.is_doable(&s, s.spawn(), self.handling()) {
                return false;
            }

            bc.skim_place(*p);
        }

        true
    }

    #[must_use]
    pub fn all_placements_of_piece(&self, board: &Board, piece: Piece) -> Vec<Placement> {
        board.fast().all_placements_of_piece(piece)
    }
}
