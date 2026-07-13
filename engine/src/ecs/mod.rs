use bevy_ecs::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use biome_config::{default_biome_definitions, default_generation_params, resolve_biome_from_noise};

pub mod grid_resource;
pub mod biome_definitions;
pub mod biome_config;
pub mod components;
pub mod systems;
pub mod pathfinding;

pub use grid_resource::GridResource;
pub use biome_definitions::{BiomeDef, BiomeDefinitions};
pub use components::*;

#[derive(Debug, Resource)]
pub struct GameTime {
    pub delta: f64,
}

impl GameTime {
    pub fn new() -> Self {
        Self { delta: 0.0 }
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self::new()
    }
}

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
}

impl GenerationParams {
    pub fn new(seed: u64) -> Self {
        let defaults = default_generation_params();
        Self {
            seed,
            scale: defaults.scale,
            octaves: defaults.octaves,
            persistence: defaults.persistence,
            lacunarity: defaults.lacunarity,
        }
    }
}

fn default_scale() -> f64 {
    default_generation_params().scale
}
fn default_octaves() -> u32 {
    default_generation_params().octaves
}
fn default_persistence() -> f64 {
    default_generation_params().persistence
}
fn default_lacunarity() -> f64 {
    default_generation_params().lacunarity
}

static DEFAULT_BIOME_DEFS: LazyLock<BiomeDefinitions> =
    LazyLock::new(default_biome_definitions);

pub fn biome_from_noise(elevation: f64, moisture: f64) -> u16 {
    resolve_biome_from_noise(elevation, moisture, &DEFAULT_BIOME_DEFS)
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

            biomes.push(biome_from_noise(elevation, moisture));
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
        let defs = default_biome_definitions();
        let max_id = (defs.definitions.len() - 1) as u16;
        let biomes = generate_grid_biomes(7, 10, 10);
        for id in &biomes {
            assert!(*id <= max_id, "biome_id {} out of expected range", id);
        }
    }

    #[test]
    fn biome_sand_is_generated() {
        let defs = default_biome_definitions();
        let sand_id = defs.id_by_name("Sand").expect("Sand biome must exist");
        let biomes = generate_grid_biomes(42, 256, 256);
        let has_sand = biomes.iter().any(|&id| id == sand_id);
        assert!(has_sand, "Sand должна быть сгенерирована на карте 256x256");
    }

    #[test]
    fn biome_deep_water_is_generated() {
        let defs = default_biome_definitions();
        let deep_water_id = defs.id_by_name("Deep Water").expect("Deep Water biome must exist");
        let biomes = generate_grid_biomes(42, 256, 256);
        let has_dw = biomes.iter().any(|&id| id == deep_water_id);
        assert!(has_dw, "Deep Water должна быть сгенерирована на карте 256x256");
    }

    #[test]
    fn biome_high_mountain_is_generated() {
        let defs = default_biome_definitions();
        let high_mountain_id = defs
            .id_by_name("High Mountain")
            .expect("High Mountain biome must exist");
        let biomes = generate_grid_biomes(42, 256, 256);
        let has_hm = biomes.iter().any(|&id| id == high_mountain_id);
        assert!(has_hm, "High Mountain должна быть сгенерирована на карте 256x256");
    }

    }
