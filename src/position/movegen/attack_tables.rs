use crate::types::*;
use crate::bitboard::*;
use super::magics::{ROOK_MAGICS, BISHOP_MAGICS};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref ATTACK_TABLE: AttackTable = AttackTable::new();
}

#[derive(Clone, Copy, Default)]
pub struct Magic {
    pub mask: Bitboard,
    pub magic: Bitboard,
    pub offset: u32,
    pub shift: u8
}

pub struct AttackTable {
    pawns: [[Bitboard; 64]; 2],
    knights: [Bitboard; 64],
    kings: [Bitboard; 64],

    rooks: [Bitboard; 102400],
    bishops: [Bitboard; 5248],

    rook_magics: [Magic; 64],
    bishop_magics: [Magic; 64],

    rays: [[Bitboard; 64]; 64]
}

// implement constructor
impl AttackTable {
    pub fn new() -> Self {
        let mut attack_table = Self {
            pawns: [[BB_NONE; 64]; 2],
            knights: [BB_NONE; 64],
            kings: [BB_NONE; 64],

            rooks: [BB_NONE; 102400],
            bishops: [BB_NONE; 5248],

            rook_magics: [Magic::default(); 64],
            bishop_magics: [Magic::default(); 64],

            rays: [[BB_NONE; 64]; 64]
        };

        attack_table.init_pawns();
        attack_table.init_knights();
        attack_table.init_kings();
        attack_table.init_rooks();
        attack_table.init_bishops();
        attack_table.init_rays();

        attack_table
    }
}

// implement sliding attack lookups
impl AttackTable {
    #[inline]
    pub fn get_pawn_attacks(&self, sq: Square, co: Color) -> Bitboard {
        self.pawns[co][sq]
    }

    // get attacks for knights & kings
    #[inline]
    pub fn get_jump_attacks(&self, sq: Square, pt: Piece) -> Bitboard {
        match pt {
            piece::KNIGHT => self.knights[sq],
            piece::KING => self.kings[sq],
            _ => panic!("Incorrect piece type. This function is for knight & king attacks only")
        }
    }

    // get attacks for rooks, bishops, queens
    #[inline]
    pub fn get_sliding_attacks(&self, sq: Square, pt: Piece, occupied: Bitboard) -> Bitboard {
        match pt {
            piece::ROOK => self.get_rook_attacks(sq, occupied),
            piece::BISHOP => self.get_bishop_attacks(sq, occupied),
            piece::QUEEN => self.get_rook_attacks(sq, occupied) | self.get_bishop_attacks(sq, occupied),
            _ => panic!("Incorrect piece type. This function is for sliding pieces only")
        }
    }

    // return ray between two squares
    #[inline]
    pub fn get_ray(&self, sq1: Square, sq2: Square) -> Bitboard {
        self.rays[sq1][sq2]
    }

    // return line between two endpoints inclusive
    #[inline]
    pub fn get_line(&self, sq1: Square, sq2: Square) -> Bitboard {
        self.get_ray(sq1, sq2) & self.get_ray(sq2, sq1)
    }

    // magic lookup for slider attacks

    #[inline]
    fn get_rook_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard{
        let m = self.rook_magics[sq];
        let index = (m.mask & occupied).wrapping_mul(m.magic) >> m.shift;

        self.rooks[index as usize + m.offset as usize]
    }

    #[inline]
    fn get_bishop_attacks(&self, sq: Square, occupied: Bitboard) -> Bitboard{
        let m = self.bishop_magics[sq];
        let index = (m.mask & occupied).wrapping_mul(m.magic) >> m.shift;

        self.bishops[index as usize + m.offset as usize]
    }
}

// implement functions that create attack table and magic tables
impl AttackTable {
    fn init_pawns(&mut self) {
        let deltas: [[i8; 2]; 2] = [[7, 9], [-7, -9]];

        let color_index = color::WHITE;
        for sq in 0..64 {
            self.pawns[color_index][sq] = step_mask(sq, &deltas[color_index]);
        }

        let color_index: usize = color::BLACK;
        for sq in 0..64 {
            self.pawns[color_index][sq] = step_mask(sq, &deltas[color_index]);
        }
    }

    fn init_knights(&mut self) {
        let deltas = [17, 15, 10, 6, -17, -15, -10, -6];

        for sq in 0..64 {
            self.knights[sq as usize] = step_mask(sq, &deltas);
        }
    }

    fn init_kings(&mut self) {
        let deltas = [9, 8, 7, 1, -9, -8, -7, -1];

        for sq in 0..64 {
            self.kings[sq] = step_mask(sq, &deltas);
        }
    }

    fn init_rooks(&mut self) {
        let deltas = [-1, 1, -8, 8];
        let mut offset: usize = 0;
        for sq in 0..64 {
            // generate the mask
            let edge_mask: Bitboard = ((BB_RANK_1 | BB_RANK_8) & !rank_bb(sq)) | (BB_FILE_A | BB_FILE_H) & !file_bb(sq);
            let mask: Bitboard = slider_mask(sq, &deltas, BB_NONE) & !edge_mask;

            // how much to left shift to get the relevant bits
            let shift: u8 = 64-popcount(mask);

            // iterate over every subset of the mask and set the attack mask in the lookup table
            let mut occupied: Bitboard = BB_NONE;
            loop {
                let index: usize  = ((occupied.wrapping_mul(ROOK_MAGICS[sq])) >> shift) as usize;
                self.rooks[offset+index] = slider_mask(sq, &deltas, occupied);

                occupied = (occupied.wrapping_sub(mask)) & mask;
                if occupied == 0 {
                    break
                }
            }
        
            
            self.rook_magics[sq] = Magic {
                mask: mask,
                magic: ROOK_MAGICS[sq],
                offset: offset as u32,
                shift: shift
            };
            
            // offset depends on the number of masked bits
            offset += 1 << (64-shift);
        }
    }

    fn init_bishops(&mut self) {
        let deltas = [-7, 7, -9, 9];
        let mut offset: usize = 0;
        for sq in 0..64 {
            // generate the mask
            let edge_mask: Bitboard = ((BB_RANK_1 | BB_RANK_8) & !rank_bb(sq)) | (BB_FILE_A | BB_FILE_H) & !file_bb(sq);
            let mask: Bitboard = slider_mask(sq, &deltas, BB_NONE) & !edge_mask;

            // how much to left shift to get the relevant bits
            let shift: u8 = 64-popcount(mask);

            // iterate over every subset of the mask and set the attack mask in the lookup table
            let mut occupied: Bitboard = BB_NONE;
            loop {
                let index: usize  = ((occupied.wrapping_mul(BISHOP_MAGICS[sq])) >> shift) as usize;
                self.bishops[offset+index] = slider_mask(sq, &deltas, occupied);

                occupied = (occupied.wrapping_sub(mask)) & mask;
                if occupied == 0 {
                    break
                }
            }
        
            
            self.bishop_magics[sq] = Magic {
                mask: mask,
                magic: BISHOP_MAGICS[sq],
                offset: offset as u32,
                shift: shift
            };
            
            // offset depends on the number of masked bits
            offset += 1 << (64-shift);
        }
    }

    fn init_rays(&mut self) {
        let directions = [-9, -8, -7, -1, 1, 7, 8, 9];

        let mut from_square: i8 = 0;
        while from_square < 64 {
            for dir in directions {
                // generate bitmask
                let mut bitmask: Bitboard = BB_NONE;
                let mut prev_square: i8 = from_square;

                bitmask |= square_bb(prev_square as Square);

                let mut curr_square: i8 = prev_square + dir;

                while curr_square >= 0 && curr_square <= 63 && chebyshev_distance(prev_square as Square, curr_square as Square) < 2 {
                    bitmask |= square_bb(curr_square as Square);

                    prev_square = curr_square;
                    curr_square += dir;
                }

                // for every square on ray, rays[from_square][to_square] = bitmask
                curr_square = prev_square;
                while curr_square != from_square {
                    self.rays[from_square as Square][curr_square as Square] = bitmask;

                    curr_square -= dir;
                }

                self.rays[from_square as Square][from_square as Square] = square_bb(from_square as Square);
            }
            from_square += 1;
        }
    }
}

// generates a mask of possible sliding moves
// a sliding piece can take but not go past an occupied square
#[inline]
fn slider_mask(sq: Square, deltas: &[i8], occupied: Bitboard) -> Bitboard {
    let mut attacks = BB_NONE;
    for delta in deltas {
        let mut curr_sq = sq as i8;

        loop {
            if curr_sq + delta < 0 || curr_sq + delta > 63 || chebyshev_distance(curr_sq as Square, (curr_sq + delta) as Square) > 2 {
                break;
            }
            curr_sq += delta;

            let curr_bb: Bitboard = square_bb(curr_sq as Square);
            attacks |= curr_bb;

            if occupied & curr_bb > 0 {
                break;
            }
        }
    }

    attacks
}

// generate attacks where piece can only move one step
fn step_mask(sq: Square, deltas: &[i8]) -> Bitboard {
    slider_mask(sq, deltas, BB_ALL)
}