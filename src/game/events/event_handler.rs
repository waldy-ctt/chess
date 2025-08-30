use crate::game::{
    assets::{
        canvas::main_scene::{move_chess_unit, new_game},
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
        let main_scene: Canvas = new_game(_ctx, self);
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
        if _button == MouseButton::Left {
            self.handle_mouse_click(_x, _y);
        }

        Ok(())
    }
}
