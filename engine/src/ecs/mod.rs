use bevy_ecs::prelude::Component;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct BiomeId(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub struct Resources(pub u8);

pub mod grid_resource;
pub mod biome_definitions;

pub use grid_resource::GridResource;
pub use biome_definitions::{BiomeDef, BiomeDefinitions};

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
    use crate::rng::Lcg;

    #[test]
    fn biome_id_mapping() {
        assert_eq!(biome_id_from_lcg_value(0), 2);
        assert_eq!(biome_id_from_lcg_value(63), 2);
        assert_eq!(biome_id_from_lcg_value(64), 3);
        assert_eq!(biome_id_from_lcg_value(159), 3);
        assert_eq!(biome_id_from_lcg_value(160), 1);
        assert_eq!(biome_id_from_lcg_value(231), 1);
        assert_eq!(biome_id_from_lcg_value(232), 0);
        assert_eq!(biome_id_from_lcg_value(255), 0);
    }

    #[test]
    fn generate_is_reproducible() {
        let (biomes_a, mut rng_a) = generate_grid_biomes(42, 8, 8);
        let (biomes_b, mut rng_b) = generate_grid_biomes(42, 8, 8);
        assert_eq!(biomes_a, biomes_b);
        assert_eq!(rng_a.next_u32(), rng_b.next_u32());
    }

    #[test]
    fn different_seeds_yield_different_maps() {
        let (biomes_a, _) = generate_grid_biomes(1, 16, 16);
        let (biomes_b, _) = generate_grid_biomes(2, 16, 16);
        assert!(biomes_a.iter().zip(biomes_b.iter()).any(|(a, b)| a != b));
    }

    #[test]
    fn biome_ids_in_valid_range() {
        let (biomes, _) = generate_grid_biomes(7, 10, 10);
        for id in &biomes {
            assert!(*id <= 3, "biome_id {} out of expected range", id);
        }
    }

    fn generate_grid_biomes(seed: u64, width: u32, height: u32) -> (Vec<u16>, Lcg) {
        let mut rng = Lcg::new(seed);
        let mut biomes = Vec::with_capacity((width as usize) * (height as usize));
        for _ in 0..height {
            for _ in 0..width {
                let v = rng.next_u32();
                biomes.push(biome_id_from_lcg_value(v));
            }
        }
        (biomes, rng)
    }
}
