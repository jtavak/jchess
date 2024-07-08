use std::cmp;

use crate::types::*;

pub const BB_NONE: Bitboard = 0x0;
pub const BB_ONE: Bitboard = 0x1;
pub const BB_ALL: Bitboard = 0xffffffffffffffff;

pub const BB_RANK_1: Bitboard = 0xff << (8*0);
pub const BB_RANK_2: Bitboard = 0xff << (8*1);
pub const BB_RANK_3: Bitboard = 0xff << (8*2);
pub const BB_RANK_4: Bitboard = 0xff << (8*3);
pub const BB_RANK_5: Bitboard = 0xff << (8*4);
pub const BB_RANK_6: Bitboard = 0xff << (8*5);
pub const BB_RANK_7: Bitboard = 0xff << (8*6);
pub const BB_RANK_8: Bitboard = 0xff << (8*7);

pub const BB_FILE_A: Bitboard = 0x101010101010101 << 0;
pub const BB_FILE_B: Bitboard = 0x101010101010101 << 1;
pub const BB_FILE_C: Bitboard = 0x101010101010101 << 2;
pub const BB_FILE_D: Bitboard = 0x101010101010101 << 3;
pub const BB_FILE_E: Bitboard = 0x101010101010101 << 4;
pub const BB_FILE_F: Bitboard = 0x101010101010101 << 5;
pub const BB_FILE_G: Bitboard = 0x101010101010101 << 6;
pub const BB_FILE_H: Bitboard = 0x101010101010101 << 7;

pub const BB_DIAG_ASC_0: Bitboard = 0x8040201008040201;
pub const BB_DIAG_DESC_0: Bitboard = 0x102040810204080;

// pieces on these squares prevent castling
pub const CASTLE_BLOCKER_MASK_KINGSIDE: Bitboard = (BB_FILE_F | BB_FILE_G) & (BB_RANK_1 | BB_RANK_8);
pub const CASTLE_BLOCKER_MASK_QUEENSIDE: Bitboard = (BB_FILE_B | BB_FILE_C | BB_FILE_D) & (BB_RANK_1 | BB_RANK_8);

#[inline]
pub const fn square_bb(sq: Square) -> Bitboard {
    BB_ONE << sq
}

#[inline]
pub const fn rank_bb(sq: Square) -> Bitboard {
    0xff << 8 * square_rank(sq)
}

#[inline]
pub const fn file_bb(sq: Square) -> Bitboard {
    0x101010101010101 << square_file(sq)
}

#[inline]
pub const fn diag_asc_bb(sq: Square) -> Bitboard {
    let shift = square_diag_asc(sq);
    if shift > 0 {
        BB_DIAG_ASC_0.overflowing_shl(8*shift as u32).0
    } else {
        BB_DIAG_ASC_0.overflowing_shr(8*-shift as u32).0
    }
}

#[inline]
pub const fn diag_desc_bb(sq: Square) -> Bitboard {
    let shift = square_diag_desc(sq);
    if shift > 0 {
        BB_DIAG_DESC_0.overflowing_shl(8*shift as u32).0
    } else {
        BB_DIAG_DESC_0.overflowing_shr(8*-shift as u32).0
    }
}

#[inline]
pub fn lsb(bb: Bitboard) -> Square {
    bb.trailing_zeros() as Square
}

#[inline]
pub fn msb(bb: Bitboard) -> Square {
    bb.leading_zeros() as Square
}

#[inline]
pub fn pop_lsb(bb: &mut Bitboard) -> Square {
    let lsb = lsb(*bb);
    *bb &= *bb - 1;
    lsb
}

#[inline]
pub fn popcount(bb: Bitboard) -> u8 {
    bb.count_ones() as u8
}

#[inline]
pub const fn square_rank(sq: Square) -> u8 {
    sq as u8 >> 3
}

#[inline]
pub const fn square_file(sq: Square) -> u8 {
    sq as u8 & 7
}

// amount of king moves from sq1 to sq2
#[inline]
pub fn chebyshev_distance(sq1: Square, sq2: Square) -> u8 {
    let rank_diff = square_rank(sq2) as i8 - square_rank(sq1) as i8;
    let file_diff = square_file(sq2) as i8 - square_file(sq1) as i8;
    cmp::max(rank_diff.abs(), file_diff.abs()) as u8
}

pub fn print_bitboard(bb: Bitboard) {
    for i in (0..8).rev() {
        for j in 0..8 {
            let square = i*8 + j;
            print!("{} ", (bb & (BB_ONE << square))>>square);
        }
        println!();
    }
    println!();
}

pub const SQUARE_NAMES: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
];

pub const PIECE_NAMES: [&str; 7] = ["Pawn", "Knight", "Bishop", "Rook", "Queen", "King", "None"];

#[inline]
const fn square_diag_asc(sq: Square) -> i8 {
    square_rank(sq) as i8 - square_file(sq) as i8
}

#[inline]
const fn square_diag_desc(sq: Square) -> i8 {
    square_rank(sq) as i8 + square_file(sq) as i8 - 7
}
