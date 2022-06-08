use crate::aliases::Bitboard;

pub struct Lookup {
    // Mask a square
    pub sq: [Bitboard; 64],

    // Mask ranks/files
    pub rank: [Bitboard; 8],
    pub file: [Bitboard; 8],

    // Mask diagonals/anti-diagonals
    // Diagonal of a square is (7 + rank - file)
    // Antidiagonal of a square is (rank + file)
    pub diag: [Bitboard; 15],
    pub adiag: [Bitboard; 15],

    // Relevance masks for each square
    // See: https://www.chessprogramming.org/Magic_Bitboards
    pub brel: [Bitboard; 64],
    pub rrel: [Bitboard; 64],

    // Masks for king/knight
    pub king: [Bitboard; 64],
    pub knight: [Bitboard; 64],
}

pub fn make_mask_lookup() -> Lookup {
    let mut ms = Lookup {
        sq: [0u64; 64],
        rank: [0u64; 8],
        file: [0u64; 8],
        diag: [0u64; 15],
        adiag: [0u64; 15],
        brel: [0u64; 64],
        rrel: [0u64; 64],
        king: [0u64; 64],
        knight: [0u64; 64],
    };

    for sq in 0..64 {
        ms.sq[sq] = 1u64 << sq;
    }

    for rk in 0..8 {
        for fl in 0..8 {
            let sq = 8 * rk + fl;
            ms.rank[rk] |= ms.sq[sq];
        }
    }

    for fl in 0..8 {
        for rk in 0..8 {
            let sq = 8 * rk + fl;
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
        if rk != 0 {
            ms.rrel[sq] &= !ms.rank[0];
        }
        if rk != 7 {
            ms.rrel[sq] &= !ms.rank[7];
        }
        if fl != 0 {
            ms.rrel[sq] &= !ms.file[0];
        }
        if fl != 7 {
            ms.rrel[sq] &= !ms.file[7];
        }
    }

    for sq in 0..64 {
        let (rk, fl): (i8, i8) = ((sq / 8) as i8, (sq % 8) as i8);
        for r in 0.max(rk - 1)..8.min(rk + 2) {
            for f in 0.max(fl as i8 - 1)..8.min(fl + 2) {
                if r != rk || f != fl {
                    ms.king[sq] |= ms.sq[(8 * r + f) as usize];
                }
            }
        }
    }

    for sq in 0..64 {
        let (rk, fl): (i8, i8) = ((sq / 8) as i8, (sq % 8) as i8);
        for r in 0.max(rk - 2)..8.min(rk + 3) {
            for f in 0.max(fl - 2)..8.min(fl + 3) {
                if (rk - r).abs() + (fl - f).abs() == 3 {
                    ms.knight[sq] |= ms.sq[(8 * r + f) as usize];
                }
            }
        }
    }

    ms
}
