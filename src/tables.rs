use crate::aliases::Bitboard;
use crate::{magic, masks};

pub struct Lookup {
    pub bmag: [magic::Magic; 64],
    pub bmag_tbl: [Vec<Bitboard>; 64],
    pub rmag: [magic::Magic; 64],
    pub rmag_tbl: [Vec<Bitboard>; 64],
}

impl Lookup {
    pub fn new(ms: &masks::Lookup) -> Lookup {
        // Duration allocated to generate each magic
        let duration = std::time::Duration::from_millis(0);
        let mut ret: Lookup = Lookup {
            bmag: [Default::default(); 64],
            bmag_tbl: [(); 64].map(|_| Default::default()),
            rmag: [Default::default(); 64],
            rmag_tbl: [(); 64].map(|_| Default::default()),
        };
        for sq in 0..64 {
            let (mag, tbl) = magic::find_bmag(ms, sq, duration);
            ret.bmag[sq as usize] = mag;
            ret.bmag_tbl[sq as usize] = tbl;
        }
        for sq in 0..64 {
            let (mag, tbl) = magic::find_rmag(ms, sq, duration);
            ret.rmag[sq as usize] = mag;
            ret.rmag_tbl[sq as usize] = tbl;
        }
        ret
    }
}
