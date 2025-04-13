use std::{
    fmt::Write as _,
    sync::{Arc, Mutex},
};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    board::Board, board_parser::Tetfu, grid::Grid, pattern::Pattern, piece::Piece,
    placement::Placement, program::Sfce, ranged::Ranged, traits::FullyDedup,
};

impl Sfce {
    pub fn move_command(
        &mut self,
        tetfu: Tetfu,
        pattern: Pattern,
        total_line_clears: Ranged<usize>,
        continuous_line_clears: Ranged<usize>,
    ) -> anyhow::Result<()> {
        let b = self.resize(tetfu.grid()).page();
        let p = pattern.queues();
        let m = Arc::new(Mutex::new(vec![]));
        p.into_par_iter().for_each(|q| {
            self.hold_queues(&q).into_par_iter().for_each(|h| {
                println!("{h}");
                self.all_placements_of_queue(&b, h.pieces(), continuous_line_clears)
                    .into_par_iter()
                    .for_each(|p| {
                        
                        let mv = b.with_many_placements(&p).with_comment(format!("{q} -> {h}"));
                        // println!("{p:?} {mv}");
                        if total_line_clears.contains(&mv.line_clears())
                            && self.is_many_doable(&b, &p)
                        {
                            m.lock().unwrap().push((p, mv))
                        }
                    })
            });
        });

        if m.lock().unwrap().is_empty() {
            anyhow::bail!("no placements found");
        }

        if !self.program.args.raw {
            write!(
                self.buf,
                "{}",
                self.tetfu(&Grid::from_pages(
                    m.lock().unwrap().iter().map(|x| x.1.clone()).fully_dedup()
                ))
            )?;
        } else {
            write!(
                self.buf,
                "{}",
                m.lock()
                    .unwrap()
                    .iter()
                    .fully_dedup_by_key(|x| x.1.to_string())
                    .map(|x| x.0.iter().map(|x| x.to_string()).join(";"))
                    .join("\n")
            )?;
        }
        Ok(())
    }

    pub fn is_many_doable(&self, board: &Board, placements: &[Placement]) -> bool {
        let mut c = board.clone();
        for p in placements {
            if !p.is_doable(&c, c.spawn(), self.handling()) {
                return false;
            }

            c.place(*p);
        }

        true
    }

    pub fn keeps_continuous_clears(
        &self,
        board: &Board,
        placements: &[Placement],
        continuous_line_clears: Ranged<usize>,
    ) -> bool {
        let mut c = board.clone();
        let mut fulfilled = c.line_clears();
        for p in placements {
            c.place(*p);
            let diff = c.line_clears() - fulfilled;
            if !continuous_line_clears.contains(&diff) {
                return false;
            }

            fulfilled = c.line_clears();
        }

        true
    }

    pub fn all_placements_of_queue(
        &self,
        board: &Board,
        queue: &[Piece],
        cls: Ranged<usize>,
    ) -> Vec<Vec<Placement>> {
        if queue.is_empty() {
            return vec![vec![]];
        }

        // i can haz optimizationburger?
        if queue.len() == 1 {
            return self
                .all_placements_of_piece(board, queue[0], cls)
                .iter()
                .map(|x| vec![*x])
                .collect();
        }

        let piece = queue[0];
        let remaining_queue = &queue[1..];
        let placements = self.all_placements_of_piece(board, piece, cls);

        placements
            .into_iter()
            .map(|p| {
                let mut s = board.clone();
                s.place(p);

                let sub_placements = self.all_placements_of_queue(&s, remaining_queue, cls);

                sub_placements.into_iter().map(move |mut sr| {
                    sr.insert(0, p);
                    sr
                })
            })
            .flatten()
            .collect()
    }

    pub fn all_placements_of_piece(
        &self,
        board: &Board,
        piece: Piece,
        continuous_line_clears: Ranged<usize>,
    ) -> Vec<Placement> {
        board
            .fast()
            .all_placements_of_piece(piece)
            .into_iter()
            .filter(|x| {
                continuous_line_clears
                    .contains(&(board.with_placement(*x).line_clears() - board.line_clears()))
            })
            .collect()
    }
}
