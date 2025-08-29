use crate::game::{
    assets::{
        canvas::main_scene::{self, move_chess_unit, new_game},
        draws::chess_boards::{get_board_size, get_each_square_size},
        shared::shared::constants::{ChessColor, ChessUnit},
    },
    central::game_modules,
};
use ggez::{
    event::{EventHandler, MouseButton},
    graphics::{self, Canvas},
    Context, GameError, GameResult,
};

impl EventHandler<GameError> for game_modules::MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let main_scene: Canvas = new_game(_ctx);
        main_scene.finish(_ctx)?;
        Ok(())
    }

    fn resize_event(
        &mut self,
        _ctx: &mut Context,
        _width: f32,
        _height: f32,
    ) -> Result<(), GameError> {
        let board_width: f32 = get_each_square_size() * get_board_size() as f32;
        self.chess_board_start_x = (_width - board_width) / 2.0;
        self.chess_board_start_y = (_height - board_width) / 2.0;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) -> Result<(), GameError> {
        println!("Mouse Clicked at:\nX: {_x}\nY: {_y}\nButton: {_button:?}");

        let mut canvas = graphics::Canvas::from_frame(_ctx, None);

        if _button == MouseButton::Left {
            if let Some((row, col)) = self.get_square_from_mouse(_x, _y) {
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
                    // TODO: Validate move according to chess rules

                    move_chess_unit(
                        _ctx,
                        &mut canvas,
                        ChessUnit::Rook,
                        ChessColor::Black,
                        [from_row, from_col],
                        [row, col],
                    );
                    canvas.finish(_ctx)?;

                    println!(
                        "Moving to empty square ({}, {}) from ({}, {})",
                        row, col, from_row, from_col
                    );
                    self.board[row][col] = self.board[from_row][from_col];
                    self.board[from_row][from_col] = None;
                    self.selected_square = None;
                } else {
                    println!("Clicked empty square ({}, {})", row, col);
                    self.selected_square = None;
                }
            } else {
                println!("Clicked outside board at ({}, {})", _x, _y);
                self.selected_square = None;
            }
        }
        Ok(())
    }
}
