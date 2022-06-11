use crate::aliases::{Bitboard, Move, Square};
use crate::{enums, masks, tables, utils};

// Using From-To based move encoding
//
//      0    |   0   |    0    |    0    | 000000 | 000000
//  ---------|-------|---------|---------|--------|--------
//  promotion|capture|special 1|special 0|  from  |   to
//
// https://www.chessprogramming.org/Encoding_Moves

pub fn move_get_to(mov: Move) -> u8 {
    (mov & 0x3f) as u8
}

pub fn move_get_from(mov: Move) -> u8 {
    ((mov >> 6) & 0x3f) as u8
}

pub fn move_get_code(mov: Move) -> u8 {
    ((mov >> 12) & 0xf) as u8
}

fn make_move(from: Square, to: Square, special: u8) -> Move {
    (to as Move) | ((from as Move) << 6) | ((special as Move) << 12)
}

const FLAG_QUIET_MOVE: u8 = 0;
const FLAG_DOUBLE_PAWN_PUSH: u8 = 1;
const FLAG_KING_CASTLE: u8 = 2;
const FLAG_QUEEN_CASTLE: u8 = 3;
const FLAG_CAPTURE: u8 = 4;
const FLAG_EP_CAPTURE: u8 = 5;
const FLAG_PROMOTE_KNIGHT: u8 = 8;
const FLAG_PROMOTE_BISHOP: u8 = 9;
const FLAG_PROMOTE_ROOK: u8 = 10;
const FLAG_PROMOTE_QUEEN: u8 = 11;
const FLAG_CAPTURE_PROMOTE_KNIGHT: u8 = 12;
const FLAG_CAPTURE_PROMOTE_BISHOP: u8 = 13;
const FLAG_CAPTURE_PROMOTE_ROOK: u8 = 14;
const FLAG_CAPTURE_PROMOTE_QUEEN: u8 = 15;

const WKING_CASTLE_RIGHTS: u8 = 1 << 0;
const WQUEEN_CASTLE_RIGHTS: u8 = 1 << 1;
const BKING_CASTLE_RIGHTS: u8 = 1 << 2;
const BQUEEN_CASTLE_RIGHTS: u8 = 1 << 3;

#[derive(Copy, Clone)]
pub struct Position {
    bitboards: [[Bitboard; 6]; 2],
    side_bitboards: [Bitboard; 2],
    all_bitboard: Bitboard,
    side: enums::Colour,
    ep_target: Square,

    // castling rights: qkQK
    castling: u8,
}

impl Position {
    fn square_repr(&self, sq: Square) -> char {
        for colour in enums::Colour::values() {
            for piece in enums::Piece::values() {
                let bitboard = self.bitboards[colour as usize][piece as usize];
                if (bitboard >> sq) & 1 == 1 {
                    match "♘♗♖♕♙♔♞♝♜♛♟♚"
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

    pub fn string(&self) -> String {
        let side_string = match self.side {
            enums::Colour::White => "w",
            enums::Colour::Black => "b",
        };

        let mut castling_string = String::new();
        for (i, c) in "KQkq".chars().enumerate() {
            if (self.castling >> i) & 1 != 0 {
                castling_string.push(c);
            }
        }
        if castling_string.is_empty() {
            castling_string = String::from("-");
        }

        let mut ret = format!(
            "{} cs:{} ep:{}\n",
            side_string,
            castling_string,
            utils::square_string(self.ep_target)
        );
        for rank in (0..8).rev() {
            ret.push_str(&*format!("{} ", rank + 1));
            for file in 0..8 {
                ret.push_str(&*format!("{}", self.square_repr(8 * rank + file)));
                if file != 7 {
                    ret.push(' ');
                }
            }
            ret.push('\n');
        }
        ret.push_str("  A B C D E F G H");
        ret
    }

    pub fn generate_pseudo_legal(&self, m: &masks::Lookup, t: &tables::Lookup) -> Vec<Move> {
        let pieces_bb = match self.side {
            enums::Colour::White => self.bitboards[0],
            enums::Colour::Black => self.bitboards[1],
        };

        let mut moves: Vec<Move> = Vec::new();

        for piece in enums::Piece::values() {
            let piece_bb = pieces_bb[piece as usize];
            for sq in bb_squares(piece_bb) {
                let mut pos_moves = match piece {
                    enums::Piece::King => self.gen_king_moves(sq, m),
                    enums::Piece::Queen => self.gen_queen_moves(sq, m, t),
                    enums::Piece::Rook => self.gen_rook_moves(sq, m, t),
                    enums::Piece::Bishop => self.gen_bishop_moves(sq, m, t),
                    enums::Piece::Knight => self.gen_knight_moves(sq, m),
                    enums::Piece::Pawn => self.gen_pawn_moves(sq, m),
                };
                moves.append(&mut pos_moves);
            }
        }
        moves
    }

    fn gen_from_atk(&self, from: Square, atk: Bitboard) -> Vec<Move> {
        bb_squares(atk & !self.side_bitboards[self.side as usize])
            .iter()
            .map(|&to| make_move(from, to, 0))
            .collect()
    }

    pub fn gen_king_moves(&self, from: Square, m: &masks::Lookup) -> Vec<Move> {
        let mut ret = self.gen_from_atk(from, m.king[from as usize]);
        match self.side {
            enums::Colour::White => {
                if self.castling & WKING_CASTLE_RIGHTS != 0
                    && self.side_bitboards[self.side as usize] & 0x0000000000000060 == 0
                {
                    ret.push(make_move(
                        enums::Square::E1 as Square,
                        enums::Square::G1 as Square,
                        FLAG_KING_CASTLE,
                    ));
                }
                if self.castling & WQUEEN_CASTLE_RIGHTS != 0
                    && self.side_bitboards[self.side as usize] & 0x000000000000000e == 0
                {
                    ret.push(make_move(
                        enums::Square::E1 as Square,
                        enums::Square::C1 as Square,
                        FLAG_QUEEN_CASTLE,
                    ));
                }
            }
            enums::Colour::Black => {
                if self.castling & BKING_CASTLE_RIGHTS != 0
                    && self.side_bitboards[self.side as usize] & 0x6000000000000000 == 0
                {
                    ret.push(make_move(
                        enums::Square::E8 as Square,
                        enums::Square::G8 as Square,
                        FLAG_KING_CASTLE,
                    ));
                }
                if self.castling & BQUEEN_CASTLE_RIGHTS != 0
                    && self.side_bitboards[self.side as usize] & 0x0e00000000000000 == 0
                {
                    ret.push(make_move(
                        enums::Square::E8 as Square,
                        enums::Square::C8 as Square,
                        FLAG_QUEEN_CASTLE,
                    ));
                }
            }
        }
        ret
    }

    pub fn gen_queen_moves(
        &self,
        from: Square,
        m: &masks::Lookup,
        t: &tables::Lookup,
    ) -> Vec<Move> {
        [
            self.gen_rook_moves(from, m, t),
            self.gen_bishop_moves(from, m, t),
        ]
        .concat()
    }

    pub fn gen_rook_moves(&self, from: Square, m: &masks::Lookup, t: &tables::Lookup) -> Vec<Move> {
        let hash = t.rmag[from as usize].transform(self.all_bitboard & m.rrel[from as usize]);
        self.gen_from_atk(from, t.rmag_tbl[from as usize][hash as usize])
    }

    pub fn gen_bishop_moves(
        &self,
        from: Square,
        m: &masks::Lookup,
        t: &tables::Lookup,
    ) -> Vec<Move> {
        let hash = t.bmag[from as usize].transform(self.all_bitboard & m.brel[from as usize]);
        self.gen_from_atk(from, t.bmag_tbl[from as usize][hash as usize])
    }

    pub fn gen_knight_moves(&self, from: Square, m: &masks::Lookup) -> Vec<Move> {
        self.gen_from_atk(from, m.knight[from as usize])
    }

    pub fn gen_pawn_moves(&self, from: Square, m: &masks::Lookup) -> Vec<Move> {
        let rk = from / 8;
        let mut ret: Vec<Move> = Vec::new();

        // move forward two squares
        // note 0x101 masks A1 and B1, we can shift this accordingly to describe the
        // two squares in front of the pawn
        match self.side {
            enums::Colour::White => {
                if (rk == 1) && (self.all_bitboard & (0x101 << (from + 8)) == 0) {
                    ret.push(make_move(from, from + 16, FLAG_DOUBLE_PAWN_PUSH));
                }
            }
            enums::Colour::Black => {
                if (rk == 6) && (self.all_bitboard & (0x101 << (from - 24)) == 0) {
                    ret.push(make_move(from, from - 16, FLAG_DOUBLE_PAWN_PUSH));
                }
            }
        }

        // move one square, this (reasonably) assumes the pawn can move forward
        // if the square in front of it is unoccupied
        match self.side {
            enums::Colour::White => {
                if self.all_bitboard & (0x1 << (from + 8)) == 0 {
                    ret.push(make_move(from, from + 8, FLAG_QUIET_MOVE));
                }
            }
            enums::Colour::Black => {
                if self.all_bitboard & (0x1 << (from - 8)) == 0 {
                    ret.push(make_move(from, from - 8, FLAG_QUIET_MOVE));
                }
            }
        }

        // do a capture
        let targets = bb_squares(m.pcapture[self.side as usize][from as usize]);
        for to in targets {
            if to == self.ep_target {
                ret.push(make_move(from, to, FLAG_EP_CAPTURE));
            } else if (1 << to) & self.side_bitboards[self.side as usize ^ 1] != 0 {
                ret.push(make_move(from, to, FLAG_CAPTURE));
            }
        }

        // potentially, moves one step forward are promotions. go through the list of
        // all moves generated and turn each move into 4 moves, corresponding to 4
        // different promotions.
        let promotions = [
            make_move(0, 0, FLAG_PROMOTE_KNIGHT),
            make_move(0, 0, FLAG_PROMOTE_BISHOP),
            make_move(0, 0, FLAG_PROMOTE_ROOK),
            make_move(0, 0, FLAG_PROMOTE_QUEEN),
        ];
        match self.side {
            enums::Colour::White => {
                if rk == 6 {
                    ret = ret
                        .iter()
                        .flat_map(|mv| promotions.iter().map(|p| mv | p).collect::<Vec<_>>())
                        .collect()
                }
            }
            enums::Colour::Black => {
                if rk == 1 {
                    ret = ret
                        .iter()
                        .flat_map(|mv| promotions.iter().map(|p| mv | p).collect::<Vec<_>>())
                        .collect()
                }
            }
        }

        ret
    }
}

pub fn make_position(fen: &str) -> Position {
    let mut ret = Position {
        bitboards: [[0u64; 6]; 2],
        side_bitboards: [0u64; 2],
        all_bitboard: 0,
        side: enums::Colour::White,
        ep_target: enums::Square::Null as Square,
        castling: 0,
    };
    let mut tokens = fen.split(' ');

    let board_token = tokens
        .next()
        .expect("fen piece placement data not provided");
    for (rk, line) in (0..8).zip(board_token.split('/').rev()) {
        let mut fl = 0;
        for c in line.chars() {
            let cp = utils::ascii_colour_piece(c);
            match cp {
                Some((colour, piece)) => {
                    let sq = 8 * rk + fl;
                    ret.bitboards[colour as usize][piece as usize] |= 1 << sq;
                    fl += 1;
                }
                None => {
                    fl += c
                        .to_digit(10)
                        .expect("expected number to describe empty squares");
                }
            }
        }
    }

    let side_token = tokens.next().expect("fen active color not provided");
    match side_token {
        "w" => ret.side = enums::Colour::White,
        "b" => ret.side = enums::Colour::Black,
        _ => panic!("bad fen active colour provided, expected 'w' or 'b'"),
    }

    let castling_token = tokens.next().expect("fen castling rights not provided");
    for (i, c) in "KQkq".chars().enumerate() {
        if castling_token.contains(c) {
            ret.castling |= 1 << i;
        }
    }

    let ep_target_token = tokens.next().expect("fen en passant target not provided");
    ret.ep_target = utils::string_square(ep_target_token);

    for (i, side_bb) in ret.side_bitboards.iter_mut().enumerate() {
        *side_bb = ret.bitboards[i].iter().fold(0, |acc, bb| acc | bb);
    }

    ret.all_bitboard = ret.side_bitboards[0] | ret.side_bitboards[1];

    ret
}

pub fn bb_squares(bb: Bitboard) -> Vec<Square> {
    let mut bb = bb as i64;
    let mut set: Vec<Square> = Vec::new();
    while bb != 0 {
        let lsb = (bb & 0i64.wrapping_sub(bb)) as Bitboard;
        set.push(lsb.trailing_zeros() as Square);
        bb &= bb.wrapping_sub(1);
    }
    set
}

pub fn make_bb(set: Vec<u8>) -> Bitboard {
    set.iter().fold(0, |acc, bb| acc ^ (1u64 << bb))
}
