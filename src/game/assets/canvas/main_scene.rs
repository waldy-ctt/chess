use crate::game::{
    assets::{
        draws::{
            chess_boards::{
                self, chess_boards, create_labels, get_board_size, get_each_square_size,
            },
            chess_unit::chess_unit,
        },
        shared::shared::{self},
    },
    central::game_modules::MyGame,
};
use ggez::{
    graphics::{self, Canvas, DrawParam, Drawable, Image, Mesh, MeshBuilder, Rect},
    Context,
};

use shared::constants as sharedConst;

struct SquarePosition {
    x_start: f32,
    x_end: f32,
    y_start: f32,
    y_end: f32,
}

struct BoardPosition {
    x_start: f32,
    x_end: f32,
    y_start: f32,
    y_end: f32,
}

fn get_board_width() -> f32 {
    get_each_square_size() * get_board_size() as f32
}

fn get_chess_board_position(ctx: &mut Context) -> BoardPosition {
    let screen_size: (f32, f32) = ctx.gfx.drawable_size();
    let chess_board_start_x: f32 = (screen_size.0 - get_board_width()) / 2.0;
    let chess_board_start_y: f32 = (screen_size.1 - get_board_width()) / 2.0;
    BoardPosition {
        x_start: chess_board_start_x,
        x_end: chess_board_start_x + get_board_width(),
        y_start: chess_board_start_y,
        y_end: chess_board_start_y + get_board_width(),
    }
}

pub fn new_game(ctx: &mut Context, game: &MyGame) -> Canvas {
    let chess_board: Mesh = chess_boards(ctx);
    let (row_labels, col_labels) = create_labels(ctx);

    let board_position: BoardPosition = get_chess_board_position(ctx);

    let mut canvas: Canvas = graphics::Canvas::from_frame(ctx, sharedConst::BACKGROUND_COLOR);

    canvas.draw(
        &chess_board,
        DrawParam::new().dest([board_position.x_start, board_position.y_start]),
    );

    for (index, label) in col_labels.iter().enumerate() {
        canvas.draw(
            label,
            DrawParam::new().dest([
                (board_position.x_start
                    + (get_each_square_size() * index as f32)
                    + ((get_each_square_size() / 2.0) - (label.dimensions(ctx).unwrap().w / 2.0))),
                (board_position.y_start - get_each_square_size()
                    + ((get_each_square_size() / 2.0) - label.dimensions(ctx).unwrap().h / 2.0)),
            ]),
        );

        canvas.draw(
            label,
            DrawParam::new().dest([
                (board_position.x_start
                    + (get_each_square_size() * index as f32)
                    + ((get_each_square_size() / 2.0) - (label.dimensions(ctx).unwrap().w / 2.0))),
                (board_position.y_end
                    + ((get_each_square_size() / 2.0) - label.dimensions(ctx).unwrap().h / 2.0)),
            ]),
        );
    }

    for (index, label) in row_labels.iter().enumerate() {
        canvas.draw(
            label,
            DrawParam::new().dest([
                board_position.x_start - get_each_square_size() + get_each_square_size() / 2.0
                    - label.dimensions(ctx).unwrap().w / 2.0,
                board_position.y_start + (get_each_square_size() * index as f32),
            ]),
        );

        canvas.draw(
            label,
            DrawParam::new().dest([
                board_position.x_end + get_each_square_size() / 2.0
                    - label.dimensions(ctx).unwrap().w / 2.0,
                board_position.y_start + (get_each_square_size() * index as f32),
            ]),
        );
    }

    for row in 0..chess_boards::get_board_size() {
        for col in 0..chess_boards::get_board_size() {
            if let Some((piece, color)) = game.board[row as usize][col as usize] {
                let image: Image = chess_unit(piece, color, ctx);
                let offset: [f32; 2] = get_image_offset(&image, ctx);
                canvas.draw(
                    &image,
                    DrawParam::new().dest([
                        board_position.x_start + (col as f32 * get_each_square_size()) + offset[0],
                        board_position.y_start + (row as f32 * get_each_square_size()) + offset[1],
                    ]),
                )
            }
        }
    }

    // Highlight selected square
    // if let Some((row, col)) = game.selected_square {
    //     let highlight = Mesh::new_rectangle(
    //         ctx,
    //         graphics::DrawMode::fill(),
    //         Rect::new(
    //             board_position.x_start + (col as f32 * get_each_square_size()),
    //             board_position.y_start + (row as f32 * get_each_square_size()),
    //             get_each_square_size(),
    //             get_each_square_size(),
    //         ),
    //         sharedConst::SYSTEM_WHITE,
    //     )
    //     .unwrap();
    //     canvas.draw(&highlight, DrawParam::new());
    // }

    canvas
}

fn get_image_offset(image: &Image, ctx: &mut Context) -> [f32; 2] {
    // Returns [offsetX, offsetY] to center the image in a square
    let dimensions = image.dimensions(ctx).unwrap();
    [
        (get_each_square_size() - dimensions.w) / 2.0, // Center horizontally
        (get_each_square_size() - dimensions.h) / 2.0, // Center vertically
    ]
}
