//! Tiny deterministic PRNG — a linear congruential generator.
//!
//! We avoid pulling in `rand` (and `rand_chacha`) to keep the WASM bundle
//! small. Determinism matters more than statistical quality for prototype
//! map generation: the same seed must always yield the same map. The
//! constants are the well-known Numerical Recipes LCG parameters, which are
//! fine for our "threshold per cell" use.

/// A linear congruential generator over `u64` state.
#[derive(Debug)]
pub struct Lcg {
    state: u64,
}

impl Lcg {
    /// Create a new generator. State is seeded directly; we mix the seed
    /// once so that nearby seeds (e.g. 1 and 2) don't share an initial
    /// output prefix.
    pub fn new(seed: u64) -> Self {
        Self {
            state: splitmix(seed),
        }
    }

    /// Advance the generator one step and return the next `u32`.
    pub fn next_u32(&mut self) -> u32 {
        // Numerical Recipes LCG constants for 64-bit state.
        const A: u64 = 6364136223846793005;
        const C: u64 = 1442695040888963407;
        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        // Use the upper 32 bits — the high bits of an LCG have better
        // distribution than the low ones.
        (self.state >> 32) as u32
    }
}

/// SplitMix64-style mixer used to initialize LCG state from a raw seed.
/// This is a deterministic, dependency-free way to spread seed entropy.
fn splitmix(mut z: u64) -> u64 {
    const SPLIT: u64 = 0x9E3779B97F4A7C15;
    z = z.wrapping_add(SPLIT);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reproducible_sequence() {
        let mut a = Lcg::new(123);
        let mut b = Lcg::new(123);
        for _ in 0..100 {
            assert_eq!(a.next_u32(), b.next_u32());
        }
    }

    #[test]
    fn different_seeds_differ() {
        let mut a = Lcg::new(1);
        let mut b = Lcg::new(2);
        let mut any_diff = false;
        for _ in 0..10 {
            if a.next_u32() != b.next_u32() {
                any_diff = true;
            }
        }
        assert!(any_diff);
    }

    #[test]
    fn output_in_u32_range() {
        let mut rng = Lcg::new(42);
        for _ in 0..1000 {
            let v = rng.next_u32();
            assert!(v <= u32::MAX);
        }
    }
}
