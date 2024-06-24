pub type Bitboard = u64;
pub type Piece = usize;
pub type Color = usize;
pub type Square = usize;

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from_square: Square,
    pub to_square: Square,
    pub promotion: Piece
}

impl Default for Move {
    fn default() -> Self {
        Self {
            from_square: square::NONE,
            to_square: square::NONE,
            promotion: piece::NONE
        }
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {}", crate::bitboard::SQUARE_NAMES[self.from_square as usize], crate::bitboard::SQUARE_NAMES[self.to_square as usize])?;
        Ok(())
    }
}

pub mod piece {
    use super::Piece;

    pub const PAWN: Piece = 0;
    pub const KNIGHT: Piece = 1;
    pub const BISHOP: Piece = 2;
    pub const ROOK: Piece = 3;
    pub const QUEEN: Piece = 4;
    pub const KING: Piece = 5;
    pub const NONE: Piece = 6;
}

pub mod color {
    use super::Color;

    pub const WHITE: Color = 0;
    pub const BLACK: Color = 1;
    pub const NONE: Color = 2;
}

pub mod square {
    use super::Square;

    pub const A1: Square = 0;
    pub const B1: Square = 1;
    pub const C1: Square = 2;
    pub const D1: Square = 3;
    pub const E1: Square = 4;
    pub const F1: Square = 5;
    pub const G1: Square = 6;
    pub const H1: Square = 7;
    pub const A2: Square = 8;
    pub const B2: Square = 9;
    pub const C2: Square = 10;
    pub const D2: Square = 11;
    pub const E2: Square = 12;
    pub const F2: Square = 13;
    pub const G2: Square = 14;
    pub const H2: Square = 15;
    pub const A3: Square = 16;
    pub const B3: Square = 17;
    pub const C3: Square = 18;
    pub const D3: Square = 19;
    pub const E3: Square = 20;
    pub const F3: Square = 21;
    pub const G3: Square = 22;
    pub const H3: Square = 23;
    pub const A4: Square = 24;
    pub const B4: Square = 25;
    pub const C4: Square = 26;
    pub const D4: Square = 27;
    pub const E4: Square = 28;
    pub const F4: Square = 29;
    pub const G4: Square = 30;
    pub const H4: Square = 31;
    pub const A5: Square = 32;
    pub const B5: Square = 33;
    pub const C5: Square = 34;
    pub const D5: Square = 35;
    pub const E5: Square = 36;
    pub const F5: Square = 37;
    pub const G5: Square = 38;
    pub const H5: Square = 39;
    pub const A6: Square = 40;
    pub const B6: Square = 41;
    pub const C6: Square = 42;
    pub const D6: Square = 43;
    pub const E6: Square = 44;
    pub const F6: Square = 45;
    pub const G6: Square = 46;
    pub const H6: Square = 47;
    pub const A7: Square = 48;
    pub const B7: Square = 49;
    pub const C7: Square = 50;
    pub const D7: Square = 51;
    pub const E7: Square = 52;
    pub const F7: Square = 53;
    pub const G7: Square = 54;
    pub const H7: Square = 55;
    pub const A8: Square = 56;
    pub const B8: Square = 57;
    pub const C8: Square = 58;
    pub const D8: Square = 59;
    pub const E8: Square = 60;
    pub const F8: Square = 61;
    pub const G8: Square = 62;
    pub const H8: Square = 63;
    pub const NONE: Square = 64;
}