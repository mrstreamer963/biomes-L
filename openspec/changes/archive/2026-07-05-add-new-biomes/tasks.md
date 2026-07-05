## 1. Rust: Expand generation parameters

- [x] 1.1 Add `elevation_very_low` (default 0.28), `elevation_sand_max` (default 0.34), and `elevation_very_high` (default 0.72) fields to `GenerationParams` in `engine/src/ecs/mod.rs`
- [x] 1.2 Add default functions `default_elevation_very_low()`, `default_elevation_sand_max()`, and `default_elevation_very_high()`
- [x] 1.3 Add `#[serde(default = "...")]` attributes on both new fields for deserialization
- [x] 1.4 Update `GenerationParams::new()` to set defaults for the two new fields

## 2. Rust: Expand biome definitions

- [x] 2.1 Add `Deep Water`, `Sand`, and `High Mountain` entries to the test fixture `test_defs()` in `engine/src/ecs/biome_definitions.rs`
- [x] 2.2 Update tests in `biome_definitions.rs` to cover new biomes (get_name, is_passable, speed_factor, get_color for IDs 4, 5, 6)

## 3. Rust: Update noise-to-biome mapping

- [x] 3.1 Expand `biome_from_noise()` in `engine/src/ecs/mod.rs` to map the 7 elevation/moisture bands to biomes 0–6:
  - elevation < very_low → 4 (Deep Water)
  - very_low ≤ elevation < low → 2 (Water)
  - low ≤ elevation < sand_max → 5 (Sand)
  - sand_max ≤ elevation ≤ high, moisture > high → 1 (Forest)
  - sand_max ≤ elevation ≤ high, moisture ≤ high → 0 (Plains)
  - high < elevation < very_high → 3 (Mountain)
  - elevation ≥ very_high → 6 (High Mountain)
- [x] 3.2 Update the test `biome_ids_in_valid_range` to expect IDs 0..=6 instead of 0..=3

## 4. TypeScript: Update biome config

- [x] 4.1 Add `elevationVeryLow` (0.28), `elevationVeryHigh` (0.72), and `elevationSandMax` (0.34) fields to `GenerationParams` interface and `DEFAULT_GENERATION_PARAMS` in `src/config/biomeConfig.ts`
- [x] 4.2 Add Deep Water, Sand, and High Mountain to `DEFAULT_BIOME_DEFINITIONS` array with correct properties

## 5. Verify

- [x] 5.1 Run `cargo test` in `engine/` — all Rust tests pass
- [x] 5.2 Run `npm run build` — TypeScript compilation succeeds
- [x] 5.3 Start dev server and visually confirm the map shows 7 biomes in the StatusPanel sidebar