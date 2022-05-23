#![allow(dead_code)]

type Bitboard = u64;
type Square = u8;

enum SLit {
  A1, B1, C1, D1, E1, F1, G1, H1,
  A2, B2, C2, D2, E2, F2, G2, H2,
  A3, B3, C3, D3, E3, F3, G3, H3,
  A4, B4, C4, D4, E4, F4, G4, H4,
  A5, B5, C5, D5, E5, F5, G5, H5,
  A6, B6, C6, D6, E6, F6, G6, H6,
  A7, B7, C7, D7, E7, F7, G7, H7,
  A8, B8, C8, D8, E8, F8, G8, H8
}

enum RLit {
  Rank1, Rank2, Rank3, Rank4,
  Rank5, Rank6, Rank7, Rank8
}

enum FLit {
  FileA, FileB, FileC, FileD,
  FileE, FileF, FileG, FileH
}

const RANK_MASK: [u64; 8] = [
  0x00000000000000ff, 0x000000000000ff00, 0x0000000000ff0000, 0x00000000ff000000,
  0x000000ff00000000, 0x0000ff0000000000, 0x00ff000000000000, 0xff00000000000000,
];

const FILE_MASK: [u64; 8] = [
  0x0101010101010101, 0x0202020202020202, 0x0404040404040404, 0x0808080808080808,
  0x1010101010101010, 0x2020202020202020, 0x4040404040404040, 0x8080808080808080
];

const ROOK_REL: [u64; 64] = [
  0x000101010101017e, 0x000202020202027c, 0x000404040404047a, 0x0008080808080876, 
  0x001010101010106e, 0x002020202020205e, 0x004040404040403e, 0x008080808080807e, 
  0x0001010101017e00, 0x0002020202027c00, 0x0004040404047a00, 0x0008080808087600, 
  0x0010101010106e00, 0x0020202020205e00, 0x0040404040403e00, 0x0080808080807e00, 
  0x00010101017e0100, 0x00020202027c0200, 0x00040404047a0400, 0x0008080808760800, 
  0x00101010106e1000, 0x00202020205e2000, 0x00404040403e4000, 0x00808080807e8000, 
  0x000101017e010100, 0x000202027c020200, 0x000404047a040400, 0x0008080876080800, 
  0x001010106e101000, 0x002020205e202000, 0x004040403e404000, 0x008080807e808000, 
  0x0001017e01010100, 0x0002027c02020200, 0x0004047a04040400, 0x0008087608080800, 
  0x0010106e10101000, 0x0020205e20202000, 0x0040403e40404000, 0x0080807e80808000, 
  0x00017e0101010100, 0x00027c0202020200, 0x00047a0404040400, 0x0008760808080800, 
  0x00106e1010101000, 0x00205e2020202000, 0x00403e4040404000, 0x00807e8080808000, 
  0x007e010101010100, 0x007c020202020200, 0x007a040404040400, 0x0076080808080800, 
  0x006e101010101000, 0x005e202020202000, 0x003e404040404000, 0x007e808080808000, 
  0x7e01010101010100, 0x7c02020202020200, 0x7a04040404040400, 0x7608080808080800, 
  0x6e10101010101000, 0x5e20202020202000, 0x3e40404040404000, 0x7e80808080808000, 
];

fn sq_to_bb(sq: Square) -> Bitboard {
  return 1u64 << sq;
}

fn print_bb(bb: Bitboard) {
  println!("\n  {:#018x}\n", bb);
  for rank in (0u8..8).rev() {
    print!("{}  ", rank + 1);
    for file in 0u8..8 {
      let sq: u8 = 8 * rank + file;
      let bit: u64 = bb & (1u64 << sq);
      print!("{}", if bit != 0 { "X" } else { "." });
      print!("{}", if file <= 6 {" "} else { "\n" });
    }
  }
  println!("\n   A B C D E F G H\n")
}

fn main() {
  for rank in 0u8..8 {
    for file in 0u8..8 {
      let mut bb: Bitboard = 0;
      bb |= RANK_MASK[rank as usize];
      bb |= FILE_MASK[file as usize];
      bb ^= sq_to_bb(8*rank + file);
      if rank != RLit::Rank1 as u8 {
        bb &= !RANK_MASK[RLit::Rank1 as usize];
      }
      if rank != RLit::Rank8 as u8 {
        bb &= !RANK_MASK[RLit::Rank8 as usize];
      }
      if file != FLit::FileA as u8 {
        bb &= !FILE_MASK[FLit::FileA as usize];
      }
      if file != FLit::FileH as u8 {
        bb &= !FILE_MASK[FLit::FileH as usize];
      }
      print_bb(bb);
      // print!("{:#018x}, ", bb);
      // if file == 3 || file == 7 {
      //   println!("");
      // }
    }
  }
}
