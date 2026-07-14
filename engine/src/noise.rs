fn generate_perm(seed: i32) -> [u8; 512] {
    let mut p = [0u8; 256];
    for i in 0..256u16 {
        p[i as usize] = i as u8;
    }

    let mut state = seed as u64;
    for i in (1..256).rev() {
        state = state.wrapping_add(0x9E3779B97F4A7C15);
        state = (state ^ (state >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        state = (state ^ (state >> 27)).wrapping_mul(0x94D049BB133111EB);
        state ^= state >> 31;
        // Truncate to u32 before narrowing: `state as usize` truncates to the
        // host's pointer width (32 bits on wasm32, 64 bits natively), so the
        // same seed produced a different permutation table — and therefore a
        // different biome map — depending on target. Forcing u32 here makes
        // the shuffle identical everywhere, matching what the deployed wasm
        // build (32-bit usize) already produces.
        let j = (state as u32 as usize) % (i + 1);
        p.swap(i, j);
    }

    let mut double = [0u8; 512];
    for i in 0..512 {
        double[i] = p[i & 255];
    }
    double
}

fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

fn gradient(hash: u8, x: f64, y: f64) -> f64 {
    let h = hash & 3;
    let u = if h & 1 == 0 { x } else { -x };
    let v = if h & 2 == 0 { y } else { -y };
    u + v
}

fn perlin_2d_with_perm(x: f64, y: f64, perm: &[u8; 512]) -> f64 {
    let x0 = x.floor() as i32;
    let y0 = y.floor() as i32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;

    let sx = x - x0 as f64;
    let sy = y - y0 as f64;

    let u = fade(sx);
    let v = fade(sy);

    let aa = perm[(perm[(x0 as i32 & 255) as usize] as usize).wrapping_add(y0 as i32 as usize) & 255];
    let ab = perm[(perm[(x0 as i32 & 255) as usize] as usize).wrapping_add(y1 as i32 as usize) & 255];
    let ba = perm[(perm[(x1 as i32 & 255) as usize] as usize).wrapping_add(y0 as i32 as usize) & 255];
    let bb = perm[(perm[(x1 as i32 & 255) as usize] as usize).wrapping_add(y1 as i32 as usize) & 255];

    let x1_lerp = lerp(
        gradient(ba, sx - 1.0, sy),
        gradient(bb, sx - 1.0, sy - 1.0),
        v,
    );
    let x0_lerp = lerp(
        gradient(aa, sx, sy),
        gradient(ab, sx, sy - 1.0),
        v,
    );

    lerp(x0_lerp, x1_lerp, u)
}

pub struct Noise {
    perm: [u8; 512],
}

impl Noise {
    pub fn new(seed: i32) -> Self {
        Self {
            perm: generate_perm(seed),
        }
    }

    pub fn perlin_2d(&self, x: f64, y: f64) -> f64 {
        perlin_2d_with_perm(x, y, &self.perm)
    }

    pub fn fbm(
        &self,
        x: f64,
        y: f64,
        octaves: u32,
        lacunarity: f64,
        persistence: f64,
        scale: f64,
    ) -> f64 {
        let mut value = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = scale;
        let mut max_amplitude = 0.0;

        for _ in 0..octaves {
            value += amplitude * perlin_2d_with_perm(x * frequency, y * frequency, &self.perm);
            max_amplitude += amplitude;
            amplitude *= persistence;
            frequency *= lacunarity;
        }

        value / max_amplitude
    }
}

pub fn perlin_2d(x: f64, y: f64, seed: i32) -> f64 {
    Noise::new(seed).perlin_2d(x, y)
}

pub fn fbm(
    x: f64,
    y: f64,
    octaves: u32,
    lacunarity: f64,
    persistence: f64,
    scale: f64,
    seed: i32,
) -> f64 {
    Noise::new(seed).fbm(x, y, octaves, lacunarity, persistence, scale)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_at_origin() {
        let v = perlin_2d(0.0, 0.0, 42);
        assert_eq!(v, 0.0);
    }

    #[test]
    fn deterministic_perlin() {
        let a = perlin_2d(3.7, 8.2, 42);
        let b = perlin_2d(3.7, 8.2, 42);
        assert_eq!(a, b);
    }

    #[test]
    fn different_seeds_differ() {
        let a = perlin_2d(3.7, 8.2, 42);
        let b = perlin_2d(3.7, 8.2, 99);
        assert_ne!(a, b);
    }

    #[test]
    fn perlin_output_range() {
        for x in 0..10 {
            for y in 0..10 {
                let v = perlin_2d(x as f64 * 0.5, y as f64 * 0.5, 42);
                assert!(v >= -1.0 && v <= 1.0, "perlin_2d({}, {}) = {} out of [-1, 1]", x, y, v);
            }
        }
    }

    #[test]
    fn fbm_one_octave_equals_perlin() {
        let x = 3.7;
        let y = 8.2;
        let seed = 42;
        let p = perlin_2d(x, y, seed);
        let f = fbm(x, y, 1, 2.0, 0.5, 1.0, seed);
        assert!((p - f).abs() < 1e-10, "fbm(1 octave) = {}, perlin_2d = {}", f, p);
    }

    #[test]
    fn fbm_deterministic() {
        let a = fbm(3.7, 8.2, 4, 2.0, 0.5, 1.0, 42);
        let b = fbm(3.7, 8.2, 4, 2.0, 0.5, 1.0, 42);
        assert_eq!(a, b);
    }

    #[test]
    fn fbm_output_range() {
        for octaves in [1, 2, 4, 8] {
            let v = fbm(3.7, 8.2, octaves, 2.0, 0.5, 1.0, 42);
            assert!(
                v >= -1.0 && v <= 1.0,
                "fbm(octaves={}) = {} out of [-1, 1]",
                octaves,
                v
            );
        }
    }

    #[test]
    fn noise_struct_cached_perm() {
        let noise = Noise::new(42);
        let a = noise.perlin_2d(3.7, 8.2);
        let b = noise.perlin_2d(3.7, 8.2);
        assert_eq!(a, b);
        assert!((a - perlin_2d(3.7, 8.2, 42)).abs() < 1e-10);
    }
}
