use fumen::{CellColor, Fumen};

use crate::{board::Board, grid::Grid, piece::Piece};

pub fn grid_to_fumen(grid: Grid) -> Fumen {
    let mut f = Fumen::default();
    for page in grid.pages() {
        let p = f.add_page();
        let mut field = [[CellColor::Empty; 10]; 23];

        for (i, row) in page.rows().iter().enumerate() {
            for (c, t) in row.iter().enumerate().take(10.min(row.len())) {
                field[i][c] = t.cell_color();
            }
        }
        p.field = field;
        p.comment = page.comment.clone();
    }

    f
}

pub fn fumen_to_grid(fumen: Fumen) -> Grid {
    Grid(
        fumen
            .pages
            .iter()
            .map(|x| {
                // dbg!(&x);
                Board {
                    data: x
                        .field
                        .iter()
                        .map(|x| {
                            x.iter()
                                .map(|x| match x {
                                    CellColor::Empty => Piece::E,
                                    CellColor::Grey => Piece::G,
                                    CellColor::I => Piece::I,
                                    CellColor::J => Piece::J,
                                    CellColor::O => Piece::O,
                                    CellColor::L => Piece::L,
                                    CellColor::Z => Piece::Z,
                                    CellColor::S => Piece::S,
                                    CellColor::T => Piece::T,
                                })
                                .collect::<Vec<_>>()
                        })
                        .collect::<Vec<_>>(),
                    comment: x.comment.clone(),
                    margin: 0,
                }
            })
            .collect::<Vec<_>>(),
    )
}
