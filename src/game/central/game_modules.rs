use ggez::{graphics, Context};

use crate::game::assets::shared::shared::constants as sharedConst;

pub struct MyGame {
    pub board: [[Option<(sharedConst::ChessUnit, sharedConst::ChessColor)>; 8]; 8],
    pub selected_square: Option<(usize, usize)>,
    pub chess_board_start_x: f32,
    pub chess_board_start_y: f32,
    pub square_size: f32,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        _ctx.gfx.add_font(
            "joystix_mono",
            graphics::FontData::from_path(_ctx, "/fonts/joystix monospace.ttf")
                .expect("Failed to load font"),
        );

        let mut board = [[None; 8]; 8];
        board[0] = [
            Some((sharedConst::ChessUnit::Rook, sharedConst::ChessColor::White)),
            Some((
                sharedConst::ChessUnit::Knight,
                sharedConst::ChessColor::White,
            )),
            Some((
                sharedConst::ChessUnit::Bishop,
                sharedConst::ChessColor::White,
            )),
            Some((
                sharedConst::ChessUnit::Queen,
                sharedConst::ChessColor::White,
            )),
            Some((sharedConst::ChessUnit::King, sharedConst::ChessColor::White)),
            Some((
                sharedConst::ChessUnit::Bishop,
                sharedConst::ChessColor::White,
            )),
            Some((
                sharedConst::ChessUnit::Knight,
                sharedConst::ChessColor::White,
            )),
            Some((sharedConst::ChessUnit::Rook, sharedConst::ChessColor::White)),
        ];
        for col in 0..8 {
            board[1][col] = Some((sharedConst::ChessUnit::Pawn, sharedConst::ChessColor::White));
            board[6][col] = Some((sharedConst::ChessUnit::Pawn, sharedConst::ChessColor::Black));
        }
        board[7] = [
            Some((sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black)),
            Some((
                sharedConst::ChessUnit::Knight,
                sharedConst::ChessColor::Black,
            )),
            Some((
                sharedConst::ChessUnit::Bishop,
                sharedConst::ChessColor::Black,
            )),
            Some((
                sharedConst::ChessUnit::Queen,
                sharedConst::ChessColor::Black,
            )),
            Some((sharedConst::ChessUnit::King, sharedConst::ChessColor::Black)),
            Some((
                sharedConst::ChessUnit::Bishop,
                sharedConst::ChessColor::Black,
            )),
            Some((
                sharedConst::ChessUnit::Knight,
                sharedConst::ChessColor::Black,
            )),
            Some((sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black)),
        ];

        let board_width = sharedConst::SQUARE_SIZE * sharedConst::BOARD_SIZE as f32;
        let screen_size = _ctx.gfx.drawable_size();
        let chess_board_start_x = (screen_size.0 - board_width) / 2.0;
        let chess_board_start_y = (screen_size.1 - board_width) / 2.0;
        let square_size = sharedConst::SQUARE_SIZE;

        MyGame {
            board,
            selected_square: None,
            chess_board_start_x,
            chess_board_start_y,
            square_size,
        }
    }

    pub fn get_square_from_mouse(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        let col = ((x - self.chess_board_start_x) / self.square_size).floor() as usize;
        let row = ((y - self.chess_board_start_y) / self.square_size).floor() as usize;

        if row < sharedConst::BOARD_SIZE as usize
            && col < sharedConst::BOARD_SIZE as usize
            && x >= self.chess_board_start_x
            && y >= self.chess_board_start_y
        {
            Some((row, col))
        } else {
            None
        }
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;
        if self.board[from_row][from_col].is_some() && self.board[to_row][to_col].is_none() {
            println!(
                "Moving {}-{:?} from ({}, {}) to ({}, {})",
                self.board[from_row][from_col].unwrap().0.to_string(),
                self.board[from_row][from_col].unwrap().1,
                from_row,
                from_col,
                to_row,
                to_col
            );
            self.board[to_row][to_col] = self.board[from_row][from_col];
            self.board[from_row][from_col] = None;
            true
        } else {
            println!(
                "Invalid move from ({}, {}) to ({}, {})",
                from_row, from_col, to_row, to_col
            );
            false
        }
    }

    pub fn handle_mouse_click(&mut self, x: f32, y: f32) {
        if let Some((row, col)) = self.get_square_from_mouse(x, y) {
            if let Some((piece, color)) = self.board[row][col] {
                println!(
                    "Selected {}-{:?} at square ({}, {})",
                    piece.to_string(),
                    color,
                    row,
                    col
                );
                self.selected_square = Some((row, col));
            } else if let Some((from_row, from_col)) = self.selected_square {
                self.move_piece((from_row, from_col), (row, col));
                self.selected_square = None;
            } else {
                println!("Clicked empty square ({}, {})", row, col);
                self.selected_square = None;
            }
        } else {
            println!("Clicked outside board at ({}, {})", x, y);
            self.selected_square = None;
        }
    }
}
