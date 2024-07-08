use std::fmt;

mod attack_tables;
mod magics;

use crate::types::*;
use crate::bitboard::*;

use attack_tables::*;

#[derive(Default, Copy, Clone)]
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

// implement move generation functions
impl Position {
    pub fn gen_legal_moves(&self, move_list: &mut [Move; 256], move_count: &mut usize) {
        assert_eq!(*move_count, 0);

        if self.is_check() {
            self.gen_evasions(move_list, move_count);
        } else {
            self.gen_masked_pseudo_legal_moves(move_list, move_count, BB_ALL, BB_ALL);
        }
        // iterate over pseudo legal moves, getting rid of illegal moves
        let mut i: usize = 0;
        let mut j: usize = 0;
        while j < *move_count {
            if self.is_legal(&move_list[j]) {
                move_list[i] = move_list[j];
                i += 1;
            }
            j += 1;
        }
        *move_count = i;
    }
    // generate all pseudo-legal moves, not caring about issues with checks, etc.
    fn gen_masked_pseudo_legal_moves(&self, move_list: &mut [Move; 256], move_count: &mut usize, from_mask: Bitboard, to_mask: Bitboard) {      
        let occupied: Bitboard = self.occupied[color::WHITE] | self.occupied[color::BLACK];
        let self_occupied: Bitboard = self.occupied[self.turn];
        let opponent_occupied: Bitboard = self.occupied[self.turn ^ 1];

        let pawns: Bitboard = self_occupied & self.pawns & from_mask;

        // generate pawn attacks
        let mut pawn_bb: Bitboard = pawns;
        while pawn_bb > 0 {
            let from_square: Square = pop_lsb(&mut pawn_bb);

            let mut attacks: Bitboard = self.attacks_from_square(from_square) & opponent_occupied & to_mask;
            while attacks > 0 {
                let to_square: Square = pop_lsb(&mut attacks);
                // if promotion
                if square_rank(to_square) == 0 || square_rank(to_square) == 7 {
                    move_list[*move_count] = Move {from_square, to_square, promotion: piece::QUEEN};
                    *move_count += 1;

                    move_list[*move_count] = Move {from_square, to_square, promotion: piece::ROOK};
                    *move_count += 1;

                    move_list[*move_count] = Move {from_square, to_square, promotion: piece::BISHOP};
                    *move_count += 1;

                    move_list[*move_count] = Move {from_square, to_square, promotion: piece::KNIGHT};
                    *move_count += 1;
                } else {
                    move_list[*move_count] = Move {from_square, to_square, promotion: piece::NONE};
                    *move_count += 1;
                }
            }
        }

        let pieces: Bitboard = self_occupied & !pawns & from_mask;

        // generate piece attacks
        let mut piece_bb: Bitboard = pieces;
        while piece_bb > 0 {
            let from_square: Square = pop_lsb(&mut piece_bb);

            let mut attacks: Bitboard = self.attacks_from_square(from_square) & to_mask;
            while attacks > 0 {
                let to_square: Square = pop_lsb(&mut attacks);
                move_list[*move_count] = Move {from_square, to_square, promotion: piece::NONE};
                *move_count += 1;
            }

        }

        // generate castling moves
        if self_occupied & self.kings & from_mask > 0 {
            let king: Square = lsb(self_occupied & self.kings & from_mask);
            let backrank: Bitboard = if self.turn == color::WHITE {BB_RANK_1} else {BB_RANK_8};
            
            let mut castling_squares: Bitboard = self.castling_rights & backrank;
            while castling_squares > 0 {
                let candidate_square: Square = pop_lsb(&mut castling_squares);

                if king < candidate_square && CASTLE_BLOCKER_MASK_KINGSIDE & backrank & occupied == 0 && square_bb(king+2) & to_mask > 0 {
                    // kingside castling
                    if !self.is_attacked(lsb(BB_FILE_E & backrank)) &&
                    !self.is_attacked(lsb(BB_FILE_F & backrank)) &&
                    !self.is_attacked(lsb(BB_FILE_G & backrank)) {
                        move_list[*move_count] = Move {
                            from_square: king,
                            to_square: king + 2,
                            promotion: piece::NONE
                        };
                        *move_count += 1;
                    }
                } else if king > candidate_square && CASTLE_BLOCKER_MASK_QUEENSIDE & backrank & occupied == 0 && square_bb(king-2) & to_mask > 0 {
                    // queenside castling
                    if !self.is_attacked(lsb(BB_FILE_C & backrank)) &&
                    !self.is_attacked(lsb(BB_FILE_D & backrank)) &&
                    !self.is_attacked(lsb(BB_FILE_E & backrank)) {
                        move_list[*move_count] = Move {
                            from_square: king,
                            to_square: king - 2,
                            promotion: piece::NONE
                        };
                        *move_count += 1;
                    }
                }
            }
        }

        // prepare pawn advance generation
        let mut single_advances: Bitboard;
        let mut double_advances: Bitboard;
        let single_delta: i8;
        if self.turn == color::WHITE {
            single_advances = (pawns << 8) & !occupied;
            double_advances = (single_advances << 8) & !occupied & BB_RANK_4 & to_mask;
            single_delta = 8;
        } else {
            single_advances = (pawns >> 8) & !occupied;
            double_advances = (single_advances >> 8) & !occupied & BB_RANK_5 & to_mask;
            single_delta = -8;
        }

        single_advances &= to_mask;

        // generate single pawn moves
        while single_advances > 0 {
            let to_square: Square = pop_lsb(&mut single_advances);
            let from_square: Square = (to_square as i8 - single_delta) as usize;
            
            // if promotion
            if square_rank(to_square) == 0 || square_rank(to_square) == 7 {
                move_list[*move_count] = Move {from_square, to_square, promotion: piece::QUEEN};
                *move_count += 1;

                move_list[*move_count] = Move {from_square, to_square, promotion: piece::ROOK};
                *move_count += 1;

                move_list[*move_count] = Move {from_square, to_square, promotion: piece::BISHOP};
                *move_count += 1;

                move_list[*move_count] = Move {from_square, to_square, promotion: piece::KNIGHT};
                *move_count += 1;
            } else {
                move_list[*move_count] = Move {from_square, to_square, promotion: piece::NONE};
                *move_count += 1;
            }
        }

        // generate double pawn moves
        while double_advances > 0 {
            let to_square: Square = pop_lsb(&mut double_advances);
            let from_square: Square = (to_square as i8 - single_delta*2) as usize;

            move_list[*move_count] = Move {from_square, to_square, promotion: piece::NONE};
            *move_count += 1;
        }

        // generate en passant
        if self.ep_square != square::NONE && square_bb(self.ep_square) & to_mask > 0 {
            let mut capturers = pawns & self_occupied & ATTACK_TABLE.get_pawn_attacks(self.ep_square, self.turn ^ 1) & from_mask;
            while capturers > 0 {
                let from_square = pop_lsb(&mut capturers);

                move_list[*move_count] = Move {from_square, to_square: self.ep_square, promotion: piece::NONE};
                *move_count += 1;
            }
        }
    }

    // generate moves that get the king out of check.
    fn gen_evasions(&self, move_list: &mut [Move; 256], move_count: &mut usize) {
        let king: Square = lsb(self.kings & self.occupied[self.turn]);
        let checkers: Bitboard = self.attackers_mask(king, self.turn);

        // generate attack rays from the sliding pieces towards the king
        let mut sliders: Bitboard = checkers & (self.rooks | self.bishops | self.queens);
        let mut attack_mask: Bitboard = BB_NONE;
        while sliders > 0 {
            let slider_square = pop_lsb(&mut sliders);
            attack_mask |= ATTACK_TABLE.get_ray(slider_square, king) & !square_bb(slider_square);
        }
        
        // generate king moves
        let mut king_moves: Bitboard = self.attacks_from_square(king) & !attack_mask;
        while king_moves > 0 {
            let king_move: Square = pop_lsb(&mut king_moves);
            if !self.is_attacked(king_move) {
                move_list[*move_count] = Move {from_square: king, to_square: king_move, promotion: piece::NONE};
                *move_count += 1;
            }
        }

        // if king is in double check, skip generating other moves
        if popcount(checkers) > 1 {
            return
        }

        let checker_square: Square = lsb(checkers);
        let checker_type: Piece = self.piece_at(checker_square);

        let blocking_mask: Bitboard;
        match checker_type {
            piece::BISHOP | piece::ROOK | piece::QUEEN => {
                // if checked by bishop/rook/queen, generate moves that capture or block
                blocking_mask = ATTACK_TABLE.get_line(checker_square, king) & !self.kings;
            }
            piece::PAWN => {
                // handle case where checking pawn can be en passanted
                if self.ep_square != square::NONE {
                    self.gen_masked_pseudo_legal_moves(move_list, move_count, BB_ALL & self.pawns, square_bb(self.ep_square));
                    blocking_mask = checkers;
                } else {
                    blocking_mask = checkers;
                }
                
            }
            piece::KNIGHT => {
                blocking_mask = checkers;
                
            }
            _ => panic!("Something other than a piece is checking your king :(")
        }

        self.gen_masked_pseudo_legal_moves(move_list, move_count, BB_ALL & !self.kings, blocking_mask);
    }

    // generates all possible attacks from a square, excluding self captures and en passant
    #[inline]
    fn attacks_mask(&self, sq: Square, pt: Piece, co: Color) -> Bitboard {
        (match pt {
            piece::PAWN => ATTACK_TABLE.get_pawn_attacks(sq, co),
            piece::KNIGHT | piece::KING => ATTACK_TABLE.get_jump_attacks(sq, pt),
            piece::ROOK | piece::BISHOP | piece::QUEEN => ATTACK_TABLE.get_sliding_attacks(sq, pt, self.occupied[color::WHITE] | self.occupied[color::BLACK]),
            piece::NONE => BB_NONE,
            _ => panic!()
        }) & !self.occupied[co]
    }

    // returns attack mask for a given square using piece and color at that square
    #[inline]
    fn attacks_from_square(&self, sq: Square) -> Bitboard {
        self.attacks_mask(sq, self.piece_at(sq), self.color_at(sq))
    }

    // returns a mask of all pieces attacking a square. co is the color of the side being attacked. Excluding en passant
    #[inline]
    fn attackers_mask(&self, sq: Square, co: Color) -> Bitboard {
        let mut attackers = BB_NONE;

        attackers |= self.pawns & self.attacks_mask(sq, piece::PAWN, co);
        attackers |= self.knights & self.attacks_mask(sq, piece::KNIGHT, co);
        attackers |= self.bishops & self.attacks_mask(sq, piece::BISHOP, co);
        attackers |= self.rooks & self.attacks_mask(sq, piece::ROOK, co);
        attackers |= self.queens & self.attacks_mask(sq, piece::QUEEN, co);
        attackers |= self.kings & self.attacks_mask(sq, piece::KING, co);

        attackers
    }

    #[inline]
    fn is_attacked(&self, sq: Square) -> bool {
        self.attackers_mask(sq, self.turn) > 0
    }

    #[inline]
    fn is_check(&self) -> bool {
        let king: Square = lsb(self.kings & self.occupied[self.turn]);
        self.is_attacked(king)
    }

    fn is_legal(&self, mv: &Move) -> bool {
        // king can't move into check
        if self.piece_at(mv.from_square) == piece::KING && self.is_attacked(mv.to_square){
            return false;
        }

        let occupied: Bitboard = self.occupied[color::WHITE] | self.occupied[color::BLACK];
        let self_occupied: Bitboard = self.occupied[self.turn];
        let opp_occupied: Bitboard = self.occupied[self.turn^1];
        

        // handle rook pins (non-ep). to do this, pretend piece is slider and see if it can attack its own king and enemy slider of the same type
        // (en passant trivial pins covered)
        let rook_attacks: Bitboard = ATTACK_TABLE.get_sliding_attacks(mv.from_square, piece::ROOK, occupied);
        
        // we need to handle vertical and horizontal seperately
        let vertical_attacks: Bitboard = rook_attacks & file_bb(mv.from_square);
        if vertical_attacks & self_occupied & self.kings > 0 && vertical_attacks & opp_occupied & (self.rooks | self.queens) > 0 {
            // if to_square is not between the attacker and king, move is illegal
            return square_bb(mv.to_square) & vertical_attacks > 0;
        }

        let horizontal_attacks: Bitboard = rook_attacks & rank_bb(mv.from_square);
        if horizontal_attacks & self_occupied & self.kings > 0 && horizontal_attacks & opp_occupied & (self.rooks | self.queens) > 0 {
            return square_bb(mv.to_square) & horizontal_attacks > 0;
        }

        // bishop pins (en passant trivial pins covered)
        let bishop_attacks: Bitboard = ATTACK_TABLE.get_sliding_attacks(mv.from_square, piece::BISHOP, occupied);

        let asc_attacks: Bitboard = bishop_attacks & diag_asc_bb(mv.from_square);
        if asc_attacks & self_occupied & self.kings > 0 && asc_attacks & opp_occupied & (self.bishops | self.queens) > 0 {
            return square_bb(mv.to_square) & asc_attacks > 0;
        }

        let desc_attacks: Bitboard = bishop_attacks & diag_desc_bb(mv.from_square);
        if desc_attacks & self_occupied & self.kings > 0 && desc_attacks & opp_occupied & (self.bishops | self.queens) > 0 {
            return square_bb(mv.to_square) & desc_attacks > 0;
        }

        // en-passant non-trivial pins
        if self.piece_at(mv.from_square) == piece::PAWN && self.piece_at(mv.to_square) == piece::NONE && square_file(mv.from_square) != square_file(mv.to_square) {
            let captured_pawn: Square = (self.ep_square as i8 + if self.turn == color::WHITE {-8} else {8}) as Square;

            // simulate making EP move
            let ep_board: Bitboard = occupied & !square_bb(captured_pawn) & !square_bb(mv.from_square) | square_bb(self.ep_square);

            // horizontal attacks
            if ATTACK_TABLE.get_sliding_attacks(lsb(self.kings & self_occupied), piece::ROOK, ep_board) & opp_occupied & (self.rooks | self.queens) > 0 {
                return false;
            }

            // diagonal attacks
            if ATTACK_TABLE.get_sliding_attacks(lsb(self.kings & self_occupied), piece::BISHOP, ep_board) & opp_occupied & (self.bishops | self.queens) > 0 {
                return false;
            }
        }
    
        true
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