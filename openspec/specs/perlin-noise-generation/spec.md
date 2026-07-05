## Purpose

TBD — Perlin noise generation and biome mapping for procedural world generation.

## Requirements

### Requirement: 2D Perlin noise функция

Система SHALL реализовать функцию `perlin_2d(x: f64, y: f64, seed: i32) -> f64`, возвращающую значение Perlin noise в диапазоне `[-1, 1]` для произвольных координат `(x, y)`.

#### Scenario: Значение noise в нуле
- **WHEN** вызывается `perlin_2d(0.0, 0.0, seed=42)` с любым seed
- **THEN** возвращается `0.0` (гарантировано алгоритмом)

#### Scenario: Детерминизм по seed
- **WHEN** вызывается `perlin_2d(x, y, seed)` дважды с одинаковыми аргументами
- **THEN** оба вызова возвращают одинаковое значение

#### Scenario: Разные seed дают разные значения
- **WHEN** вызывается `perlin_2d(x, y, seed1)` и `perlin_2d(x, y, seed2)` с разными seed
- **THEN** значения различаются как минимум в 50% случаев

### Requirement: FBM (Fractional Brownian Motion)

Система SHALL реализовать функцию `fbm(x: f64, y: f64, octaves: u32, lacunarity: f64, persistence: f64, scale: f64, seed: i32) -> f64`, суммирующую несколько октав Perlin noise с изменяющейся амплитудой и частотой. Результат SHALL быть нормализован к диапазону `[-1, 1]` независимо от количества октав.

#### Scenario: FBM с одной октавой
- **WHEN** вызывается `fbm(x, y, octaves=1, scale=1.0, ...)`
- **THEN** результат эквивалентен `perlin_2d(x * 1.0, y * 1.0, seed)`

#### Scenario: FBM с несколькими октавами
- **WHEN** вызывается `fbm(x, y, octaves=4, persistence=0.5, lacunarity=2.0, scale=1.0, seed)`
- **THEN** результат содержит вклад 4 октав с уменьшающейся амплитудой

#### Scenario: Детерминизм FBM
- **WHEN** вызывается `fbm` с одинаковыми параметрами дважды
- **THEN** возвращаются идентичные значения

### Requirement: Маппинг noise → biome

Система SHALL предоставлять функцию `biome_from_noise(elevation: f64, moisture: f64, thresholds: BiomeThresholds) -> u16`, которая по двум noise-значениям и пороговой таблице определяет индекс биома.

#### Scenario: Определение Water
- **WHEN** `elevation < thresholds.elevation_low`
- **THEN** возвращается id биома, соответствующего Water

#### Scenario: Определение Mountain
- **WHEN** `elevation > thresholds.elevation_high`
- **THEN** возвращается id биома, соответствующего Mountain

#### Scenario: Определение Forest/Plains по влажности
- **WHEN** `elevation` в среднем диапазоне и `moisture > thresholds.moisture_high`
- **THEN** возвращается id биома леса
- **WHEN** `elevation` в среднем диапазоне и `moisture <= thresholds.moisture_high`
- **THEN** возвращается id биома равнин

### Requirement: GenerationParams

Система SHALL определить структуру `GenerationParams` с полями:
- `seed: u64` — seed для генерации permutation table
- `scale: f64` — базовый пространственный масштаб noise (по умолчанию 8.0)
- `octaves: u32` — количество октав FBM (по умолчанию 4)
- `persistence: f64` — коэффициент амплитуды октав (по умолчанию 0.5)
- `lacunarity: f64` — коэффициент частоты октав (по умолчанию 2.0)
- `elevation_low: f64` — порог воды (по умолчанию 0.30)
- `elevation_high: f64` — порог гор (по умолчанию 0.70)
- `moisture_high: f64` — порог леса (по умолчанию 0.50)

Все значения SHALL иметь дефолты, чтобы можно было вызвать `create_grid` только с seed.

#### Scenario: Дефолтные значения
- **WHEN** `GenerationParams::new(seed=42)` создаётся с одним seed
- **THEN** scale = 8.0, octaves = 4, persistence = 0.5, lacunarity = 2.0, elevation_low = 0.30, elevation_high = 0.70, moisture_high = 0.50

### Requirement: Обновлённый create_grid

Система SHALL предоставлять WASM-функцию `create_grid(params: GenerationParams, width: u32, height: u32) -> u32`, которая генерирует сетку биомов с использованием noise-функций вместо LCG.

#### Scenario: Создание сетки с шумом
- **WHEN** вызывается `create_grid(params, width=8, height=8)`
- **THEN** возвращается ненулевой дескриптор, и биомы образуют пространственно-когерентные зоны (соседние клетки чаще имеют одинаковый биом)

#### Scenario: Наследование snapshot API
- **WHEN** вызывается `grid_snapshot(handle)` для сетки, созданной через noise
- **THEN** возвращается валидный snapshot с корректными biomeIds (0..=3)
