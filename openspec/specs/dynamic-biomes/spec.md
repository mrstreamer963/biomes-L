## Purpose

Единый источник истины для биомов: `src/config/biomes.json`, динамический реестр `BiomeDefinitions` в ECS и регистрация из JS при старте.

## Requirements

### Requirement: Реестр определений биомов

Система SHALL предоставить Resource `BiomeDefinitions` в bevy_ecs World с произвольным списком `BiomeDef`. Каждое определение включает:
- `name: String`
- `passable: bool`
- `speed_factor: f32` (в JSON — поле `speed`)
- `color: u32` (десятичный RGB, эквивалент `0xRRGGBB`)
- `generation: Option<GenerationConditions>` — опциональные условия для noise-маппинга

#### Scenario: Регистрация биомов

- **WHEN** вызывается `register_biome_definitions(handle, definitions)` с массивом из JS
- **THEN** BiomeDefinitions Resource обновляется, индекс в массиве = BiomeId

#### Scenario: Обновление определений

- **WHEN** вызывается `register_biome_definitions` с новым массивом
- **THEN** BiomeDefinitions Resource заменяется полностью

### Requirement: Доступ к свойствам биома по BiomeId

Система SHALL предоставить методы на `BiomeDefinitions`:
- `get_name(id) -> Option<&str>`
- `id_by_name(name) -> Option<u16>`
- `is_passable(id) -> bool` (false для неизвестного id)
- `speed_factor(id) -> f32` (0.0 для неизвестного id)
- `get_color(id) -> u32` (0x000000 для неизвестного id)

#### Scenario: Получение свойств

- **WHEN** вызывается `biome_defs.get_name(0)` с дефолтным конфигом
- **THEN** возвращается `"Deep Water"`

#### Scenario: Поиск по имени

- **WHEN** вызывается `biome_defs.id_by_name("Plains")`
- **THEN** возвращается `Some(6)` в дефолтном конфиге

### Requirement: Конфиг биомов — biomes.json

Система SHALL использовать `src/config/biomes.json` как единственный источник для:
- `definitions` — массив биомов (порядок = BiomeId = порядок проверки generation-правил)
- `generationParams` — параметры шума (seed, scale, octaves, persistence, lacunarity)

TypeScript импортирует JSON через `src/config/biomeConfig.ts` и передаёт `definitions` в WASM через `register_biome_definitions`. Rust читает тот же файл через `include_str!` в `engine/src/ecs/biome_config.rs` для генерации карты.

#### Scenario: Дефолтный конфиг — 7 биомов

- **WHEN** приложение стартует с дефолтным `biomes.json`
- **THEN** массив `definitions` содержит 7 биомов в порядке генерации:

| BiomeId | Биом | generation |
|---------|------|------------|
| 0 | Deep Water | elevationLt: 0.28 |
| 1 | Water | elevationLt: 0.30 |
| 2 | Sand | elevationLt: 0.34 |
| 3 | High Mountain | elevationGte: 0.72 |
| 4 | Mountain | elevationGt: 0.70 |
| 5 | Forest | moistureGt: 0.50 |
| 6 | Plains | {} (fallback) |

#### Scenario: Добавление биома

- **WHEN** в `biomes.json` добавляется одна запись в `definitions` с полем `generation` в нужной позиции массива
- **THEN** новый биом появляется на карте без изменения Rust-кода

#### Scenario: TS и Rust используют один JSON

- **WHEN** изменяется цвет или passable биома в `biomes.json`
- **THEN** и рендер (TS), и pathfinding (WASM после register) видят одинаковые свойства
