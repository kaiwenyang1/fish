use crate::aliases::Bitboard;

pub fn print_bb(bb: Bitboard) {
    println!("\n  {:#018x}\n", bb);
    for rank in (0u8..8).rev() {
        print!("{}  ", rank + 1);
        for file in 0u8..8 {
            let sq: u8 = 8 * rank + file;
            let bit: u64 = bb & (1u64 << sq);
            print!("{}", if bit != 0 { "X" } else { "." });
            print!("{}", if file <= 6 { " " } else { "\n" });
        }
    }
    println!("\n   A B C D E F G H\n")
}
