## Purpose

Perlin noise (FBM) для процедурной генерации карты: два независимых слоя (elevation, moisture) и маппинг на биомы по правилам из `biomes.json`.

## Requirements

### Requirement: 2D Perlin noise функция

Система SHALL реализовать `perlin_2d(x, y, seed) -> f64` в диапазоне `[-1, 1]`.

#### Scenario: Значение noise в нуле

- **WHEN** вызывается `perlin_2d(0.0, 0.0, seed)` с любым seed
- **THEN** возвращается `0.0`

#### Scenario: Детерминизм по seed

- **WHEN** `perlin_2d(x, y, seed)` вызывается дважды с одинаковыми аргументами
- **THEN** оба вызова возвращают одинаковое значение

### Requirement: FBM (Fractional Brownian Motion)

Система SHALL реализовать `fbm(x, y, octaves, lacunarity, persistence, scale, seed) -> f64`, нормализованный к `[-1, 1]`.

#### Scenario: FBM с одной октавой

- **WHEN** `fbm(..., octaves=1, scale=1.0, ...)`
- **THEN** результат эквивалентен `perlin_2d(x * scale, y * scale, seed)`

#### Scenario: Детерминизм FBM

- **WHEN** `fbm` вызывается дважды с одинаковыми параметрами
- **THEN** возвращаются идентичные значения

### Requirement: Два noise-слоя на клетку

При генерации карты система SHALL для каждой клетки `(x, y)` вычислять:
- `elevation` — FBM с `seed` из `GenerationParams`, координаты `nx = x/width`, `ny = y/height`, нормализация `(fbm + 1.0) / 2.0` → `[0, 1]`
- `moisture` — FBM с `seed + 1000`, те же параметры и нормализация

#### Scenario: Независимость слоёв

- **WHEN** используются разные seed для elevation и moisture
- **THEN** поля elevation и moisture пространственно когерентны, но не идентичны

### Requirement: Маппинг noise → biome

Система SHALL предоставлять `biome_from_noise(elevation, moisture) -> u16`, который применяет правила из `biomes.json`:
- обход `definitions` **в порядке массива** (индекс 0 проверяется первым)
- для каждого биома с полем `generation` проверяются условия (все заданные поля должны совпасть)
- первый подошедший биом возвращает свой индекс
- биом с `generation: {}` совпадает всегда (fallback)

Поддерживаемые условия в `generation`:
- `elevationLt` — elevation < порог
- `elevationGt` — elevation > порог
- `elevationGte` — elevation >= порог
- `moistureGt` — moisture > порог

#### Scenario: Deep Water при низкой высоте

- **WHEN** elevation = 0.1, любая moisture
- **THEN** возвращается id биома Deep Water (0 в дефолтном конфиге)

#### Scenario: Water при средне-низкой высоте

- **WHEN** elevation = 0.29, любая moisture
- **THEN** возвращается id биома Water (1)

#### Scenario: Forest при высокой влажности на суше

- **WHEN** elevation = 0.5, moisture = 0.6
- **THEN** возвращается id биома Forest (5)

#### Scenario: Plains как fallback

- **WHEN** elevation = 0.5, moisture = 0.3
- **THEN** возвращается id биома Plains (6)

### Requirement: GenerationParams

Система SHALL определить `GenerationParams` с полями:
- `seed: u64`
- `scale: f64` (по умолчанию 8.0)
- `octaves: u32` (по умолчанию 4)
- `persistence: f64` (по умолчанию 0.5)
- `lacunarity: f64` (по умолчанию 2.0)

Дефолты SHALL загружаться из `biomes.json` → `generationParams`.

#### Scenario: Дефолтные значения шума

- **WHEN** `GenerationParams::new(seed=42)` создаётся с одним seed
- **THEN** scale = 8.0, octaves = 4, persistence = 0.5, lacunarity = 2.0

### Requirement: create_grid через noise

Система SHALL генерировать `biome_ids` в `create_grid` через `biome_from_noise`, а не через LCG.

#### Scenario: Snapshot после noise-генерации

- **WHEN** вызывается `grid_snapshot(handle)` после `create_grid`
- **THEN** `biomeIds` содержит значения в диапазоне `0..=(definitions.len() - 1)`
