use crate::aliases::{Bitboard, Move};

// Using From-To based move encoding
//
//      0    |   0   |    0    |    0    | 000000 | 000000
//  ---------|-------|---------|---------|--------|--------
//  promotion|capture|special 1|special 0|  from  |   to
//
// https://www.chessprogramming.org/Encoding_Moves

fn move_get_to(mov: Move) -> u8 {
    (mov & 0x3f) as u8
}

fn move_get_from(mov: Move) -> u8 {
    ((mov >> 6) & 0x3f) as u8
}

fn move_get_code(mov: Move) -> u8 {
    ((mov >> 12) & 0xf) as u8
}

fn make_move(from: u8, to: u8, special: u8) -> Move {
    (to as Move) | ((from as Move) << 6) | ((special as Move) << 12)
}

const QUIET_MOVE: u8 = 0;
const DOUBLE_PAWN_PUSH: u8 = 1;
const KING_CASTLE: u8 = 2;
const QUEEN_CASTLE: u8 = 3;
const CAPTURES: u8 = 4;
const EP_CAPTURE: u8 = 5;
const KNIGHT_PROM: u8 = 8;
const BISHOP_PROM: u8 = 9;
const ROOK_PROM: u8 = 10;
const QUEEN_PROM: u8 = 11;
const NP_CAPTURE: u8 = 12;
const BP_CAPTURE: u8 = 13;
const RP_CAPTURE: u8 = 14;
const QP_CAPTURE: u8 = 15;

const WKING_BIT_BOARD: Bitboard = 1 << 4;
const WQUEEN_BIT_BOARD: Bitboard = 1 << 3;
const WROOK_BIT_BOARD: Bitboard = 1 << 0 | 1 << 7;
const WBISHOP_BIT_BOARD: Bitboard = 1 << 2 | 1 << 5;
const WKNIGHT_BIT_BOARD: Bitboard = 1 << 1 | 1 << 6;
const WPAWN_BIT_BOARD: Bitboard =
    1 << 8 | 1 << 9 | 1 << 10 | 1 << 11 | 1 << 12 | 1 << 13 | 1 << 14 | 1 << 15;

const BKING_BIT_BOARD: Bitboard = 1 << 60;
const BQUEEN_BIT_BOARD: Bitboard = 1 << 59;
const BROOK_BIT_BOARD: Bitboard = 1 << 56 | 1 << 63;
const BBISHOP_BIT_BOARD: Bitboard = 1 << 58 | 1 << 61;
const BKNIGHT_BIT_BOARD: Bitboard = 1 << 57 | 1 << 62;
const BPAWN_BIT_BOARD: Bitboard =
    1 << 48 | 1 << 49 | 1 << 50 | 1 << 51 | 1 << 52 | 1 << 53 | 1 << 54 | 1 << 55;

#[rustfmt::skip]
enum EnumSquare {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Colour {
    White,
    Black,
}

impl Colour {
    pub fn values() -> [Self; 2] {
        [Self::White, Self::Black]
    }
}

#[derive(Copy, Clone)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Piece {
    pub fn values() -> [Self; 6] {
        [
            Self::King,
            Self::Queen,
            Self::Rook,
            Self::Bishop,
            Self::Knight,
            Self::Pawn,
        ]
    }
}

#[derive(Copy, Clone)]
pub struct Position {
    bitboards: [[Bitboard; 6]; 2],
    side: Colour,
}

impl Position {
    fn square_repr(&self, idx: u8) -> char {
        for colour in Colour::values() {
            for piece in Piece::values() {
                let bitboard = self.bitboards[colour as usize][piece as usize];
                if (bitboard >> idx) & 1 == 1 {
                    match "♔♕♖♗♘♙♚♛♜♝♞♟"
                        .chars()
                        .nth(6 * colour as usize + piece as usize)
                    {
                        Some(c) => return c,
                        None => panic!(),
                    }
                }
            }
        }
        '.'
    }

    pub fn print(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                print!("|{}", self.square_repr(8 * rank + file));
            }
            println!("|");
        }
    }

    pub fn generate_pseudo_legal(&self) -> Vec<Move> {
        let pieces_bb = match self.side {
            Colour::White => self.bitboards[0],
            Colour::Black => self.bitboards[1],
        };

        let mut moves: Vec<Move> = Vec::new();

        for piece in Piece::values() {
            let piece_bb = pieces_bb[piece as usize];
            println!("{:?}", serialize_bb(piece_bb));
            for piece_bb in serialize_bb(piece_bb) {
                // TODO: Add additional information such as enpassant and castling
                let mut pos_moves = match piece {
                    Piece::King => gen_king_moves(piece_bb),
                    Piece::Queen => gen_queen_moves(piece_bb),
                    Piece::Rook => gen_rook_moves(piece_bb),
                    Piece::Bishop => gen_bishop_moves(piece_bb),
                    Piece::Knight => gen_knight_moves(piece_bb),
                    Piece::Pawn => gen_pawn_moves(piece_bb),
                };
                moves.append(&mut pos_moves);
            }
        }
        moves
    }
}

pub fn gen_king_moves(_idx: u8) -> Vec<Move> {
    vec![1, 2]
}

pub fn gen_queen_moves(_idx: u8) -> Vec<Move> {
    vec![1, 2]
}

pub fn gen_rook_moves(_idx: u8) -> Vec<Move> {
    vec![1, 2]
}

pub fn gen_bishop_moves(_idx: u8) -> Vec<Move> {
    vec![1, 2]
}

pub fn gen_knight_moves(_idx: u8) -> Vec<Move> {
    vec![1, 2]
}

pub fn gen_pawn_moves(_idx: u8) -> Vec<Move> {
    vec![1, 2]
}

pub fn init_chess() -> Position {
    Position {
        bitboards: [
            [
                WKING_BIT_BOARD,
                WQUEEN_BIT_BOARD,
                WROOK_BIT_BOARD,
                WBISHOP_BIT_BOARD,
                WKNIGHT_BIT_BOARD,
                WPAWN_BIT_BOARD,
            ],
            [
                BKING_BIT_BOARD,
                BQUEEN_BIT_BOARD,
                BROOK_BIT_BOARD,
                BBISHOP_BIT_BOARD,
                BKNIGHT_BIT_BOARD,
                BPAWN_BIT_BOARD,
            ],
        ],
        side: Colour::White,
    }
}

pub fn serialize_bb(bb: Bitboard) -> Vec<u8> {
    let mut bb = bb as i64;
    let mut set: Vec<u8> = Vec::new();
    while bb != 0 {
        let lsb = (bb & -bb) as Bitboard;
        set.push(lsb.trailing_zeros() as u8);
        bb &= bb - 1;
    }
    set
}

pub fn deserialize_bb(set: Vec<u8>) -> Bitboard {
    set.iter().fold(0, |acc, bb| acc ^ (1u64 << bb))
}
