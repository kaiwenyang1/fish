#![allow(dead_code)]

use std::time::UNIX_EPOCH;

type Square = u8;
type Bitboard = u64;

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

struct MaskSet {
  // Mask a square
  sq: [Bitboard; 64],

  // Mask ranks/files
  rank: [Bitboard; 8],
  file: [Bitboard; 8],

  // Mask diagonals/anti-diagonals
  // Diagonal of a square is (7 + rank - file)
  // Antidiagonal of a square is (rank + file)
  diag: [Bitboard; 15],
  adiag: [Bitboard; 15],

  // Relevance masks for each square
  // See: https://www.chessprogramming.org/Magic_Bitboards
  brel: [Bitboard; 64],
  rrel: [Bitboard; 64],
}

fn make_mask_set() -> MaskSet {
  let mut ms = MaskSet {
    sq: [0u64; 64],
    rank: [0u64; 8],
    file: [0u64; 8],
    diag: [0u64; 15],
    adiag: [0u64; 15],
    brel: [0u64; 64],
    rrel: [0u64; 64]
  };

  for sq in 0..64 {
    ms.sq[sq] = 1u64 << sq;
  }

  for rk in 0..8 {
    for fl in 0..8 {
      let sq = 8*rk + fl;
      ms.rank[rk] |= ms.sq[sq];
    }
  }

  for fl in 0..8 {
    for rk in 0..8 {
      let sq = 8*rk + fl;
      ms.file[fl] |= ms.sq[sq];
    }
  }

  for dg in 0..15 {
    for sq in 0..64 {
      let (rk, fl) = (sq / 8, sq % 8);
      if 7 + rk - fl == dg {
        ms.diag[dg] |= ms.sq[sq];
      }
    }
  }

  for adg in 0..15 {
    for sq in 0..64 {
      let (rk, fl) = (sq / 8, sq % 8);
      if rk + fl == adg {
        ms.adiag[adg] |= ms.sq[sq];
      }
    }
  }

  for sq in 0..64 {
    let (rk, fl) = (sq / 8, sq % 8);
    let (dg, adg) = (7 + rk - fl, rk + fl);
    ms.brel[sq] = (ms.diag[dg] | ms.adiag[adg]) ^ ms.sq[sq];
    ms.brel[sq] &= !(ms.rank[0] | ms.rank[7] | ms.file[0] | ms.file[7]);
  }

  for sq in 0..64 {
    let (rk, fl) = (sq / 8, sq % 8);
    ms.rrel[sq] = (ms.rank[rk] | ms.file[fl]) ^ ms.sq[sq];
    if rk != 0 { ms.rrel[sq] &= !ms.rank[0]; }
    if rk != 7 { ms.rrel[sq] &= !ms.rank[7]; }
    if fl != 0 { ms.rrel[sq] &= !ms.file[0]; }
    if fl != 7 { ms.rrel[sq] &= !ms.file[7]; }
  }

  return ms;
}

fn batk(sq: Square, b: Bitboard) -> Bitboard {
  let mut ret: u64 = 0;
  let (rk, fl) = (sq / 8, sq % 8);
  for (r, f) in (0..rk).rev().zip((0..fl).rev()) {
    let sq = 8 * r + f;
    ret |= 1u64 << sq;
    if b & (1 << sq) != 0 { break; }
  }
  for (r, f) in (0..rk).rev().zip(fl+1..8) {
    let sq = 8 * r + f;
    ret |= 1u64 << sq;
    if b & (1 << sq) != 0 { break; }
  }
  for (r, f) in (rk+1..8).zip((0..fl).rev()) {
    let sq = 8 * r + f;
    ret |= 1u64 << sq;
    if b & (1 << sq) != 0 { break; }
  }
  for (r, f) in (rk+1..8).zip(fl+1..8) {
    let sq = 8 * r + f;
    ret |= 1u64 << sq;
    if b & (1 << sq) != 0 { break; }
  }
  ret
}

fn ratk(sq: Square, b: Bitboard) -> Bitboard {
  let mut ret: u64 = 0;
  let (rk, fl) = (sq / 8, sq % 8);
  for r in (0..rk).rev() {
    let sq = 8 * r + fl;
    ret |= 1u64 << sq;
    if b & (1 << sq) != 0 { break; }
  }
  for r in rk+1..8 {
    let sq = 8 * r + fl;
    ret |= 1u64 << sq;
    if b & (1 << sq) != 0 { break; }
  }
  for f in (0..fl).rev() {
    let sq = 8 * rk + f;
    ret |= 1u64 << sq;
    if b & (1 << sq) != 0 { break; }
  }
  for f in fl+1..8 {
    let sq = 8 * rk + f;
    ret |= 1u64 << sq;
    if b & (1 << sq) != 0 { break; }
  }
  ret
}

// xorshift* PRNG
// https://en.wikipedia.org/wiki/Xorshift
struct PRNG {
  state: u64,
}

impl PRNG {
  fn next(&mut self) -> u64 {
    self.state ^= self.state >> 12;
    self.state ^= self.state << 25;
    self.state ^= self.state >> 27;
    self.state = u64::wrapping_mul(self.state, 0x2545F4914F6CDD1Du64);
    self.state
  }

  fn next_sparse(&mut self) -> u64 {
    let (x, y, z) = (self.next(), self.next(), self.next());
    x & y & z
  }
}

#[derive(Default, Copy, Clone)]
struct Magic {
  num: u64,
  shift: u8,
}

impl Magic {
  fn transform(&self, b: Bitboard) -> u64 {
    return u64::wrapping_mul(b, self.num) >> self.shift;
  }
}

// check_mag checks if a given magic is valid.
// - ms:     Initialized MaskSet, relevance masks are needed
// - mag:      The magic to check
// - vec:    Vector of size 2^(64 - mag.shift), initialized to all 0s. If the magic
//           is valid, the hash to attack set mapping generated will be stored in vec
// - sq:     The square to check the magic for
// - bishop: If true, will check if the magic is valid for a bishop on sq.
//           Otherwise it checks if the magic is valid for a rook on sq
fn check_mag(ms: &MaskSet, mag: &Magic, vec: &mut Vec<Bitboard>,
             sq: Square, bishop: bool) -> bool {
  let rel_mask = if bishop { ms.brel[sq as usize] } else { ms.rrel[sq as usize] };
  let mut rel_bits: Bitboard = 0;

  // Stack of modified positions in arr, so we can roll back easily
  let mut stack: Vec<usize> = Vec::new();

  // See: https://www.chessprogramming.org/Traversing_Subsets_of_a_Set
  // We will traverse over all possible sets of the relevance mask, checking
  // that all collisions occur only when the attack sets are the same
  loop {
    let att = if bishop { batk(sq, rel_bits) } else { ratk(sq, rel_bits) };
    let hash = mag.transform(rel_bits) as usize;

    // Undesirable collision
    if vec[hash] != 0 && vec[hash] != att {
      for i in stack {
        vec[i] = 0;
      }
      return false;
    }

    // No collision
    vec[hash] = att;
    stack.push(hash);

    rel_bits = u64::wrapping_sub(rel_bits, rel_mask) & rel_mask;
    if rel_bits == 0 { break; }
  }
  true
}

// find_mag finds a magic with the shortest possible width in the given duration,
// along with the corresponding mapping. It is guaranteed to find a magic for the
// initial width, and might exceed the given duration to do so.
fn find_mag(ms: &MaskSet, sq: Square, d: std::time::Duration,
            bishop: bool) -> (Magic, Vec<Bitboard>) {

  // Seed the PRNG
  let mut rng = PRNG{
    state: std::time::SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Unexpected SystemTime error")
      .as_millis() as u64
  };

  let start_time = std::time::SystemTime::now();

  // Keep track of the best result so far
  let mut ret_mag: Option<Magic> = None;
  let mut ret_vec: Vec<Bitboard> = Vec::new();

  // We will start from the initial width, and lower as we go
  let initial_width = if bishop { 10 } else { 13 };
  'outer: for width in (1..initial_width+1).rev() {
    let mut vec = vec![0; 1 << width];
    loop {
      // Try a random magic
      let mag = Magic{ num: rng.next_sparse(), shift: 64 - width };

      // Deadline exceeded, return immediately
      let dt = std::time::SystemTime::now()
          .duration_since(start_time)
          .expect("Unexpected SystemTime error");

      if dt > d && ret_mag.is_some() {
        break 'outer;
      }

      // This magic works
      if check_mag(ms, &mag, &mut vec, sq, bishop) {
        ret_mag = Some(mag);
        ret_vec = vec;
        break
      }
    }
  }

  match ret_mag {
    Some(mag) => return (mag, ret_vec),
    None => panic!("unexpected panic on magic generation"),
  }
}

fn find_bmag(ms: &MaskSet, sq: Square,
             d: std::time::Duration) -> (Magic, Vec<Bitboard>) {
  let ret = find_mag(ms, sq, d, true);
  println!("found bishop magic {:#018X} for square {} with width {}",
           ret.0.num, sq, 64 - ret.0.shift);
  ret
}

fn find_rmag(ms: &MaskSet, sq: Square,
             d: std::time::Duration) -> (Magic, Vec<Bitboard>) {
  let ret = find_mag(ms, sq, d, false);
  println!("found rook magic {:#018X} for square {} with width {}",
           ret.0.num, sq, 64 - ret.0.shift);
  ret
}

struct TableSet {
  bmag: [Magic; 64],
  bmag_tbl: [Vec<Bitboard>; 64],
  rmag: [Magic; 64],
  rmag_tbl: [Vec<Bitboard>; 64],
}

fn make_table_set(ms: &MaskSet) -> TableSet {
  // Duration allocated to generate each magic
  let duration = std::time::Duration::from_millis(100);
  let mut ret: TableSet = TableSet{
    bmag: [Default::default(); 64],
    bmag_tbl: [(); 64].map(|_| Default::default()),
    rmag: [Default::default(); 64],
    rmag_tbl: [(); 64].map(|_| Default::default()),
  };
  for sq in 0..64 as Square {
    let (mag, tbl) = find_bmag(&ms, sq,duration);
    ret.bmag[sq as usize] = mag;
    ret.bmag_tbl[sq as usize] = tbl;
  }
  for sq in 0..64 {
    let (mag, tbl) = find_rmag(&ms, sq,duration);
    ret.rmag[sq as usize] = mag;
    ret.rmag_tbl[sq as usize] = tbl;
  }
  ret
}

fn main() {
  let ms = make_mask_set();
  make_table_set(&ms);
}
