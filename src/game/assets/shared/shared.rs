use ggez::graphics::Color;

pub mod constants {
    use super::*;

    pub const BACKGROUND_COLOR: Color =
        Color::new(185.0 / 255.0, 187.0 / 255.0, 223.0 / 255.0, 1.0);
    pub const SYSTEM_WHITE: Color = Color::new(233.0 / 255.0, 236.0 / 255.0, 239.0 / 255.0, 1.0);
    pub const SYSTEM_BLACK: Color = Color::new(51.0 / 255.0, 51.0 / 255.0, 51.0 / 255.0, 1.0);

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum ChessUnit {
        King,
        Queen,
        Rook,
        Bishop,
        Knight,
        Pawn,
    }

    impl ChessUnit {
        pub fn to_string(&self) -> &'static str {
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

    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum ChessColor {
        Black,
        White,
    }

    impl ChessColor {
        pub fn to_string(&self) -> &'static str {
            match self {
                Self::Black => "blue",
                Self::White => "white",
            }
        }
    }

    pub const SQUARE_SIZE: f32 = 50.0;
    pub const BOARD_SIZE: i8 = 8; // 8x8
    pub const ALPHABET: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
}
