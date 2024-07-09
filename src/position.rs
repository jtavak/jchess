use std::fmt;

use crate::types::*;
use crate::bitboard::*;

#[derive(Default, Copy, Clone)]
pub struct Position {
    pub pawns: Bitboard,
    pub knights: Bitboard,
    pub bishops: Bitboard,
    pub rooks: Bitboard,
    pub queens: Bitboard,
    pub kings: Bitboard,

    pub occupied: [Bitboard; 2],

    pub castling_rights: Bitboard,

    pub ep_square: Square,

    pub turn: Color,

    pub halfmove_count: u8,
    pub fullmove_count: u8,
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

// implement move making features
impl Position {
    pub fn make(&mut self, mv: &Move) {
        // increment move counters
        self.halfmove_count += 1;
        if self.turn == color::BLACK {
            self.fullmove_count += 1;
        }

        // TODO: implement halfmove clock zeroing

        let from_bb: Bitboard = square_bb(mv.from_square);
        let to_bb: Bitboard = square_bb(mv.to_square);

        let mut piece_type: Piece = self.remove_piece_at(mv.from_square);

        // castling rights
        self.castling_rights &= !(from_bb | to_bb);
        if piece_type == piece::KING {
            // update castling rights on king move
            if self.turn == color::WHITE {
                self.castling_rights &= !BB_RANK_1;
            } else {
                self.castling_rights &= !BB_RANK_8;
            }
        }

        // en passant related things
        let prev_ep_square: Square = self.ep_square;
        self.ep_square = square::NONE;

        if piece_type == piece::PAWN {
            let delta: i8 = mv.to_square as i8 - mv.from_square as i8;

            if delta == 16 {
                self.ep_square = mv.from_square + 8;
            } else if delta == -16 {
                self.ep_square = mv.from_square - 8;
            } else if mv.to_square == prev_ep_square {
                self.remove_piece_at((prev_ep_square as i8 + if self.turn == color::WHITE {-8} else {8}) as Square);
            }
        }

        // handle pawn promotions
        if mv.promotion != piece::NONE {
            piece_type = mv.promotion;
        }

        // add new piece, also handle castling
        if piece_type == piece::KING && chebyshev_distance(mv.from_square, mv.to_square) > 1 {
            if square_file(mv.to_square) < square_file(mv.from_square) {
                self.set_piece_at(if self.turn == color::WHITE {square::C1} else {square::C8}, piece::KING, self.turn);
                self.set_piece_at(if self.turn == color::WHITE {square::D1} else {square::D8}, piece::ROOK, self.turn);
                self.remove_piece_at(if self.turn == color::WHITE {square::A1} else {square::A8});
            } else {
                self.set_piece_at(if self.turn == color::WHITE {square::G1} else {square::G8}, piece::KING, self.turn);
                self.set_piece_at(if self.turn == color::WHITE {square::F1} else {square::F8}, piece::ROOK, self.turn);
                self.remove_piece_at(if self.turn == color::WHITE {square::H1} else {square::H8});
            }
        } else {
            self.set_piece_at(mv.to_square, piece_type, self.turn);
        }


        self.turn ^= 1;
    }

    // remove piece (assuming it's already there)
    fn remove_piece_at(&mut self, sq: Square) -> Piece {
        let piece_type: Piece = self.piece_at(sq);
        let piece_color: Color = self.color_at(sq);
        let piece_bb: Bitboard = !square_bb(sq);

        match piece_type {
            piece::PAWN => self.pawns &= piece_bb,
            piece::KNIGHT => self.knights &= piece_bb,
            piece::BISHOP => self.bishops &= piece_bb,
            piece::ROOK => self.rooks &= piece_bb,
            piece::QUEEN => self.queens &= piece_bb,
            piece::KING => self.kings &= piece_bb,
            piece::NONE => return piece_type,
            _ => ()
        }

        self.occupied[piece_color] &= piece_bb;

        piece_type
    }

    fn set_piece_at(&mut self, sq: Square, piece_type: Piece, piece_color: Color) {
        self.remove_piece_at(sq);

        let piece_bb: Bitboard = square_bb(sq);
        match piece_type {
            piece::PAWN => self.pawns |= piece_bb,
            piece::KNIGHT => self.knights |= piece_bb,
            piece::BISHOP => self.bishops |= piece_bb,
            piece::ROOK => self.rooks |= piece_bb,
            piece::QUEEN => self.queens |= piece_bb,
            piece::KING => self.kings |= piece_bb,
            _ => ()
        }

        self.occupied[piece_color] |= piece_bb;
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
        writeln!(f, "\n{} to play\n", if self.turn == color::WHITE {"White"} else {"Black"})?;
        Ok(())
    }
}