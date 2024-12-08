use std::fmt::Write;

use strum::IntoEnumIterator;

use crate::{
    board::Board, board_parser::Tetfu, input::Input, piece::Rotation, placement::Placement,
    program::Sfce,
};

pub fn finesse_command(f: &mut Sfce, tetfu: Tetfu) -> anyhow::Result<()> {
    let board = f
        .resize(tetfu.grid())
        .pages()
        .last()
        .cloned()
        .unwrap_or_default();

    let og = board.clone().only_gray();
    // println!("{board}");
    let placement = identify_placement(&board);

    if let Some(p) = placement {
        // eprintln!("{p:?}");
        let keys = f.finesse(p, &og);
        let mut i = Input::new(&og, p.piece(), board.spawn(), Rotation::North, f.handling());
        if let Some(k) = keys {
            let v = i.show_inputs(&k);
            write!(f.buf, "{}", f.tetfu(v))?;
        } else {
            eprintln!("this placement is impossible");
        }
    } else {
        println!("no piece is placed");
    }
    // println!("{:?}", placement);

    Ok(())
}

fn identify_placement(board: &Board) -> Option<Placement> {
    let mut colored_cells = vec![];
    for x in 0..board.width() {
        for y in 0..board.height() {
            // println!("{:?}", board.get(x, y));
            if board.get(x, y).is_filled_with_piece() {
                colored_cells.push((x, y, board.get(x, y)));
            }
        }
    }

    // println!("{colored_cells:?}");
    if colored_cells.len() != 4 {
        return None;
    }

    for (x, y, piece) in colored_cells {
        for rotation in Rotation::iter() {
            let p = Placement::new(piece, x, y, rotation);
            let trial = board.clone().only_gray();
            // let trial_p = board.clone().only_gray().with_placement(p);
            // println!("{p:?}");
            // println!("{board}");
            // println!("{trial}");
            // println!("{trial_p}");
            // println!();

            if trial.with_placement(p) == *board {
                return Some(p);
            }
        }
    }

    None
}