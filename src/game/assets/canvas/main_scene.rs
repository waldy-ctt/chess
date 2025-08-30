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
                // ============ REMOVE WHEN DRAWED
                let (piece, color) = match (row, col) {
                    (0, 0) | (0, 7) => {
                        (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black)
                    }
                    (0, 1) | (0, 6) => {
                        (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black)
                    }
                    (0, 2) | (0, 5) => {
                        (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black)
                    }
                    (0, 3) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                    (0, 4) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                    (1, _) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                    (7, 0) | (7, 7) => {
                        (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black)
                    }
                    (7, 1) | (7, 6) => {
                        (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black)
                    }
                    (7, 2) | (7, 5) => {
                        (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black)
                    }
                    (7, 3) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                    (7, 4) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                    (6, _) => (sharedConst::ChessUnit::Rook, sharedConst::ChessColor::Black),
                    _ => continue,
                };
                // ============ REMOVE WHEN DRAWED
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
    if let Some((row, col)) = game.selected_square {
        let highlight = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(
                board_position.x_start + (col as f32 * get_each_square_size()),
                board_position.y_start + (row as f32 * get_each_square_size()),
                get_each_square_size(),
                get_each_square_size(),
            ),
            sharedConst::SYSTEM_WHITE,
        )
        .unwrap();
        canvas.draw(&highlight, DrawParam::new());
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

pub fn move_chess_unit(
    ctx: &mut Context,
    canvas: &mut Canvas,
    chess_unit_type: sharedConst::ChessUnit,
    chess_color: sharedConst::ChessColor,
    first_postition: [usize; 2],
    second_position: [usize; 2],
) {
    let first_block: Mesh = Mesh::from_data(
        ctx,
        MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                Rect::new(
                    0.0,
                    0.0,
                    chess_boards::get_each_square_size(),
                    chess_boards::get_each_square_size(),
                ),
                chess_boards::get_square_type(first_postition),
            )
            .expect("Failed to generate new first block")
            .build(),
    );

    let first_square_position: SquarePosition = calculate_square_position(ctx, first_postition);

    canvas.draw(
        &first_block,
        DrawParam::new().dest([first_square_position.x_start, first_square_position.y_start]),
    );

    let second_block: Mesh = Mesh::from_data(
        ctx,
        MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                Rect::new(
                    0.0,
                    0.0,
                    chess_boards::get_each_square_size(),
                    chess_boards::get_each_square_size(),
                ),
                chess_boards::get_square_type(second_position),
            )
            .expect("Failed to generate block for second block")
            .build(),
    );

    let second_square_position: SquarePosition = calculate_square_position(ctx, second_position);

    canvas.draw(
        &second_block,
        DrawParam::new().dest([
            second_square_position.x_start,
            second_square_position.y_start,
        ]),
    );

    let new_chess_image: Image = chess_unit(chess_unit_type, chess_color, ctx);
    let new_chess_offset: [f32; 2] = get_image_offset(&new_chess_image, ctx);

    canvas.draw(
        &new_chess_image,
        DrawParam::new().dest([
            second_square_position.x_start + new_chess_offset[0],
            second_square_position.y_start + new_chess_offset[1],
        ]),
    );
}

fn calculate_square_position(ctx: &mut Context, position: [usize; 2]) -> SquarePosition {
    let board_position: BoardPosition = get_chess_board_position(ctx);

    SquarePosition {
        x_start: board_position.x_start
            + (position[0] as f32) * chess_boards::get_each_square_size(),
        x_end: (board_position.x_start
            + (position[0] as f32) * chess_boards::get_each_square_size())
            + chess_boards::get_each_square_size(),
        y_start: board_position.y_start
            + (position[1] as f32) * chess_boards::get_each_square_size(),
        y_end: (board_position.y_start
            + (position[1] as f32) * chess_boards::get_each_square_size())
            + chess_boards::get_each_square_size(),
    }
}
