use std::fmt;

pub mod movegen;

use crate::types::*;
use crate::bitboard::{BB_NONE, square_bb};
use movegen::AttackTable;

pub struct Position {
    pawns: Bitboard,
    knights: Bitboard,
    bishops: Bitboard,
    rooks: Bitboard,
    queens: Bitboard,
    kings: Bitboard,

    occupied: [Bitboard; 2],

    castling_rights: Bitboard,

    ep_square: Square,

    turn: Color,

    halfmove_count: u8,
    fullmove_count: u8,
}

impl Position {
    // create new standard chess starting position
    pub fn new() -> Self {
        let mut pos: Position = Self {
            pawns: BB_NONE,
            knights: BB_NONE,
            bishops: BB_NONE,
            rooks: BB_NONE,
            queens: BB_NONE,
            kings: BB_NONE,

            occupied: [BB_NONE; 2],

            castling_rights: BB_NONE,
            ep_square: square::NONE,

            turn: color::WHITE,

            halfmove_count: 0,
            fullmove_count: 0,
        };

        pos.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        pos
    }

    pub fn clear(&mut self) {
        self.pawns = BB_NONE;
        self.knights = BB_NONE;
        self.bishops = BB_NONE;
        self.rooks = BB_NONE;
        self.queens = BB_NONE;
        self.kings = BB_NONE;

        self.occupied = [BB_NONE; 2];

        self.castling_rights = BB_NONE;
        self.ep_square = square::NONE;

        self.turn = color::WHITE;

        self.halfmove_count = 0;
        self.fullmove_count = 0;
    }

    // returns the piece at a given square
    #[inline]
    pub fn piece_at(&self, sq: Square) -> Piece {
        let mask: Bitboard = square_bb(sq);

        if mask & self.pawns > 0 {
            return piece::PAWN
        }
        if mask & self.knights > 0 {
            return piece::KNIGHT
        }
        if mask & self.bishops > 0 {
            return piece::BISHOP
        }
        if mask & self.rooks > 0 {
            return piece::ROOK
        }
        if mask & self.queens > 0 {
            return piece::QUEEN
        }
        if mask & self.kings > 0 {
            return piece::KING
        }

        return piece::NONE
    }

    // returns the color at a given square
    #[inline]
    pub fn color_at(&self, sq: Square) -> Color {
        let mask: Bitboard = square_bb(sq);

        if mask & self.occupied[color::WHITE] > 0 {
            return color::WHITE
        }
        if mask & self.occupied[color::BLACK] > 0 {
            return color::BLACK
        }
        return color::NONE
    }

    pub fn parse_fen(&mut self, fen: &str) {
        let mut fen_data = fen.split(" ");
        let rank_data: Vec<&str> = fen_data.next().unwrap().split("/").collect();
        self.clear();
        
        // parse piece positions
        let mut sq: Square = 0;
        for rank in rank_data.iter().rev() {
            for ch in rank.chars() {
                if ch.is_ascii_uppercase(){
                    self.occupied[color::WHITE] |= square_bb(sq);
                } else if ch.is_ascii_lowercase() {
                    self.occupied[color::BLACK] |= square_bb(sq);
                }

                match ch {
                    '1'..='8' => {
                        sq += ch as Square - '0' as Square;
                    },
                    'p'|'P' => {
                        self.pawns |= square_bb(sq);
                        sq += 1;
                    },
                    'n'|'N' => {
                        self.knights |= square_bb(sq);
                        sq += 1;
                    },
                    'b'|'B' => {
                        self.bishops |= square_bb(sq);
                        sq += 1;
                    },
                    'r'|'R' => {
                        self.rooks |= square_bb(sq);
                        sq += 1;
                    },
                    'q'|'Q' => {
                        self.queens |= square_bb(sq);
                        sq += 1;
                    },
                    'k'|'K' => {
                        self.kings |= square_bb(sq);
                        sq += 1;
                    },
                    _ => ()
                }
            }
        }
        assert_eq!(sq, 64);

        // set turn (if it does not exist assume white to play and no castling rights/ep square)
        match fen_data.next() {
            Some("w") => self.turn = color::WHITE,
            Some("b") => self.turn = color::BLACK,
            _ => return
        }

        // set castling rights
        for ch in fen_data.next().unwrap().chars() {
            match ch {
                '-' => (),
                'K' => self.castling_rights |= square_bb(square::H1),
                'Q' => self.castling_rights |= square_bb(square::A1),
                'k' => self.castling_rights |= square_bb(square::H8),
                'q' => self.castling_rights |= square_bb(square::A8),
                _ => return
            }
        }

        // set en passant square
        match fen_data.next() {
            Some("-") => (),
            Some(sq_str) => {
                let sq_pos: Vec<char> = sq_str.chars().collect();
                self.ep_square = 8 * (sq_pos[1] as Square - '1' as Square) + (sq_pos[0] as Square - 'a' as Square);
            },
            _ => ()
        }

        // set halfmove count
        match fen_data.next() {
            Some(halfmove_str) => self.halfmove_count = halfmove_str.parse::<u8>().unwrap_or(0),
            _ => ()
        }

         // set fullmove count
         match fen_data.next() {
            Some(fullmove_str) => self.fullmove_count = fullmove_str.parse::<u8>().unwrap_or(0),
            _ => ()
        }

    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..8).rev() {
            for j in 0..8 {
                let mut piece_char: char = match self.piece_at(i*8 + j) {
                    piece::PAWN => 'P',
                    piece::KNIGHT => 'N',
                    piece::BISHOP => 'B',
                    piece::ROOK => 'R',
                    piece::QUEEN => 'Q',
                    piece::KING => 'K',
                    _ => '.'
                };
                if self.color_at(i*8 + j) == color::BLACK{
                    piece_char = piece_char.to_ascii_lowercase();
                }

                write!(f, "{} ", piece_char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}