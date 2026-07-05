use serde::{Deserialize, Serialize};

pub mod grid_resource;
pub mod biome_definitions;

pub use grid_resource::GridResource;
pub use biome_definitions::{BiomeDef, BiomeDefinitions};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GenerationParams {
    pub seed: u64,
    #[serde(default = "default_scale")]
    pub scale: f64,
    #[serde(default = "default_octaves")]
    pub octaves: u32,
    #[serde(default = "default_persistence")]
    pub persistence: f64,
    #[serde(default = "default_lacunarity")]
    pub lacunarity: f64,
    #[serde(default = "default_elevation_low")]
    pub elevation_low: f64,
    #[serde(default = "default_elevation_high")]
    pub elevation_high: f64,
    #[serde(default = "default_moisture_high")]
    pub moisture_high: f64,
}

impl GenerationParams {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            scale: default_scale(),
            octaves: default_octaves(),
            persistence: default_persistence(),
            lacunarity: default_lacunarity(),
            elevation_low: default_elevation_low(),
            elevation_high: default_elevation_high(),
            moisture_high: default_moisture_high(),
        }
    }
}

fn default_scale() -> f64 { 8.0 }
fn default_octaves() -> u32 { 4 }
fn default_persistence() -> f64 { 0.5 }
fn default_lacunarity() -> f64 { 2.0 }
fn default_elevation_low() -> f64 { 0.30 }
fn default_elevation_high() -> f64 { 0.70 }
fn default_moisture_high() -> f64 { 0.50 }

pub fn biome_from_noise(elevation: f64, moisture: f64, params: &GenerationParams) -> u16 {
    if elevation < params.elevation_low {
        2
    } else if elevation > params.elevation_high {
        3
    } else if moisture > params.moisture_high {
        1
    } else {
        0
    }
}

#[deprecated(note = "Use noise-based generation via biome_from_noise instead")]
pub fn biome_id_from_lcg_value(v: u32) -> u16 {
    match v & 0xFF {
        0..=63 => 2,
        64..=159 => 3,
        160..=231 => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::noise;

    #[test]
    fn generate_is_reproducible() {
        let biomes_a = generate_grid_biomes(42, 8, 8);
        let biomes_b = generate_grid_biomes(42, 8, 8);
        assert_eq!(biomes_a, biomes_b);
    }

    #[test]
    fn different_seeds_yield_different_maps() {
        let biomes_a = generate_grid_biomes(1, 16, 16);
        let biomes_b = generate_grid_biomes(2, 16, 16);
        assert!(biomes_a.iter().zip(biomes_b.iter()).any(|(a, b)| a != b));
    }

    #[test]
    fn biome_ids_in_valid_range() {
        let biomes = generate_grid_biomes(7, 10, 10);
        for id in &biomes {
            assert!(*id <= 3, "biome_id {} out of expected range", id);
        }
    }

    fn generate_grid_biomes(seed: u64, width: u32, height: u32) -> Vec<u16> {
        let params = GenerationParams::new(seed);
        let elevation_noise = noise::Noise::new(seed as i32);
        let moisture_noise = noise::Noise::new(seed.wrapping_add(1000) as i32);
        let mut biomes = Vec::with_capacity((width as usize) * (height as usize));
        for y in 0..height {
            for x in 0..width {
                let nx = x as f64 / width.max(1) as f64;
                let ny = y as f64 / height.max(1) as f64;
                let elevation = (elevation_noise.fbm(
                    nx, ny,
                    params.octaves, params.lacunarity, params.persistence,
                    params.scale,
                ) + 1.0) / 2.0;
                let moisture = (moisture_noise.fbm(
                    nx, ny,
                    params.octaves, params.lacunarity, params.persistence,
                    params.scale,
                ) + 1.0) / 2.0;
                biomes.push(biome_from_noise(elevation, moisture, &params));
            }
        }
        biomes
    }
}
