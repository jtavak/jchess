pub mod attack_tables;
mod magics;

use crate::types::*;
use crate::bitboard::*;

use crate::position::Position;
use attack_tables::ATTACK_TABLE;

pub fn gen_legal_moves(pos: &Position, move_list: &mut [Move; 256], move_count: &mut usize) {
    assert_eq!(*move_count, 0);

    if is_check(pos) {
        gen_evasions(pos, move_list, move_count);
    } else {
        gen_masked_pseudo_legal_moves(pos, move_list, move_count, BB_ALL, BB_ALL);
    }
    // iterate over pseudo legal moves, getting rid of illegal moves
    let mut i: usize = 0;
    let mut j: usize = 0;
    while j < *move_count {
        if is_legal(pos, &move_list[j]) {
            move_list[i] = move_list[j];
            i += 1;
        }
        j += 1;
    }
    *move_count = i;
}

// generate all pseudo-legal moves, not caring about issues with checks, etc.
fn gen_masked_pseudo_legal_moves(pos: &Position, move_list: &mut [Move; 256], move_count: &mut usize, from_mask: Bitboard, to_mask: Bitboard) {      
    let occupied: Bitboard = pos.occupied[color::WHITE] | pos.occupied[color::BLACK];
    let self_occupied: Bitboard = pos.occupied[pos.turn];
    let opponent_occupied: Bitboard = pos.occupied[pos.turn ^ 1];

    let pawns: Bitboard = self_occupied & pos.pawns & from_mask;

    // generate pawn attacks
    let mut pawn_bb: Bitboard = pawns;
    while pawn_bb > 0 {
        let from_square: Square = pop_lsb(&mut pawn_bb);

        let mut attacks: Bitboard = attacks_from_square(pos, from_square) & opponent_occupied & to_mask;
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

        let mut attacks: Bitboard = attacks_from_square(pos, from_square) & to_mask;
        while attacks > 0 {
            let to_square: Square = pop_lsb(&mut attacks);
            move_list[*move_count] = Move {from_square, to_square, promotion: piece::NONE};
            *move_count += 1;
        }

    }

    // generate castling moves
    if self_occupied & pos.kings & from_mask > 0 {
        let king: Square = lsb(self_occupied & pos.kings & from_mask);
        let backrank: Bitboard = if pos.turn == color::WHITE {BB_RANK_1} else {BB_RANK_8};
        
        let mut castling_squares: Bitboard = pos.castling_rights & backrank;
        while castling_squares > 0 {
            let candidate_square: Square = pop_lsb(&mut castling_squares);

            if king < candidate_square && CASTLE_BLOCKER_MASK_KINGSIDE & backrank & occupied == 0 && square_bb(king+2) & to_mask > 0 {
                // kingside castling
                if !is_attacked(pos, lsb(BB_FILE_E & backrank)) &&
                !is_attacked(pos, lsb(BB_FILE_F & backrank)) &&
                !is_attacked(pos, lsb(BB_FILE_G & backrank)) {
                    move_list[*move_count] = Move {
                        from_square: king,
                        to_square: king + 2,
                        promotion: piece::NONE
                    };
                    *move_count += 1;
                }
            } else if king > candidate_square && CASTLE_BLOCKER_MASK_QUEENSIDE & backrank & occupied == 0 && square_bb(king-2) & to_mask > 0 {
                // queenside castling
                if !is_attacked(pos, lsb(BB_FILE_C & backrank)) &&
                !is_attacked(pos, lsb(BB_FILE_D & backrank)) &&
                !is_attacked(pos, lsb(BB_FILE_E & backrank)) {
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
    if pos.turn == color::WHITE {
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
    if pos.ep_square != square::NONE && square_bb(pos.ep_square) & to_mask > 0 {
        let mut capturers = pawns & self_occupied & ATTACK_TABLE.get_pawn_attacks(pos.ep_square, pos.turn ^ 1) & from_mask;
        while capturers > 0 {
            let from_square = pop_lsb(&mut capturers);

            move_list[*move_count] = Move {from_square, to_square: pos.ep_square, promotion: piece::NONE};
            *move_count += 1;
        }
    }
}

// generate moves that get the king out of check.
fn gen_evasions(pos: &Position, move_list: &mut [Move; 256], move_count: &mut usize) {
    let king: Square = lsb(pos.kings & pos.occupied[pos.turn]);
    let checkers: Bitboard = attackers_mask(pos, king, pos.turn);

    // generate attack rays from the sliding pieces towards the king
    let mut sliders: Bitboard = checkers & (pos.rooks | pos.bishops | pos.queens);
    let mut attack_mask: Bitboard = BB_NONE;
    while sliders > 0 {
        let slider_square = pop_lsb(&mut sliders);
        attack_mask |= ATTACK_TABLE.get_ray(slider_square, king) & !square_bb(slider_square);
    }
    
    // generate king moves
    let mut king_moves: Bitboard = attacks_from_square(pos, king) & !attack_mask;
    while king_moves > 0 {
        let king_move: Square = pop_lsb(&mut king_moves);
        if !is_attacked(pos, king_move) {
            move_list[*move_count] = Move {from_square: king, to_square: king_move, promotion: piece::NONE};
            *move_count += 1;
        }
    }

    // if king is in double check, skip generating other moves
    if popcount(checkers) > 1 {
        return
    }

    let checker_square: Square = lsb(checkers);
    let checker_type: Piece = pos.piece_at(checker_square);

    let blocking_mask: Bitboard;
    match checker_type {
        piece::BISHOP | piece::ROOK | piece::QUEEN => {
            // if checked by bishop/rook/queen, generate moves that capture or block
            blocking_mask = ATTACK_TABLE.get_line(checker_square, king) & !pos.kings;
        }
        piece::PAWN => {
            // handle case where checking pawn can be en passanted
            if pos.ep_square != square::NONE {
                gen_masked_pseudo_legal_moves(pos, move_list, move_count, BB_ALL & pos.pawns, square_bb(pos.ep_square));
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

    gen_masked_pseudo_legal_moves(pos, move_list, move_count, BB_ALL & !pos.kings, blocking_mask);
}

// generates all possible attacks from a square, excluding self captures and en passant
#[inline]
fn attacks_mask(pos: &Position, sq: Square, pt: Piece, co: Color) -> Bitboard {
    (match pt {
        piece::PAWN => ATTACK_TABLE.get_pawn_attacks(sq, co),
        piece::KNIGHT | piece::KING => ATTACK_TABLE.get_jump_attacks(sq, pt),
        piece::ROOK | piece::BISHOP | piece::QUEEN => ATTACK_TABLE.get_sliding_attacks(sq, pt, pos.occupied[color::WHITE] | pos.occupied[color::BLACK]),
        piece::NONE => BB_NONE,
        _ => panic!()
    }) & !pos.occupied[co]
}

// returns attack mask for a given square using piece and color at that square
#[inline]
fn attacks_from_square(pos: &Position, sq: Square) -> Bitboard {
    attacks_mask(pos, sq, pos.piece_at(sq), pos.color_at(sq))
}

#[inline]
fn is_attacked(pos: &Position, sq: Square) -> bool {
    attackers_mask(pos, sq, pos.turn) > 0
}

// returns a mask of all pieces attacking a square. co is the color of the side being attacked. Excluding en passant
#[inline]
fn attackers_mask(pos: &Position, sq: Square, co: Color) -> Bitboard {
    let mut attackers = BB_NONE;

    attackers |= pos.pawns & attacks_mask(pos, sq, piece::PAWN, co);
    attackers |= pos.knights & attacks_mask(pos, sq, piece::KNIGHT, co);
    attackers |= pos.bishops & attacks_mask(pos, sq, piece::BISHOP, co);
    attackers |= pos.rooks & attacks_mask(pos, sq, piece::ROOK, co);
    attackers |= pos.queens & attacks_mask(pos, sq, piece::QUEEN, co);
    attackers |= pos.kings & attacks_mask(pos, sq, piece::KING, co);

    attackers
}

#[inline]
fn is_check(pos: &Position) -> bool {
    let king: Square = lsb(pos.kings & pos.occupied[pos.turn]);
    is_attacked(pos, king)
}

fn is_legal(pos: &Position, mv: &Move) -> bool {
    // king can't move into check
    if pos.piece_at(mv.from_square) == piece::KING && is_attacked(pos, mv.to_square){
        return false;
    }

    let occupied: Bitboard = pos.occupied[color::WHITE] | pos.occupied[color::BLACK];
    let self_occupied: Bitboard = pos.occupied[pos.turn];
    let opp_occupied: Bitboard = pos.occupied[pos.turn^1];
    

    // handle rook pins (non-ep). to do this, pretend piece is slider and see if it can attack its own king and enemy slider of the same type
    // (en passant trivial pins covered)
    let rook_attacks: Bitboard = ATTACK_TABLE.get_sliding_attacks(mv.from_square, piece::ROOK, occupied);
    
    // we need to handle vertical and horizontal seperately
    let vertical_attacks: Bitboard = rook_attacks & file_bb(mv.from_square);
    if vertical_attacks & self_occupied & pos.kings > 0 && vertical_attacks & opp_occupied & (pos.rooks | pos.queens) > 0 {
        // if to_square is not between the attacker and king, move is illegal
        return square_bb(mv.to_square) & vertical_attacks > 0;
    }

    let horizontal_attacks: Bitboard = rook_attacks & rank_bb(mv.from_square);
    if horizontal_attacks & self_occupied & pos.kings > 0 && horizontal_attacks & opp_occupied & (pos.rooks | pos.queens) > 0 {
        return square_bb(mv.to_square) & horizontal_attacks > 0;
    }

    // bishop pins (en passant trivial pins covered)
    let bishop_attacks: Bitboard = ATTACK_TABLE.get_sliding_attacks(mv.from_square, piece::BISHOP, occupied);

    let asc_attacks: Bitboard = bishop_attacks & diag_asc_bb(mv.from_square);
    if asc_attacks & self_occupied & pos.kings > 0 && asc_attacks & opp_occupied & (pos.bishops | pos.queens) > 0 {
        return square_bb(mv.to_square) & asc_attacks > 0;
    }

    let desc_attacks: Bitboard = bishop_attacks & diag_desc_bb(mv.from_square);
    if desc_attacks & self_occupied & pos.kings > 0 && desc_attacks & opp_occupied & (pos.bishops | pos.queens) > 0 {
        return square_bb(mv.to_square) & desc_attacks > 0;
    }

    // en-passant non-trivial pins
    if pos.piece_at(mv.from_square) == piece::PAWN && pos.piece_at(mv.to_square) == piece::NONE && square_file(mv.from_square) != square_file(mv.to_square) {
        let captured_pawn: Square = (pos.ep_square as i8 + if pos.turn == color::WHITE {-8} else {8}) as Square;

        // simulate making EP move
        let ep_board: Bitboard = occupied & !square_bb(captured_pawn) & !square_bb(mv.from_square) | square_bb(pos.ep_square);

        // horizontal attacks
        if ATTACK_TABLE.get_sliding_attacks(lsb(pos.kings & self_occupied), piece::ROOK, ep_board) & opp_occupied & (pos.rooks | pos.queens) > 0 {
            return false;
        }

        // diagonal attacks
        if ATTACK_TABLE.get_sliding_attacks(lsb(pos.kings & self_occupied), piece::BISHOP, ep_board) & opp_occupied & (pos.bishops | pos.queens) > 0 {
            return false;
        }
    }

    true
}