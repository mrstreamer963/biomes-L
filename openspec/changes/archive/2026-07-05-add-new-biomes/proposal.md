## Why

The current biome system has only 4 biomes (Water, Plains, Forest, Mountain), which makes the generated terrain feel flat and uninteresting. Adding **Deep Water**, **Sand**, and **High Mountain** will create more natural-looking coastlines, deeper oceans, and dramatic peaks — making the map visually richer and more varied.

## What Changes

- Add `Deep Water` biome — very low elevation areas (deep ocean trenches)
- Add `Sand` biome — transition zone between water and fertile land
- Add `High Mountain` biome — extreme elevation peaks above current mountains
- Add new elevation thresholds (`elevation_very_low`, `elevation_sand_max`, `elevation_very_high`) to `GenerationParams` (defaults: 0.28, 0.34, 0.72)
- Update `biome_from_noise()` to map the expanded elevation/moisture space
- Update biome definitions in both Rust (`BiomeDef`) and TypeScript (`biomeConfig.ts`)
- Update tests in `mod.rs` and `biome_definitions.rs`
- **No breaking changes** — existing biome IDs shift (Water=2→3, Mountain=3→4, Plains=0→0, Forest=1→1), but all biome access is ID-based from definitions, not hardcoded

## Capabilities

### New Capabilities
- `biome-expansion`: Adding new biomes (Deep Water, Sand, High Mountain) with corresponding elevation thresholds, noise mapping, and visual definitions

### Modified Capabilities
<!-- No existing specs to modify -->

## Impact

- **engine/src/ecs/mod.rs**: `GenerationParams` gains 2 new fields (`elevation_very_low`, `elevation_very_high`); `biome_from_noise()` logic expanded
- **engine/src/ecs/biome_definitions.rs**: Test fixtures updated to include new biomes
- **engine/src/wasm_api.rs**: No changes needed (fully dynamic via definitions)
- **src/config/biomeConfig.ts**: `DEFAULT_BIOME_DEFINITIONS` adds 3 new entries; `DEFAULT_GENERATION_PARAMS` adds 2 new fields
- **src/components/StatusPanel.vue**: No changes needed (renders dynamically from definitions)
- **src/game/MapRenderer.ts**: No changes needed (renders dynamically from definitions)
- **src/worker/game.worker.ts**: No changes needed (passes definitions through)