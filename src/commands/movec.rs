use std::{
    fmt::Write,
    sync::{Arc, Mutex},
};

use indicatif::ProgressStyle;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use strum::IntoEnumIterator;

use crate::{
    board::Board,
    board_parser::Tetfu,
    grid::Grid,
    pattern::Pattern,
    piece::{Piece, Rotation},
    placement::Placement,
    program::Sfce,
    ranged::Ranged,
    traits::FullyDedupParallel,
};

impl Sfce {
    pub fn move_command(
        &mut self,
        tetfu: &Tetfu,
        pattern: &Pattern,
        clears: Ranged<usize>,
        minimal: bool,
    ) -> anyhow::Result<()> {
        let board = self
            .resize(tetfu.grid())
            .pages()
            .last()
            .cloned()
            .unwrap_or_default();

        let pages = Arc::new(Mutex::new(Grid::default()));
        let qbar = indicatif::ProgressBar::new(pattern.queues().len() as u64);
        let style = ProgressStyle::default_spinner()
            .template("{spinner} {prefix} {msg:.bold} [{pos}/{len}]")
            .unwrap()
            .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
        qbar.set_prefix("Generating placements for queue");
        qbar.set_style(style);

        pattern.queues().par_iter().for_each(|queue| {
            qbar.set_message(
                queue
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<String>(),
            );
            qbar.inc(1);

            let apoq = self.all_placements_of_queue(&board, queue.pieces());

            let ap = apoq
                .par_iter()
                .map(|x| (x, board.with_many_placements(x).with_comment(queue)))
                .filter(|x| !x.1.intersects_margin())
                .filter(|x| clears.contains(&x.1.line_clears()))
                .fully_dedup_by(|(x, _), (y, _)| {
                    minimal
                        && x.iter()
                            .map(Placement::piece)
                            .eq(y.iter().map(Placement::piece))
                })
                .fully_dedup_by_key(|x| x.1.clone())
                .filter(|x| self.is_doable(&board, x.0))
                .map(|x| x.1)
                .collect::<Vec<_>>();

            pages.lock().unwrap().extend(ap);
        });

        pages.lock().unwrap().dedup_by_board();
        write!(self.buf, "{}", self.tetfu(&pages.lock().unwrap()))?;

        Ok(())
    }

    #[must_use]
    pub fn all_placements_of_queue(&self, board: &Board, queue: &[Piece]) -> Vec<Vec<Placement>> {
        if queue.is_empty() {
            return vec![vec![]];
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

        let mut s = board.clone().skimmed();
        for p in placements {
            let m = p.is_doable(&s.clone().skimmed(), s.spawn(), self.handling());

            if m {
                s.place(*p);
                s.skim();
            } else {
                return false;
            }
        }

        true
    }

    #[must_use]
    pub fn all_placements_of_piece(&self, board: &Board, piece: Piece) -> Vec<Placement> {
        let width = board.width();
        let height = board.height();

        let r = Rotation::iter().collect_vec();
        let z = &r;

        (0..width)
            .into_par_iter()
            .flat_map(|x| {
                (0..height).into_par_iter().flat_map(move |y| {
                    z.par_iter().filter_map(move |rotation| {
                        let p = Placement::new(piece, x, y, *rotation);

                        if board.is_valid_placement_with_skim(p, false) {
                            Some(p)
                        } else {
                            None
                        }
                    })
                })
            })
            .collect()
    }
}
