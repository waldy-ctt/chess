use ggez::{
    graphics::{self, Color, Mesh, MeshBuilder, Text},
    Context,
};

use crate::game::assets::shared::shared::constants as sharedConst;

pub fn chess_boards(ctx: &mut Context) -> Mesh {
    let mut builder = MeshBuilder::new();

    // Loop over the 8x8 grid
    for row in 1..=sharedConst::BOARD_SIZE {
        for col in 1..=sharedConst::BOARD_SIZE {
            let x = (col - 1) as f32 * sharedConst::SQUARE_SIZE;
            let y = (row - 1) as f32 * sharedConst::SQUARE_SIZE;
            let color = if (row + col) % 2 == 0 {
                sharedConst::SYSTEM_WHITE
            } else {
                sharedConst::SYSTEM_BLACK
            };

            builder
                .rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(x, y, sharedConst::SQUARE_SIZE, sharedConst::SQUARE_SIZE),
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
    for col in 0..sharedConst::BOARD_SIZE {
        let mut text = Text::new(sharedConst::ALPHABET[col as usize].to_string());
        text.set_font("joystix_mono");
        text.set_scale(sharedConst::SQUARE_SIZE);
        text.fragments_mut()[0].color = Some(sharedConst::SYSTEM_WHITE);
        col_labels.push(text);
    }

    // Row labels (1-8)
    for row in 0..sharedConst::BOARD_SIZE {
        let mut text = Text::new((sharedConst::BOARD_SIZE - row).to_string()); // 8 down to 1
        text.set_font("joystix_mono");
        text.set_scale(sharedConst::SQUARE_SIZE);
        text.fragments_mut()[0].color = Some(sharedConst::SYSTEM_WHITE);
        row_labels.push(text);
    }

    (row_labels, col_labels)
}

pub fn get_each_square_size() -> f32 {
    sharedConst::SQUARE_SIZE
}

pub fn get_board_size() -> i8 {
    sharedConst::BOARD_SIZE
}

pub fn get_square_type(position: [usize; 2]) -> Color {
    let color: Color;

    if (position[0] + position[1]) % 2 == 0 {
        color = sharedConst::SYSTEM_WHITE;
    } else {
        color = sharedConst::SYSTEM_BLACK;
    }

    color
}
