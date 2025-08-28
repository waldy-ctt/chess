use crate::game::assets::shared::shared;
use ggez::{graphics::Image, Context};

pub fn chess_unit(
    chess_unit: shared::constants::ChessUnit,
    chess_color: shared::constants::ChessColor,
    ctx: &mut Context,
) -> Image {
    let path: String = format!(
        "/images/{}-{}.png",
        chess_unit.to_string(),
        chess_color.to_string()
    );
    let image: Image = Image::from_path(ctx, path).expect(
        format!(
            "Failed to load {}-{}",
            chess_unit.to_string(),
            chess_color.to_string()
        )
        .as_str(),
    );

    image
}
