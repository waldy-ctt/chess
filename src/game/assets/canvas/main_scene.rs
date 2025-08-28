use crate::game::assets::{
    draws::{
        chess_boards::{self, chess_boards, create_labels, get_board_size, get_each_square_size},
        chess_unit::chess_unit,
    },
    shared::shared,
};
use ggez::{
    graphics::{self, Canvas, DrawParam, Drawable, Image, Mesh},
    Context,
};

use shared::constants as sharedConst;

pub fn main_scene(ctx: &mut Context) -> Canvas {
    let canvas: Canvas = new_game(ctx);

    canvas
}

fn new_game(ctx: &mut Context) -> Canvas {
    let chess_board: Mesh = chess_boards(ctx);
    let (row_labels, col_labels) = create_labels(ctx);

    let board_width: f32 = get_each_square_size() * get_board_size() as f32;

    let mut canvas: Canvas = graphics::Canvas::from_frame(ctx, sharedConst::BACKGROUND_COLOR);

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
        canvas.draw(
            label,
            DrawParam::new().dest([
                chess_board_start_x - get_each_square_size() + get_each_square_size() / 2.0
                    - label.dimensions(ctx).unwrap().w / 2.0,
                chess_board_start_y + (get_each_square_size() * index as f32),
            ]),
        );

        canvas.draw(
            label,
            DrawParam::new().dest([
                chess_board_start_x + board_width + get_each_square_size() / 2.0
                    - label.dimensions(ctx).unwrap().w / 2.0,
                chess_board_start_y + (get_each_square_size() * index as f32),
            ]),
        );
    }

    for row in 0..chess_boards::get_board_size() {
        for col in 0..chess_boards::get_board_size() {
            // let (piece, color) = match (row, col) {
            //     // White pieces (rank 1: major pieces, rank 2: pawns)
            //     (0, 0) | (0, 7) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::White),
            //     (0, 1) | (0, 6) => (
            //         sharedConst::ChessUnit::Knight,
            //         sharedConst::ChessColor::White,
            //     ),
            //     (0, 2) | (0, 5) => (
            //         sharedConst::ChessUnit::Bishop,
            //         sharedConst::ChessColor::White,
            //     ),
            //     (0, 3) => (
            //         sharedConst::ChessUnit::Queen,
            //         sharedConst::ChessColor::White,
            //     ),
            //     (0, 4) => (sharedConst::ChessUnit::King, sharedConst::ChessColor::White),
            //     (1, _) => (sharedConst::ChessUnit::Pawn, sharedConst::ChessColor::White),
            //     // Black pieces (rank 8: major pieces, rank 7: pawns)
            //     (7, 0) | (7, 7) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
            //     (7, 1) | (7, 6) => (
            //         sharedConst::ChessUnit::Knight,
            //         sharedConst::ChessColor::Black,
            //     ),
            //     (7, 2) | (7, 5) => (
            //         sharedConst::ChessUnit::Bishop,
            //         sharedConst::ChessColor::Black,
            //     ),
            //     (7, 3) => (
            //         sharedConst::ChessUnit::Queen,
            //         sharedConst::ChessColor::Black,
            //     ),
            //     (7, 4) => (sharedConst::ChessUnit::King, sharedConst::ChessColor::Black),
            //     (6, _) => (sharedConst::ChessUnit::Pawn, sharedConst::ChessColor::Black),
            //     // Empty squares
            //     _ => continue, // Skip drawing for empty squares
            // };
            // TODO: Uncomment above when drawed all piece and color

            let (piece, color) = match (row, col) {
                // White pieces (rank 1: major pieces, rank 2: pawns)
                (0, 0) | (0, 7) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                (0, 1) | (0, 6) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                (0, 2) | (0, 5) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                (0, 3) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                (0, 4) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                (1, _) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                // Black pieces (rank 8: major pieces, rank 7: pawns)
                (7, 0) | (7, 7) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                (7, 1) | (7, 6) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                (7, 2) | (7, 5) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                (7, 3) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                (7, 4) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                (6, _) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                // Empty squares
                _ => continue, // Skip drawing for empty squares
            };

            let image: Image = chess_unit(piece, color, ctx);
            let offset: [f32; 2] = get_image_offset(&image, ctx);
            canvas.draw(
                &image,
                DrawParam::new().dest([
                    chess_board_start_x + (col as f32 * get_each_square_size()) + offset[0],
                    chess_board_start_y + (row as f32 * get_each_square_size()) + offset[1],
                ]),
            )
        }
    }

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
