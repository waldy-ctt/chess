use std::{default, path::Path};

use crate::game::assets::draws::chess_boards::{
    self, chess_boards, create_labels, get_board_size, get_each_square_size,
};
use ggez::{
    graphics::{self, Canvas, Color, DrawParam, Drawable, GraphicsContext, Image, Mesh},
    Context,
};

pub fn main_scene(ctx: &mut Context) -> Canvas {
    let canvas: Canvas = new_game(ctx);

    canvas
}

fn new_game(ctx: &mut Context) -> Canvas {
    let chess_board: Mesh = chess_boards(ctx);
    let (row_labels, col_labels) = create_labels(ctx);

    let board_width: f32 = get_each_square_size() * get_board_size() as f32;

    let mut canvas: Canvas =
        graphics::Canvas::from_frame(ctx, graphics::Color::new(0.1, 0.2, 0.3, 1.0));

    let screen_size: (f32, f32) = ctx.gfx.drawable_size();

    let chess_board_start_x: f32 = (screen_size.0 - board_width) / 2.0;
    let chess_board_start_y: f32 = (screen_size.1 - board_width) / 2.0;

    canvas.draw(
        &chess_board,
        DrawParam::new().dest([chess_board_start_x, chess_board_start_y]),
    );

    for (index, label) in col_labels.iter().enumerate() {
        canvas.draw(
            label,
            DrawParam::new().dest([
                (chess_board_start_x
                    + (get_each_square_size() * index as f32)
                    + ((get_each_square_size() / 2.0) - (label.dimensions(ctx).unwrap().w / 2.0))),
                (chess_board_start_y - get_each_square_size()
                    + ((get_each_square_size() / 2.0) - label.dimensions(ctx).unwrap().h / 2.0)),
            ]),
        );

        canvas.draw(
            label,
            DrawParam::new().dest([
                (chess_board_start_x
                    + (get_each_square_size() * index as f32)
                    + ((get_each_square_size() / 2.0) - (label.dimensions(ctx).unwrap().w / 2.0))),
                (chess_board_start_y
                    + board_width
                    + ((get_each_square_size() / 2.0) - label.dimensions(ctx).unwrap().h / 2.0)),
            ]),
        );
    }

    for (index, label) in row_labels.iter().enumerate() {
        // canvas.draw(
        //     label,
        //     DrawParam::new().dest([
        //         chess_board_start_x - get_each_square_size() + ((get_each_square_size() / 2.0) - (label.dimensions(ctx).unwrap().w / 2.0)),
        //         chess_board_start_y + (get_each_square_size() * index as f32) + ((get_each_square_size() / 2.0) - (label.dimensions(ctx).unwrap().h / 2.0)),
        //     ]),
        // )

        let pos = [
            chess_board_start_x - get_each_square_size(),
            chess_board_start_y + (get_each_square_size() * index as f32),
        ];

        let dims = label.dimensions(ctx).unwrap();

        let bg_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(pos[0], pos[1], 50.0, 50.0),
            Color::new(1.0, 0.0, 0.0, 0.5), // Semi-transparent red
        ).expect("Failed to create background rectangle");
        canvas.draw(&bg_rect, graphics::DrawParam::default());

        canvas.draw(
            label,
            DrawParam::new().dest([
                chess_board_start_x - get_each_square_size(),
                chess_board_start_y + (get_each_square_size() * index as f32),
            ]),
        );
    }

    let rook: Image =
        Image::from_path(ctx, "/images/rook-blue.png").expect("Failed to load chess unit");

    canvas.draw(&rook, DrawParam::default());

    canvas
}
