## Context

The current biome system generates 4 biomes (Water, Plains, Forest, Mountain) using a two-layer noise approach: elevation noise determines land vs. water, and moisture noise differentiates forest from plains. The biome definitions are stored as a dynamic array indexed by ID, so adding new biomes is a matter of extending the definitions array and adjusting the noise-to-biome mapping function.

The codebase is small and self-contained: a Rust engine (compiled to WASM) and a TypeScript/ Vue frontend. Biomes are referenced by numeric ID throughout, so reordering/additions are safe as long as the definitions array stays consistent across both sides.

## Goals / Non-Goals

**Goals:**
- Add 3 new biome definitions (Deep Water, Sand, High Mountain) to both Rust and TypeScript
- Add 2 new elevation thresholds (`elevation_very_low`, `elevation_very_high`) to `GenerationParams`
- Expand `biome_from_noise()` to map 6 elevation/moisture bands → 6 biomes (Sand is defined but not generated yet)
- All existing tests pass with updated expected values
- All generated biome IDs are in range 0..=6

**Non-Goals:**
- Sand biome generation logic (coastal transitions) — deferred to future work
- Performance optimization of noise generation
- UI changes — the StatusPanel already renders dynamically from definitions
- Any new WASM or worker complexity

## Decisions

### Decision: New elevation thresholds, not reusing existing ones
The existing `elevation_low` (0.30) and `elevation_high` (0.70) thresholds define the main water/land and land/mountain boundaries. Adding `elevation_very_low` (0.28) below the water line and `elevation_very_high` (0.72) above the mountain line creates three additional bands without changing the existing terrain character.

**Alternative considered:** Replacing `elevation_high` with a higher value and keeping only 4 bands. This would shift existing terrain, potentially breaking the existing Plains/Forest/Mountain balance. Separate thresholds are additive, not disruptive.

### Decision: Sand is generated as a narrow coastal band
Sand is generated as a narrow elevation band between Water and Plains using `elevation_sand_max` (0.34). This creates visible sandy coastlines around water bodies. The band is intentionally narrow (~2% of cells) to avoid dominating the landscape.

**Alternative considered:** Deferring Sand to a future coastal-detection pass. However, the simple elevation band works well enough visually and gives immediate feedback in the map.

### Decision: Biome IDs are explicit (0–6) not dynamic
The `biome_from_noise()` function returns hardcoded IDs. With the expansion, the mapping is:
- 0 = Plains, 1 = Forest, 2 = Water, 3 = Mountain, 4 = Deep Water, 5 = Sand, 6 = High Mountain

IDs 0–3 are unchanged from the current system. This preserves backward compatibility for any code that implicitly relies on these IDs (e.g., test assertions).

## Risks / Trade-offs

- [Risk] New elevation thresholds may need tuning after visual inspection — default values (0.15, 0.85) are educated guesses based on the existing range
  → Mitigation: Thresholds are configurable in `GenerationParams`, so tuning is a single-field change
- [Risk] Sand biome exists in definitions but is never generated, which may confuse developers
  → Mitigation: Document this clearly in both the spec and code comments
- [Risk RESOLVED] Sand is now generated as a coastal band via elevation_sand_max threshold
- [Risk] Biome IDs are now hardcoded in `biome_from_noise()`, which is fragile if biomes are reordered
  → Mitigation: The function is a single, well-tested switch point; future expansions should use a configurable biome table