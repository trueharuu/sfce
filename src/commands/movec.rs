use std::fmt::Write;

use indicatif::ProgressStyle;
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
    traits::{CollectVec, FullyDedup},
};

pub fn command(
    f: &mut Sfce,
    tetfu: &Tetfu,
    pattern: &Pattern,
    clears: Ranged<usize>,
    minimal: bool,
) -> anyhow::Result<()> {
    let board = f
        .resize(tetfu.grid())
        .pages()
        .last()
        .cloned()
        .unwrap_or_default();
    // .to_gray();
    // dbg!(&board);
    let mut pages = Grid::default();
    let qbar = indicatif::ProgressBar::new(pattern.queues().len() as u64);
    let style = ProgressStyle::default_spinner()
        .template("{spinner} {prefix} {msg:.bold} [{pos}/{len}]")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    qbar.set_prefix("Generating placements for queue");
    qbar.set_style(style);
    for queue in pattern.queues() {
        qbar.set_message(queue.iter().map(std::string::ToString::to_string).vec().join(""));
        qbar.inc(1);

        let apoq = all_placements_of_queue(f, &board, queue.pieces());
        let ap = apoq
            .iter()
            .map(|x| (x, board.with_many_placements(x).with_comment(&queue)))
            .filter(|x| clears.contains(&x.1.line_clears()))
            .fully_dedup_by(|(x, _), (y, _)| {
                // println!("{px:?} = {py:?}");
                minimal && x.iter().map(super::super::placement::Placement::piece).eq(y.iter().map(super::super::placement::Placement::piece))
            })
            .fully_dedup_by_key(|x| x.1.clone())
            .filter(|x| is_doable(f, &board, x.0))
            .map(|x| x.1);
        // println!("{:?}", ap.vec())

        pages.extend(ap);
    }

    pages.dedup_by_board();
    write!(f.buf, "{}", f.tetfu(&pages))?;

    Ok(())
}

fn all_placements_of_queue(sfce: &mut Sfce, board: &Board, queue: &[Piece]) -> Vec<Vec<Placement>> {
    if queue.is_empty() {
        return vec![vec![]];
    }

    let piece = queue[0];
    let remaining_queue = &queue[1..];
    let placements = all_placements_of_piece(sfce, board, piece);
    let mut result = vec![];

    for p in placements {
        let mut s = board.clone();
        s.place(p);
        let sub_placements = all_placements_of_queue(sfce, &s, remaining_queue);

        for mut sr in sub_placements {
            sr.insert(0, p);
            result.push(sr);
        }
    }

    result
}

fn is_doable(sfce: &mut Sfce, board: &Board, placements: &[Placement]) -> bool {
    if sfce.handling().ignore {
        return true;
    }

    let mut s = board.clone().skimmed();
    for p in placements {
        let m = sfce.is_placement_possible(&s, *p);
        println!("[{m:.1}] placing {} {:?} on {s}", p.piece(), p.rotation());
        if m {
            s.place(*p);
            s.skim();
        } else {
            return false;
        }
    }

    true
}

fn all_placements_of_piece(_: &mut Sfce, board: &Board, piece: Piece) -> Vec<Placement> {
    let mut m = vec![];
    for x in 0..board.width() {
        for y in 0..board.height() {
            for rotation in Rotation::iter() {
                let p = Placement::new(piece, x, y, rotation);

                if board.clone().skimmed().is_valid_placement(p, false) {
                    println!("{p:?} on board was valid!");
                    m.push(p);
                }
            }
        }
    }

    m
}
