## ADDED Requirements

### Requirement: Biome definitions include Deep Water, Sand, and High Mountain

The system SHALL define 7 biomes total: Plains, Forest, Water, Mountain, Deep Water, Sand, High Mountain.

| Biome | ID | Passable | Speed | Color |
|-------|----|----------|-------|-------|
| Plains | 0 | true | 1.0 | 0x7ec850 |
| Forest | 1 | true | 0.6 | 0x2d5a27 |
| Water | 2 | false | 0.0 | 0x3b82f6 |
| Mountain | 3 | false | 0.0 | 0x8b7355 |
| Deep Water | 4 | false | 0.0 | 0x1e3a5f |
| Sand | 5 | true | 0.9 | 0xeedd88 |
| High Mountain | 6 | false | 0.0 | 0xffffff |

#### Scenario: Deep Water definition exists
- **WHEN** the biome definitions are loaded
- **THEN** there SHALL be a biome with name "Deep Water" at ID 4, passable=false, speed=0.0, color=0x1e3a5f

#### Scenario: Sand definition exists
- **WHEN** the biome definitions are loaded
- **THEN** there SHALL be a biome with name "Sand" at ID 5, passable=true, speed=0.9, color=0xeedd88

#### Scenario: High Mountain definition exists
- **WHEN** the biome definitions are loaded
- **THEN** there SHALL be a biome with name "High Mountain" at ID 6, passable=false, speed=0.0, color=0xffffff

#### Scenario: Biome count is 7
- **WHEN** the biome definitions are loaded
- **THEN** there SHALL be exactly 7 entries in the definitions array

### Requirement: GenerationParams has new elevation thresholds

The system SHALL include `elevation_very_low`, `elevation_sand_max`, and `elevation_very_high` fields in `GenerationParams` with default values 0.28, 0.34, and 0.72 respectively.

#### Scenario: Default elevation_very_low is 0.28
- **WHEN** `GenerationParams::new()` is called
- **THEN** `elevation_very_low` SHALL default to 0.28

#### Scenario: Default elevation_very_high is 0.72
- **WHEN** `GenerationParams::new()` is called
- **THEN** `elevation_very_high` SHALL default to 0.72

### Requirement: Noise-to-biome mapping uses expanded elevation thresholds

The `biome_from_noise()` function SHALL map elevation and moisture to 7 biomes as follows:

| Elevation Range | Moisture Condition | Biome ID |
|-----------------|-------------------|----------|
| < elevation_very_low | any | 4 (Deep Water) |
| elevation_very_low .. elevation_low | any | 2 (Water) |
| elevation_low .. elevation_sand_max | any | 5 (Sand) |
| elevation_sand_max .. elevation_high | > moisture_high | 1 (Forest) |
| elevation_sand_max .. elevation_high | <= moisture_high | 0 (Plains) |
| elevation_high .. elevation_very_high | any | 3 (Mountain) |
| >= elevation_very_high | any | 6 (High Mountain) |

Sand is generated as a narrow coastal band between Water and Plains using the `elevation_sand_max` (default 0.34) threshold.

#### Scenario: Very low elevation produces Deep Water
- **WHEN** elevation is 0.1 and any moisture value
- **THEN** `biome_from_noise()` SHALL return 4 (Deep Water)

#### Scenario: Low elevation produces Water
- **WHEN** elevation is 0.2 and any moisture value
- **THEN** `biome_from_noise()` SHALL return 2 (Water)

#### Scenario: Low elevation near water produces Sand
- **WHEN** elevation is 0.32 and any moisture value
- **THEN** `biome_from_noise()` SHALL return 5 (Sand)

#### Scenario: Very high elevation produces High Mountain
- **WHEN** elevation is 0.9 and any moisture value
- **THEN** `biome_from_noise()` SHALL return 6 (High Mountain)

#### Scenario: High elevation below threshold produces Mountain
- **WHEN** elevation is 0.75 and any moisture value
- **THEN** `biome_from_noise()` SHALL return 3 (Mountain)

### Requirement: Biome IDs stay within valid range

All generated biome IDs SHALL be in the range 0..=6.

#### Scenario: Generated IDs are valid
- **WHEN** a grid of 100x100 cells is generated with any seed
- **THEN** every biome ID SHALL be between 0 and 6 inclusive

### Requirement: TypeScript config mirrors Rust definitions

The `DEFAULT_BIOME_DEFINITIONS` in `biomeConfig.ts` SHALL contain the same 7 entries as the Rust definitions, with matching names, passable flags, speed factors, and colors.

#### Scenario: TypeScript config has 7 biomes
- **WHEN** `DEFAULT_BIOME_DEFINITIONS` is loaded
- **THEN** it SHALL have exactly 7 entries

#### Scenario: TypeScript Deep Water matches Rust
- **WHEN** comparing `DEFAULT_BIOME_DEFINITIONS[4]`
- **THEN** name SHALL be "Deep Water", passable SHALL be false, speed SHALL be 0.0, color SHALL be 0x1e3a5f

#### Scenario: TypeScript Sand matches Rust
- **WHEN** comparing `DEFAULT_BIOME_DEFINITIONS[5]`
- **THEN** name SHALL be "Sand", passable SHALL be true, speed SHALL be 0.9, color SHALL be 0xeedd88

#### Scenario: TypeScript High Mountain matches Rust
- **WHEN** comparing `DEFAULT_BIOME_DEFINITIONS[6]`
- **THEN** name SHALL be "High Mountain", passable SHALL be false, speed SHALL be 0.0, color SHALL be 0xffffff