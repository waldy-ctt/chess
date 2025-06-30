use ggez::{
    graphics::{Color, Mesh, MeshBuilder},
    Context};

const SQUARE_SIZE: f32 = 50.0; // Size of each square in pixel
const BOARD_SIZE: i8 = 8; // 8x8

pub fn chess_boards(ctx: &mut Context) -> Mesh {
    let mut builder: MeshBuilder = MeshBuilder::new();

    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let x: f32 = col as f32 * SQUARE_SIZE;
            let y: f32 = row as f32 * SQUARE_SIZE;
            let color = if (row + col) % 2 == 0 {
                Color::WHITE
            } else {
                Color::BLACK
            };

            let _ = builder.rectangle(
                ggez::graphics::DrawMode::fill(), 
                ggez::graphics::Rect::new(x, y, SQUARE_SIZE, SQUARE_SIZE),
                color
            );
        }
    }

    let mesh_data = builder.build();
    Mesh::from_data(ctx, mesh_data)
}

pub fn get_each_square_size() -> f32 {
    SQUARE_SIZE
}

pub fn get_board_size() -> i8 {
   BOARD_SIZE 
}
