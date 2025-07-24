use std::fmt::format;

use ggez::{
    graphics::{Image, Mesh},
    Context,
};

enum ChessUnit {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

enum ChessColor {
    Black,
    White,
}

impl ChessUnit {
    fn to_string(&self) -> &'static str {
        match self {
            Self::King => "king",
            Self::Queen => "queen",
            Self::Rook => "rook",
            Self::Bishop => "bishop",
            Self::Knight => "knight",
            Self::Pawn => "pawn",
        }
    }
}

impl ChessColor {
    fn to_string(&self) -> &'static str {
        match self {
            Self::Black => "blue",
            Self::White => "white",
        }
    }
}

pub fn chess_unit(chess_unit: ChessUnit, chess_color: ChessColor, ctx: &mut Context) -> Mesh {
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
}
