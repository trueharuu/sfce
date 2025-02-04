use std::{
    fmt::Write,
    sync::{Arc, Mutex},
};

use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    board::Board, board_parser::Tetfu, grid::Grid, pattern::Pattern, piece::Piece,
    placement::Placement, program::Sfce, ranged::Ranged, traits::FullyDedupParallel,
};

impl Sfce {
    pub fn move_command(
        &mut self,
        tetfu: &Tetfu,
        pattern: &Pattern,
        clears: Ranged<usize>,
        minimal: bool,
    ) -> anyhow::Result<()> {
        let rs = self.resize(tetfu.grid());
        let pages = self.move_placements(&rs, pattern, clears, minimal);
        write!(self.buf, "{}", self.tetfu(&pages))?;

        Ok(())
    }

    #[must_use]
    pub fn move_placements(
        &self,
        rs: &Grid,
        pattern: &Pattern,
        clears: Ranged<usize>,
        minimal: bool,
    ) -> Grid {
        let pgs = rs.pages();
        let pages = Arc::new(Mutex::new(Grid::default()));
        for board in pgs {
            // pages
            //     .lock()
            //     .unwrap()
            //     .add_page(board.clone().with_comment(String::new()));
            pattern.queues().par_iter().for_each(|queue| {
                if !self.is_queue_placeable(board, queue.pieces()) {
                    return;
                }

                self.hold_queues(queue.clone()).par_iter().for_each(|h| {
                    let apoq = self.all_placements_of_queue(board, h.pieces());

                    let ap = apoq
                        .par_iter()
                        .map(|x| {
                            (
                                x,
                                board
                                    .with_many_placements(x)
                                    .with_comment(format!("{queue}")),
                            )
                        })
                        .filter(|x| clears.contains(&x.1.line_clears()))
                        .fully_dedup_by(|(x, _), (y, _)| {
                            minimal
                                && x.iter()
                                    .map(Placement::piece)
                                    .eq(y.iter().map(Placement::piece))
                        })
                        .filter(|x| self.is_doable(board, x.0))
                        .fully_dedup_by_key(|x| x.1.clone())
                        .map(|x| x.1)
                        .collect::<Vec<_>>();

                    pages.lock().unwrap().extend(ap);
                });
            });
        }

        pages.lock().unwrap().dedup_by_board();

        Arc::into_inner(pages).unwrap().into_inner().unwrap()
    }

    #[must_use]
    pub fn is_queue_placeable(&self, board: &Board, queue: &[Piece]) -> bool {
        board.fast().empty_cells().len() >= queue.len() * 4
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn percent_command(
        &mut self,
        tetfu: &Tetfu,
        pattern: &Pattern,
        clears: Ranged<usize>,
    ) -> anyhow::Result<()> {
        let board = self
            .resize(tetfu.grid())
            .pages()
            .last()
            .cloned()
            .unwrap_or_default();

        let s = Arc::new(Mutex::new(vec![]));
        let f = Arc::new(Mutex::new(vec![]));

        let z = pattern.queues();
        z.par_iter().for_each(|q| {
            if !self.is_queue_placeable(&board, q.pieces()) {
                f.lock().unwrap().push(q);
                return;
            }

            let h = self.hold_queues(q.clone());

            let success = h.par_iter().any(|queue| {
                let apoq = self.all_placements_of_queue(&board, queue.pieces());

                apoq.par_iter()
                    .map(|x| (x, board.with_many_placements(x).with_comment(queue)))
                    .filter(|x| clears.contains(&x.1.line_clears()))
                    .any(|x| self.is_doable(&board, x.0))
            });

            if success {
                s.lock().unwrap().push(q);
            } else {
                f.lock().unwrap().push(q);
            }
        });

        let fs = Arc::try_unwrap(s).unwrap().into_inner().unwrap();
        let ff = Arc::try_unwrap(f).unwrap().into_inner().unwrap();

        writeln!(
            self.buf,
            "{}/{} queues ({:.2}%)",
            fs.len(),
            fs.len() + ff.len(),
            100.0 * fs.len() as f64 / (fs.len() + ff.len()) as f64
        )?;
        write!(self.buf, "fail queues:")?;
        for (i, &q) in ff.iter().enumerate() {
            if i % self.program.args.pw == 0 {
                writeln!(self.buf)?;
            }
            write!(self.buf, "{q} ")?;
        }

        writeln!(self.buf)?;
        Ok(())
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
        // println!("{bc}");
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
