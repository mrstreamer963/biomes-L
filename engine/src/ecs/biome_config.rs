use serde::Deserialize;

use super::biome_definitions::{BiomeDef, BiomeDefinitions};
use super::GenerationParams;

const BIOMES_JSON: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../src/config/biomes.json"));

#[derive(Debug, Deserialize)]
struct BiomesFile {
    definitions: Vec<BiomeDef>,
    #[serde(rename = "generationParams")]
    generation_params: GenerationParamsJson,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenerationParamsJson {
    seed: u64,
    scale: f64,
    octaves: u32,
    persistence: f64,
    lacunarity: f64,
    elevation_low: f64,
    elevation_high: f64,
    moisture_high: f64,
    elevation_very_low: f64,
    elevation_very_high: f64,
    elevation_sand_max: f64,
}

fn load_biomes_file() -> BiomesFile {
    serde_json::from_str(BIOMES_JSON).expect("biomes.json must be valid")
}

pub fn default_biome_definitions() -> BiomeDefinitions {
    BiomeDefinitions::new(load_biomes_file().definitions)
}

pub fn default_generation_params() -> GenerationParams {
    let gp = load_biomes_file().generation_params;
    GenerationParams {
        seed: gp.seed,
        scale: gp.scale,
        octaves: gp.octaves,
        persistence: gp.persistence,
        lacunarity: gp.lacunarity,
        elevation_low: gp.elevation_low,
        elevation_high: gp.elevation_high,
        moisture_high: gp.moisture_high,
        elevation_very_low: gp.elevation_very_low,
        elevation_sand_max: gp.elevation_sand_max,
        elevation_very_high: gp.elevation_very_high,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_has_seven_biomes_in_expected_order() {
        let defs = default_biome_definitions();
        assert_eq!(defs.definitions.len(), 7);
        assert_eq!(defs.get_name(0), Some("Plains"));
        assert_eq!(defs.get_name(1), Some("Forest"));
        assert_eq!(defs.get_name(2), Some("Water"));
        assert_eq!(defs.get_name(3), Some("Mountain"));
        assert_eq!(defs.get_name(4), Some("Deep Water"));
        assert_eq!(defs.get_name(5), Some("Sand"));
        assert_eq!(defs.get_name(6), Some("High Mountain"));
    }

    #[test]
    fn json_biome_properties_match_expected() {
        let defs = default_biome_definitions();
        assert!(defs.is_passable(0));
        assert_eq!(defs.speed_factor(0), 1.0);
        assert_eq!(defs.get_color(0), 0x7ec850);
        assert!(defs.is_passable(1));
        assert_eq!(defs.speed_factor(1), 0.6);
        assert_eq!(defs.get_color(1), 0x2d5a27);
        assert!(!defs.is_passable(2));
        assert_eq!(defs.get_color(2), 0x3b82f6);
        assert!(!defs.is_passable(3));
        assert_eq!(defs.get_color(3), 0x8b7355);
        assert!(!defs.is_passable(4));
        assert_eq!(defs.get_color(4), 0x1e3a5f);
        assert!(defs.is_passable(5));
        assert_eq!(defs.speed_factor(5), 0.9);
        assert_eq!(defs.get_color(5), 0xeedd88);
        assert!(!defs.is_passable(6));
        assert_eq!(defs.get_color(6), 0xffffff);
    }

    #[test]
    fn id_by_name_resolves_all_biomes() {
        let defs = default_biome_definitions();
        assert_eq!(defs.id_by_name("Plains"), Some(0));
        assert_eq!(defs.id_by_name("Forest"), Some(1));
        assert_eq!(defs.id_by_name("Water"), Some(2));
        assert_eq!(defs.id_by_name("Mountain"), Some(3));
        assert_eq!(defs.id_by_name("Deep Water"), Some(4));
        assert_eq!(defs.id_by_name("Sand"), Some(5));
        assert_eq!(defs.id_by_name("High Mountain"), Some(6));
        assert_eq!(defs.id_by_name("Unknown"), None);
    }

    #[test]
    fn default_generation_params_match_json() {
        let params = default_generation_params();
        assert_eq!(params.seed, 42);
        assert_eq!(params.scale, 8.0);
        assert_eq!(params.octaves, 4);
        assert_eq!(params.persistence, 0.5);
        assert_eq!(params.lacunarity, 2.0);
        assert_eq!(params.elevation_low, 0.30);
        assert_eq!(params.elevation_high, 0.70);
        assert_eq!(params.moisture_high, 0.50);
        assert_eq!(params.elevation_very_low, 0.28);
        assert_eq!(params.elevation_very_high, 0.72);
        assert_eq!(params.elevation_sand_max, 0.34);
    }
}
