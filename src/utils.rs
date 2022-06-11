use crate::aliases::Bitboard;

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
