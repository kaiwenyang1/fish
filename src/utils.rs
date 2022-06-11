use crate::aliases::{Bitboard, Move, Square};
use crate::{enums, positions};

pub fn square_string(sq: Square) -> String {
    if sq == enums::Square::Null as Square {
        return String::from("-");
    }
    let (rk, fl) = (sq / 8, sq % 8);
    format!("{}{}", (b'a' + fl) as char, (b'1' + rk) as char)
}

pub fn move_string(mv: Move) -> String {
    format!(
        "{}{}",
        square_string(positions::move_get_from(mv)),
        square_string(positions::move_get_to(mv))
    )
}

pub fn bb_string(bb: Bitboard) -> String {
    let mut ret = String::new();
    ret.push_str(&*format!("{:#018x}\n", bb));
    for rank in (0u8..8).rev() {
        ret.push_str(&*format!("{} ", rank + 1));
        for file in 0u8..8 {
            let sq: u8 = 8 * rank + file;
            let bit: u64 = bb & (1u64 << sq);
            ret.push_str(if bit != 0 { "X" } else { "." });
            ret.push_str(if file <= 6 { " " } else { "\n" });
        }
    }
    ret.push_str("  A B C D E F G H");
    ret
}

pub fn string_square(s: &str) -> Square {
    if s == "-" {
        enums::Square::Null as Square
    } else {
        let fl = (s.chars().next().expect("bad string for square provided") as u8) - b'a';
        let rk = (s.chars().nth(1).expect("bad string for square provided") as u8) - b'1';
        8 * rk + fl
    }
}

pub fn ascii_colour_piece(c: char) -> Option<(enums::Colour, enums::Piece)> {
    let colour = if c.is_uppercase() {
        enums::Colour::White
    } else {
        enums::Colour::Black
    };
    match c.to_ascii_uppercase() {
        'K' => Some((colour, enums::Piece::King)),
        'Q' => Some((colour, enums::Piece::Queen)),
        'R' => Some((colour, enums::Piece::Rook)),
        'B' => Some((colour, enums::Piece::Bishop)),
        'N' => Some((colour, enums::Piece::Knight)),
        'P' => Some((colour, enums::Piece::Pawn)),
        _ => None,
    }
}
