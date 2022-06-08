use crate::aliases::{Bitboard, Square};
use crate::masks;

use std::time::UNIX_EPOCH;

fn batk(sq: Square, b: Bitboard) -> Bitboard {
    let mut ret: u64 = 0;
    let (rk, fl) = (sq / 8, sq % 8);
    for (r, f) in (0..rk).rev().zip((0..fl).rev()) {
        let sq = 8 * r + f;
        ret |= 1u64 << sq;
        if b & (1 << sq) != 0 {
            break;
        }
    }
    for (r, f) in (0..rk).rev().zip(fl + 1..8) {
        let sq = 8 * r + f;
        ret |= 1u64 << sq;
        if b & (1 << sq) != 0 {
            break;
        }
    }
    for (r, f) in (rk + 1..8).zip((0..fl).rev()) {
        let sq = 8 * r + f;
        ret |= 1u64 << sq;
        if b & (1 << sq) != 0 {
            break;
        }
    }
    for (r, f) in (rk + 1..8).zip(fl + 1..8) {
        let sq = 8 * r + f;
        ret |= 1u64 << sq;
        if b & (1 << sq) != 0 {
            break;
        }
    }
    ret
}

fn ratk(sq: Square, b: Bitboard) -> Bitboard {
    let mut ret: u64 = 0;
    let (rk, fl) = (sq / 8, sq % 8);
    for r in (0..rk).rev() {
        let sq = 8 * r + fl;
        ret |= 1u64 << sq;
        if b & (1 << sq) != 0 {
            break;
        }
    }
    for r in rk + 1..8 {
        let sq = 8 * r + fl;
        ret |= 1u64 << sq;
        if b & (1 << sq) != 0 {
            break;
        }
    }
    for f in (0..fl).rev() {
        let sq = 8 * rk + f;
        ret |= 1u64 << sq;
        if b & (1 << sq) != 0 {
            break;
        }
    }
    for f in fl + 1..8 {
        let sq = 8 * rk + f;
        ret |= 1u64 << sq;
        if b & (1 << sq) != 0 {
            break;
        }
    }
    ret
}

// xorshift* Prng
// https://en.wikipedia.org/wiki/Xorshift
struct Prng {
    state: u64,
}

impl Prng {
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
pub struct Magic {
    pub num: u64,
    pub shift: u8,
}

impl Magic {
    fn transform(&self, b: Bitboard) -> u64 {
        u64::wrapping_mul(b, self.num) >> self.shift
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
fn check_mag(
    ms: &masks::Lookup,
    mag: &Magic,
    vec: &mut [Bitboard],
    sq: Square,
    bishop: bool,
) -> bool {
    let rel_mask = if bishop {
        ms.brel[sq as usize]
    } else {
        ms.rrel[sq as usize]
    };
    let mut rel_bits: Bitboard = 0;

    // Stack of modified positions in arr, so we can roll back easily
    let mut stack: Vec<usize> = Vec::new();

    // See: https://www.chessprogramming.org/Traversing_Subsets_of_a_Set
    // We will traverse over all possible sets of the relevance mask, checking
    // that all collisions occur only when the attack sets are the same
    loop {
        let att = if bishop {
            batk(sq, rel_bits)
        } else {
            ratk(sq, rel_bits)
        };
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
        if rel_bits == 0 {
            break;
        }
    }
    true
}

// find_mag finds a magic with the shortest possible width in the given duration,
// along with the corresponding mapping. It is guaranteed to find a magic for the
// initial width, and might exceed the given duration to do so.
fn find_mag(
    ms: &masks::Lookup,
    sq: Square,
    d: std::time::Duration,
    bishop: bool,
) -> (Magic, Vec<Bitboard>) {
    // Seed the PRNG
    let mut rng = Prng {
        state: std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Unexpected SystemTime error")
            .as_millis() as u64,
    };

    let start_time = std::time::SystemTime::now();

    // Keep track of the best result so far
    let mut ret_mag: Option<Magic> = None;
    let mut ret_vec: Vec<Bitboard> = Vec::new();

    // We will start from the initial width, and lower as we go
    let initial_width = if bishop { 10 } else { 13 };
    'outer: for width in (1..initial_width + 1).rev() {
        let mut vec = vec![0; 1 << width];
        loop {
            // Try a random magic
            let mag = Magic {
                num: rng.next_sparse(),
                shift: 64 - width,
            };

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
                break;
            }
        }
    }

    match ret_mag {
        Some(mag) => (mag, ret_vec),
        None => panic!("unexpected panic on magic generation"),
    }
}

pub fn find_bmag(ms: &masks::Lookup, sq: Square, d: std::time::Duration) -> (Magic, Vec<Bitboard>) {
    let ret = find_mag(ms, sq, d, true);
    println!(
        "found bishop magic {:#018X} for square {} with width {}",
        ret.0.num,
        sq,
        64 - ret.0.shift
    );
    ret
}

pub fn find_rmag(ms: &masks::Lookup, sq: Square, d: std::time::Duration) -> (Magic, Vec<Bitboard>) {
    let ret = find_mag(ms, sq, d, false);
    println!(
        "found rook magic {:#018X} for square {} with width {}",
        ret.0.num,
        sq,
        64 - ret.0.shift
    );
    ret
}
