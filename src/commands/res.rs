use std::fmt::Write;

use crate::{board::Board, grid::Grid, piece::Piece, program::Sfce};

impl Sfce {
    pub fn res_command(&mut self, n: usize) -> anyhow::Result<()> {
        let x = Grid::from_pages(
            Self::gen_residuals(self.program.args.board_width.unwrap_or(4), n)
                .iter()
                .map(|x| Board {
                    data: x.clone(),
                    ..Default::default()
                }),
        );

        write!(self.buf, "{}", self.tetfu(&x))?;

        Ok(())
    }

    #[allow(clippy::items_after_statements)]
    #[must_use]
    pub fn gen_residuals(columns: usize, n: usize) -> Vec<Vec<Vec<Piece>>> {
        let mut result = Vec::new();

        // Helper function to check if a row is valid (not all empty or all filled)
        fn is_valid_row(row: &[Piece]) -> bool {
            let g_count = row.iter().filter(|&&p| p == Piece::G).count();
            g_count > 0 && g_count < row.len()
        }

        // Helper function to recursively generate boards
        fn generate(
            current_board: &mut Vec<Vec<Piece>>,
            columns: usize,
            remaining_g: usize,
            result: &mut Vec<Vec<Vec<Piece>>>,
            current_row: Vec<Piece>,
            current_pos: usize,
        ) {
            if current_pos == columns {
                if is_valid_row(&current_row) {
                    let mut new_board = current_board.clone();
                    new_board.push(current_row);

                    if remaining_g == 0 {
                        result.push(new_board);
                    } else {
                        // Continue with next row
                        generate(&mut new_board, columns, remaining_g, result, Vec::new(), 0);
                    }
                }
                return;
            }

            // Try placing an empty piece
            let mut row_with_e = current_row.clone();
            row_with_e.push(Piece::E);
            generate(
                current_board,
                columns,
                remaining_g,
                result,
                row_with_e,
                current_pos + 1,
            );

            // Try placing a G piece if we have any remaining
            if remaining_g > 0 {
                let mut row_with_g = current_row.clone();
                row_with_g.push(Piece::G);
                generate(
                    current_board,
                    columns,
                    remaining_g - 1,
                    result,
                    row_with_g,
                    current_pos + 1,
                );
            }
        }

        // Start the generation process
        generate(&mut Vec::new(), columns, n, &mut result, Vec::new(), 0);

        // Filter out boards with invalid rows
        result.retain(|board| board.iter().all(|row| is_valid_row(row)));

        result
    }
}
