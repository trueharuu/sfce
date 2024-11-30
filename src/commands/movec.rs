use std::{
    fmt::Write,
    sync::{Arc, Mutex},
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use strum::IntoEnumIterator;

use crate::{
    board_parser::Tetfu, grid::Grid, page::Page, pattern::Pattern, piece::{Piece, Placement, Rotation}, program::Sfce, ranged::Ranged, traits::CollectVec
};

pub fn move_command(
    f: &mut Sfce,
    tetfu: Tetfu,
    pattern: Pattern,
    clears: Ranged<usize>,
) -> anyhow::Result<()> {
    let board = f
        .resize(tetfu.grid())
        .pages()
        .last()
        .cloned()
        .unwrap_or_default();
    // dbg!(&board);
    let pages = Arc::new(Mutex::new(Grid::default()));
    pattern.queues().par_iter().for_each(|queue| {
        let pl = placements(board.clone(), queue.clone());
        let possible = pl.iter().filter(|x| clears.contains(x.line_clears())).map(|x| {
            x.clone().with_comment(
                queue
                    .iter()
                    .map(|x| x.to_string())
                    .vec()
                    .join("")
                    .to_string(),
            )
        });
        pages.lock().unwrap().extend(possible);
    });

    pages.lock().unwrap().dedup();

    write!(f.buf, "{}", f.tetfu(pages.lock().unwrap().clone()))?;
    Ok(())
}

fn placements(board: Page, queue: Vec<Piece>) -> Vec<Page> {
    if queue.is_empty() {
        return vec![board];
    }

    let mut results = Vec::new();
    let piece = queue.first().unwrap();
    let remaining_queue = queue[1..].to_vec();

    for placement in possible_placements(&board, *piece).pages() {
        results.extend(placements(placement.clone(), remaining_queue.clone()));
    }

    results
}

fn possible_placements(board: &Page, piece: Piece) -> Grid {
    let mut pages = Grid::default();
    for rotation in Rotation::iter() {
        for x in 0..board.width() {
            for y in 0..board.height() {
                let p = Placement {
                    x,
                    y,
                    rotation,
                    piece,
                };

                let page = board.clone();
                let removed_lines = page
                    .0
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| !x.contains(&Piece::E));
                let s = board.clone().skimmed();

                if s.is_valid_placement(p) {
                    let mut new_board = s.with_placement(p);
                    for (i, l) in removed_lines {
                        new_board.0.insert(i, l.clone());
                    }

                    pages.add_page(new_board);
                }
            }
        }
    }

    pages
}
