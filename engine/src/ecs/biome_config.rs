use serde::Deserialize;

use super::biome_definitions::{BiomeDef, BiomeDefinitions, GenerationConditions};
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
    }
}

fn conditions_match(conditions: &GenerationConditions, elevation: f64, moisture: f64) -> bool {
    if let Some(v) = conditions.elevation_lt {
        if elevation >= v {
            return false;
        }
    }
    if let Some(v) = conditions.elevation_gt {
        if elevation <= v {
            return false;
        }
    }
    if let Some(v) = conditions.elevation_gte {
        if elevation < v {
            return false;
        }
    }
    if let Some(v) = conditions.moisture_gt {
        if moisture <= v {
            return false;
        }
    }
    true
}

pub fn resolve_biome_from_noise(
    elevation: f64,
    moisture: f64,
    defs: &BiomeDefinitions,
) -> u16 {
    for (id, def) in defs.definitions.iter().enumerate() {
        if let Some(conditions) = &def.generation {
            if conditions_match(conditions, elevation, moisture) {
                return id as u16;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_has_seven_biomes_in_expected_order() {
        let defs = default_biome_definitions();
        assert_eq!(defs.definitions.len(), 7);
        assert_eq!(defs.get_name(0), Some("Deep Water"));
        assert_eq!(defs.get_name(1), Some("Water"));
        assert_eq!(defs.get_name(2), Some("Sand"));
        assert_eq!(defs.get_name(3), Some("High Mountain"));
        assert_eq!(defs.get_name(4), Some("Mountain"));
        assert_eq!(defs.get_name(5), Some("Forest"));
        assert_eq!(defs.get_name(6), Some("Plains"));
    }

    #[test]
    fn json_biome_properties_match_expected() {
        let defs = default_biome_definitions();
        assert!(!defs.is_passable(0));
        assert_eq!(defs.get_color(0), 0x1e3a5f);
        assert!(!defs.is_passable(1));
        assert_eq!(defs.get_color(1), 0x3b82f6);
        assert!(defs.is_passable(2));
        assert_eq!(defs.speed_factor(2), 0.9);
        assert_eq!(defs.get_color(2), 0xeedd88);
        assert!(!defs.is_passable(3));
        assert_eq!(defs.get_color(3), 0xffffff);
        assert!(!defs.is_passable(4));
        assert_eq!(defs.get_color(4), 0x8b7355);
        assert!(defs.is_passable(5));
        assert_eq!(defs.speed_factor(5), 0.6);
        assert_eq!(defs.get_color(5), 0x2d5a27);
        assert!(defs.is_passable(6));
        assert_eq!(defs.speed_factor(6), 1.0);
        assert_eq!(defs.get_color(6), 0x7ec850);
    }

    #[test]
    fn id_by_name_resolves_all_biomes() {
        let defs = default_biome_definitions();
        assert_eq!(defs.id_by_name("Deep Water"), Some(0));
        assert_eq!(defs.id_by_name("Water"), Some(1));
        assert_eq!(defs.id_by_name("Sand"), Some(2));
        assert_eq!(defs.id_by_name("High Mountain"), Some(3));
        assert_eq!(defs.id_by_name("Mountain"), Some(4));
        assert_eq!(defs.id_by_name("Forest"), Some(5));
        assert_eq!(defs.id_by_name("Plains"), Some(6));
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
    }

    #[test]
    fn generation_rules_follow_array_order() {
        let defs = default_biome_definitions();
        assert_eq!(defs.get_name(0), Some("Deep Water"));
        assert_eq!(defs.get_name(6), Some("Plains"));
    }

    #[test]
    fn generation_rules_resolve_expected_biomes() {
        let defs = default_biome_definitions();

        assert_eq!(
            resolve_biome_from_noise(0.1, 0.5, &defs),
            defs.id_by_name("Deep Water").unwrap()
        );
        assert_eq!(
            resolve_biome_from_noise(0.29, 0.5, &defs),
            defs.id_by_name("Water").unwrap()
        );
        assert_eq!(
            resolve_biome_from_noise(0.32, 0.5, &defs),
            defs.id_by_name("Sand").unwrap()
        );
        assert_eq!(
            resolve_biome_from_noise(0.9, 0.5, &defs),
            defs.id_by_name("High Mountain").unwrap()
        );
        assert_eq!(
            resolve_biome_from_noise(0.71, 0.5, &defs),
            defs.id_by_name("Mountain").unwrap()
        );
        assert_eq!(
            resolve_biome_from_noise(0.5, 0.6, &defs),
            defs.id_by_name("Forest").unwrap()
        );
        assert_eq!(
            resolve_biome_from_noise(0.5, 0.3, &defs),
            defs.id_by_name("Plains").unwrap()
        );
    }
}
