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
    #[serde(default = "default_elevation_very_low")]
    pub elevation_very_low: f64,
    #[serde(default = "default_elevation_sand_max")]
    pub elevation_sand_max: f64,
    #[serde(default = "default_elevation_very_high")]
    pub elevation_very_high: f64,
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
            elevation_very_low: default_elevation_very_low(),
            elevation_sand_max: default_elevation_sand_max(),
            elevation_very_high: default_elevation_very_high(),
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
fn default_elevation_very_low() -> f64 { 0.28 }
fn default_elevation_sand_max() -> f64 { 0.34 }
fn default_elevation_very_high() -> f64 { 0.72 }

pub fn biome_from_noise(elevation: f64, moisture: f64, params: &GenerationParams) -> u16 {
    if elevation < params.elevation_very_low {
        4
    } else if elevation < params.elevation_low {
        2
    } else if elevation < params.elevation_sand_max {
        5
    } else if elevation >= params.elevation_very_high {
        6
    } else if elevation > params.elevation_high {
        3
    } else if moisture > params.moisture_high {
        1
    } else {
        0
    }
}

/// Генерирует сетку биомов заданного размера на основе шума и параметров генерации.
pub fn generate_grid_biomes(seed: u64, width: u32, height: u32) -> Vec<u16> {
    let params = GenerationParams::new(seed);
    let elevation_noise = crate::noise::Noise::new(seed as i32);
    let moisture_noise = crate::noise::Noise::new(seed.wrapping_add(1000) as i32);

    let total_cells = (width as usize) * (height as usize);
    let mut biomes = Vec::with_capacity(total_cells);

    let denom_x = width.max(1) as f64;
    let denom_y = height.max(1) as f64;

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / denom_x;
            let ny = y as f64 / denom_y;

            // Нормализуем шум из диапазона [-1.0; 1.0] в диапазон [0.0; 1.0]
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
            assert!(*id <= 6, "biome_id {} out of expected range", id);
        }
    }

    #[test]
    fn biome_sand_is_generated() {
        // Sand должна появляться как прибрежная полоса между Water и Plains
        let biomes = generate_grid_biomes(42, 256, 256);
        let has_sand = biomes.iter().any(|&id| id == 5);
        assert!(has_sand, "Sand (ID 5) должна быть сгенерирована на карте 256x256");
    }

    #[test]
    fn biome_deep_water_is_generated() {
        let biomes = generate_grid_biomes(42, 256, 256);
        let has_dw = biomes.iter().any(|&id| id == 4);
        assert!(has_dw, "Deep Water (ID 4) должна быть сгенерирована на карте 256x256");
    }

    #[test]
    fn biome_high_mountain_is_generated() {
        let biomes = generate_grid_biomes(42, 256, 256);
        let has_hm = biomes.iter().any(|&id| id == 6);
        assert!(has_hm, "High Mountain (ID 6) должна быть сгенерирована на карте 256x256");
    }

    }
