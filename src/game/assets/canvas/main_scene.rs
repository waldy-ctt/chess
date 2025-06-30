use ggez::{graphics::{self, Canvas, DrawParam, GraphicsContext, Mesh}, Context};
use crate::game::assets::draws::chess_boards::{chess_boards, get_board_size, get_each_square_size};

pub fn main_scene(ctx: &mut Context) -> Canvas {
    let chess_board: Mesh = chess_boards(ctx);

    let board_width: f32 = get_each_square_size() * get_board_size() as f32;

    let mut canvas: Canvas = graphics::Canvas::from_frame(ctx, graphics::Color::new(0.1, 0.2, 0.3, 1.0));

    let screen_size: (f32, f32) = ctx.gfx.drawable_size();

    canvas.draw(&chess_board, DrawParam::new().dest([((screen_size.0 - board_width) / 2.0 ), ((screen_size.1 - board_width) / 2.0)]));

    canvas
}
