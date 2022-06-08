use std::fmt;

type BitBoard = u64;

// Using From-To based move encoding
// | promotion | capture | special 1 | special 0 | from | to |
// https://www.chessprogramming.org/Encoding_Moves
type Move = u32;

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

const WKING_BIT_BOARD: BitBoard = 1 << 4;
const WQUEEN_BIT_BOARD: BitBoard = 1 << 3;
const WROOK_BIT_BOARD: BitBoard = 1 << 0 | 1 << 7;
const WBISHOP_BIT_BOARD: BitBoard = 1 << 2 | 1 << 5;
const WKNIGHT_BIT_BOARD: BitBoard = 1 << 1 | 1 << 6;
const WPAWN_BIT_BOARD: BitBoard =
    1 << 8 | 1 << 9 | 1 << 10 | 1 << 11 | 1 << 12 | 1 << 13 | 1 << 14 | 1 << 15;

const BKING_BIT_BOARD: BitBoard = 1 << 60;
const BQUEEN_BIT_BOARD: BitBoard = 1 << 59;
const BROOK_BIT_BOARD: BitBoard = 1 << 56 | 1 << 63;
const BBISHOP_BIT_BOARD: BitBoard = 1 << 58 | 1 << 61;
const BKNIGHT_BIT_BOARD: BitBoard = 1 << 57 | 1 << 62;
const BPAWN_BIT_BOARD: BitBoard =
    1 << 48 | 1 << 49 | 1 << 50 | 1 << 51 | 1 << 52 | 1 << 53 | 1 << 54 | 1 << 55;

enum EnumSquare {
    A1,
    B1,
    C1,
    D1,
    E1,
    F1,
    G1,
    H1,
    A2,
    B2,
    C2,
    D2,
    E2,
    F2,
    G2,
    H2,
    A3,
    B3,
    C3,
    D3,
    E3,
    F3,
    G3,
    H3,
    A4,
    B4,
    C4,
    D4,
    E4,
    F4,
    G4,
    H4,
    A5,
    B5,
    C5,
    D5,
    E5,
    F5,
    G5,
    H5,
    A6,
    B6,
    C6,
    D6,
    E6,
    F6,
    G6,
    H6,
    A7,
    B7,
    C7,
    D7,
    E7,
    F7,
    G7,
    H7,
    A8,
    B8,
    C8,
    D8,
    E8,
    F8,
    G8,
    H8,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Colour {
    White,
    Black,
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

#[derive(Copy, Clone)]
pub struct Board {
    piece_bb: [[BitBoard; 6]; 2],
    piece_to_move: Colour,
}

const COLOURS_T: [Colour; 2] = [Colour::White, Colour::Black];
const PIECES_T: [Piece; 6] = [
    Piece::King,
    Piece::Queen,
    Piece::Rook,
    Piece::Bishop,
    Piece::Knight,
    Piece::Pawn,
];

impl Piece {
    pub fn get_piece(&self, colour: Colour) -> String {
        match colour {
            Colour::White => format!("{}", self).to_uppercase(),
            Colour::Black => format!("{}", self),
        }
    }
}

impl Board {
    fn get_piece(&self, idx: u8) -> String {
        for (colour, colour_t) in self.piece_bb.iter().zip(COLOURS_T) {
            for (bitboard, piece_t) in colour.iter().zip(PIECES_T) {
                if (bitboard >> idx) & 1 == 1 {
                    return piece_t.get_piece(colour_t);
                }
            }
        }
        return String::from(".");
    }

    pub fn print_bitboard(&self) {
        let mut out: u64 = 0;
        for colour in self.piece_bb {
            for bitboard in colour {
                out |= bitboard;
            }
        }
        println!("{}", out);
        for rank in (0..8).rev() {
            for file in 0..8 {
                print!("|{}", (out >> (8 * rank + file)) & 1);
            }
            println!("|");
        }
    }

    pub fn print_board(&self) {
        for rank in (0..8).rev() {
            for file in 0..8 {
                print!("|{}", self.get_piece(8 * rank + file));
            }
            println!("|");
        }
    }

    pub fn generate_pseudo_legal(&self) -> Vec<Move> {
        let pieces_bb = match self.piece_to_move {
            Colour::White => self.piece_bb[0],
            Colour::Black => self.piece_bb[1],
        };

        let mut moves: Vec<Move> = Vec::new();

        for (&piece_bb, piece_t) in pieces_bb.iter().zip(PIECES_T) {
            println!("{:?}", serialize_bb(piece_bb));
            for piece_bb in serialize_bb(piece_bb) {
                // TODO: Add additional information such as enpassant and castling
                let mut pos_moves = match piece_t {
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

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Piece::King => write!(f, "k"),
            Piece::Queen => write!(f, "q"),
            Piece::Rook => write!(f, "r"),
            Piece::Bishop => write!(f, "b"),
            Piece::Knight => write!(f, "n"),
            Piece::Pawn => write!(f, "p"),
        }
    }
}

pub fn gen_king_moves(idx: u8) -> Vec<Move> {
    vec![1, 2]
}

pub fn gen_queen_moves(idx: u8) -> Vec<Move> {
    vec![1, 2]
}

pub fn gen_rook_moves(idx: u8) -> Vec<Move> {
    vec![1, 2]
}

pub fn gen_bishop_moves(idx: u8) -> Vec<Move> {
    vec![1, 2]
}

pub fn gen_knight_moves(idx: u8) -> Vec<Move> {
    vec![1, 2]
}

pub fn gen_pawn_moves(idx: u8) -> Vec<Move> {
    vec![1, 2]
}

pub fn init_chess() -> Board {
    Board {
        piece_bb: [
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
        piece_to_move: Colour::White,
    }
}

pub fn serialize_bb(bb: BitBoard) -> Vec<u8> {
    let mut bb = bb as i64;
    let mut set: Vec<u8> = Vec::new();
    while bb != 0 {
        let lsb = (bb & -bb) as BitBoard;
        set.push(lsb.trailing_zeros() as u8);
        bb &= bb - 1;
    }
    return set;
}

pub fn deserialize_bb(set: Vec<u8>) -> BitBoard {
    set.iter().fold(0, |acc, bb| acc ^ ((1 as u64) << bb))
}
