use std::{collections::HashSet, fmt::Write as _};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    board::Board, board_parser::Tetfu, grid::Grid, pattern::Pattern, piece::Piece,
    placement::Placement, program::Sfce,
};

impl Sfce {
    pub fn congruent_command(
        &mut self,
        tetfu: &Tetfu,
        pattern: &Pattern,
        color: Piece,
        _minimal: bool,
    ) -> anyhow::Result<()> {
        let pgs = tetfu.pages();

        let z = pgs.first().unwrap().clone();
        let cspots = z.clone().filter(|_, _, c| c == color).fast().filled_cells();
        let board = z.clone().filter(|_, _, c| c != color);
        let mut g = Grid::default();
        let mut seen_queues = HashSet::new();
        for q in pattern.queues() {
            if seen_queues.contains(&q) {
                continue; 
            }
            for hq in self.hold_queues(&q) {
                if seen_queues.contains(&q) || seen_queues.contains(&hq) {
                    // println!("seen {q} before!");
                    continue;
                }
                let pms = self.congruent_many(&board, cspots.clone(), hq.pieces());

                for pm in pms {
                    if self.is_doable(&board, &pm) {
                        seen_queues.insert(q.clone());
                        seen_queues.insert(hq.clone());
                        g.add_page((z.clone() | board.with_many_placements(&pm)).with_comment(&q));
                    }
                }
            }
        }

        writeln!(self.buf, "{}", self.tetfu(&g))?;

        Ok(())
    }

    pub fn congruent_many(
        &self,
        board: &Board,
        cspots: HashSet<(usize, usize)>,
        queue: &[Piece],
    ) -> Vec<Vec<Placement>> {
        if queue.is_empty() {
            return vec![vec![]];
        }

        // i can haz optimizationburger?
        if queue.len() == 1 {
            return self
                .congruent_single(board, cspots.clone(), queue[0])
                .iter()
                .map(|x| vec![*x])
                .collect();
        }

        let piece = queue[0];
        let remaining_queue = &queue[1..];
        let placements = self.congruent_single(board, cspots.clone(), piece);

        placements
            .into_par_iter()
            .map(|p| {
                let mut s = board.clone();
                s.skim_place(p);

                let nc = cspots
                    .iter()
                    .filter(|x| !p.cells().unwrap().contains(x))
                    .copied()
                    .collect();

                let sub_placements = self.congruent_many(&s, nc, remaining_queue);

                sub_placements.into_par_iter().map(move |mut sr| {
                    sr.insert(0, p);
                    sr
                })
            })
            .flatten()
            .collect()
    }

    pub fn congruent_single(
        &self,
        board: &Board,
        cspots: HashSet<(usize, usize)>,
        piece: Piece,
    ) -> Vec<Placement> {
        // println!("searching for {piece} within {cspots:?}");
        self.all_placements_of_piece(board, piece)
            .into_par_iter()
            .filter(|x| {
                x.cells()
                    .unwrap()
                    .into_par_iter()
                    .all(|(x, y)| cspots.contains(&(x, y)))
            })
            .collect()
    }
}
