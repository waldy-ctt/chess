use ggez::{
    graphics::{self, Mesh, MeshBuilder, Text},
    Context,
};

use crate::game::assets::shared::shared;

const SQUARE_SIZE: f32 = 50.0; // Size of each square in pixel
const BOARD_SIZE: i8 = 8; // 8x8
const ALPHABET: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

pub fn chess_boards(ctx: &mut Context) -> Mesh {
    let mut builder = MeshBuilder::new();

    // Loop over the 8x8 grid
    for row in 1..=BOARD_SIZE {
        for col in 1..=BOARD_SIZE {
            let x = (col - 1) as f32 * SQUARE_SIZE;
            let y = (row - 1) as f32 * SQUARE_SIZE;
            let color = if (row + col) % 2 == 0 {
                shared::constants::SYSTEM_WHITE
            } else {
                shared::constants::SYSTEM_BLACK
            };

            builder
                .rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(x, y, SQUARE_SIZE, SQUARE_SIZE),
                    color,
                )
                .expect("Failed to add rectangle to mesh");
        }
    }

    let mesh_data = builder.build();
    Mesh::from_data(ctx, mesh_data)
}

pub fn create_labels(_ctx: &mut Context) -> (Vec<Text>, Vec<Text>) {
    let mut row_labels = Vec::new();
    let mut col_labels = Vec::new();

    // Column labels (A-H)
    for col in 0..BOARD_SIZE {
        let mut text = Text::new(ALPHABET[col as usize].to_string());
        text.set_font("joystix_mono");
        text.set_scale(SQUARE_SIZE);
        text.fragments_mut()[0].color = Some(shared::constants::SYSTEM_WHITE);
        col_labels.push(text);
    }

    // Row labels (1-8)
    for row in 0..BOARD_SIZE {
        let mut text = Text::new((BOARD_SIZE - row).to_string()); // 8 down to 1
        text.set_font("joystix_mono");
        text.set_scale(SQUARE_SIZE);
        text.fragments_mut()[0].color = Some(shared::constants::SYSTEM_WHITE);
        row_labels.push(text);
    }

    (row_labels, col_labels)
}

pub fn get_each_square_size() -> f32 {
    SQUARE_SIZE
}

pub fn get_board_size() -> i8 {
    BOARD_SIZE
}
