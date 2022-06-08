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

    ms
}
