use std::{
    fmt::Write as _,
    sync::{Arc, Mutex},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use strum::IntoEnumIterator;

use crate::{
    board::Board,
    board_parser::Tetfu,
    grid::Grid,
    pattern::Pattern,
    piece::{Piece, Rotation},
    placement::Placement,
    program::Sfce,
    traits::{CollectVec, FullyDedupParallel},
};

impl Sfce {
    // TODO: handle skims
    pub fn congruent_command(
        &mut self,
        tetfu: &Tetfu,
        pattern: &Pattern,
        color: Piece,
        minimal: bool,
    ) -> anyhow::Result<()> {
        let p = self.resize(tetfu.grid()).page();

        let grid = Arc::new(Mutex::new(Grid::default()));

        let og = p.clone().filter(|_, _, p| p != color);
        let target = p.clone().filter(|_, _, p| p == color);

        pattern.queues().into_par_iter().for_each(|q| {

            grid.lock().unwrap().extend(
                q.hold_queues()
                    .into_par_iter()
                    .flat_map(|h| {
                        self.congruent_placements_of_queue(&target, h.pieces())
                            .into_par_iter()
                    })
                    .filter(|z| self.is_doable(&og, z))
                    .map(|z| {
                        (
                            z.clone(),
                            p.clone()
                                .to_gray()
                                .with_many_placements(&z)
                                .with_comment(format!("{q}")),
                        )
                    })
                    .fully_dedup_by(|(x, _), (y, _)| {
                        minimal
                            && x.iter()
                                .map(Placement::piece)
                                .eq(y.iter().map(Placement::piece))
                    })
                    .map(|x| x.1)
                    .collect::<Vec<_>>(),
            );
        });

        grid.lock().unwrap().dedup_by_board();

        writeln!(
            self.buf,
            "{}",
            self.tetfu(&Arc::into_inner(grid).unwrap().into_inner().unwrap())
        )?;

        Ok(())
    }

    #[must_use]
    pub fn congruent_placements_of_queue(
        &self,
        board: &Board,
        queue: &[Piece],
    ) -> Vec<Vec<Placement>> {
        if queue.is_empty() {
            return vec![vec![]];
        }

        if queue.len() == 1 {
            return self
                .congruent_placements_of_piece(board, queue[0])
                .iter()
                .map(|x| vec![*x])
                .collect();
        }

        let piece = queue[0];
        let remaining_queue = &queue[1..];
        let placements = self.congruent_placements_of_piece(board, piece);

        placements
            .into_par_iter()
            .map(|p| {
                let used_cells = p.piece().cells(p.x(), p.y(), p.rotation()).unwrap();
                let s = board
                    .clone()
                    .filter(|x, y, _| !used_cells.contains(&(x, y)));

                let sub_placements = self.congruent_placements_of_queue(&s, remaining_queue);

                sub_placements.into_par_iter().map(move |mut sr| {
                    sr.insert(0, p);
                    sr
                })
            })
            .flatten()
            .collect()
    }

    #[must_use]
    pub fn congruent_placements_of_piece(&self, board: &Board, piece: Piece) -> Vec<Placement> {
        Rotation::iter()
            .flat_map(|r| {
                board
                    .fast()
                    .filled_cells()
                    .iter()
                    .map(|(x, y)| Placement::new(piece, *x, *y, r))
                    .filter(|x| {
                        piece.cells(x.x(), x.y(), x.rotation()).is_some_and(|x| {
                            x.iter().all(|x| board.fast().filled_cells().contains(x))
                        })
                    })
                    .vec()
            })
            .vec()
    }
}
