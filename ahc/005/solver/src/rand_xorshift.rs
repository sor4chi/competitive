use core::fmt;
use core::num::Wrapping as w;
use rand_core::{impls, le, Error, RngCore, SeedableRng};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct XorShiftRng {
    x: w<u32>,
    y: w<u32>,
    z: w<u32>,
    w: w<u32>,
}

impl fmt::Debug for XorShiftRng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XorShiftRng {{}}")
    }
}

impl RngCore for XorShiftRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let x = self.x;
        let t = x ^ (x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let w_ = self.w;
        self.w = w_ ^ (w_ >> 19) ^ (t ^ (t >> 8));
        self.w.0
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for XorShiftRng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u32 = [0u32; 4];
        le::read_u32_into(&seed, &mut seed_u32);

        if seed_u32.iter().all(|&x| x == 0) {
            seed_u32 = [0xBAD_5EED, 0xBAD_5EED, 0xBAD_5EED, 0xBAD_5EED];
        }

        XorShiftRng {
            x: w(seed_u32[0]),
            y: w(seed_u32[1]),
            z: w(seed_u32[2]),
            w: w(seed_u32[3]),
        }
    }

    fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
        let mut b = [0u8; 16];
        loop {
            rng.try_fill_bytes(&mut b[..])?;
            if !b.iter().all(|&x| x == 0) {
                break;
            }
        }

        Ok(XorShiftRng {
            x: w(u32::from_le_bytes([b[0], b[1], b[2], b[3]])),
            y: w(u32::from_le_bytes([b[4], b[5], b[6], b[7]])),
            z: w(u32::from_le_bytes([b[8], b[9], b[10], b[11]])),
            w: w(u32::from_le_bytes([b[12], b[13], b[14], b[15]])),
        })
    }
}
