## Purpose

Генерация и представление игровой сетки биомов на основе динамического реестра `BiomeDefinitions` и плоского массива `biome_ids`.

## Requirements

### Requirement: Динамический BiomeId

Система SHALL использовать числовой `BiomeId` (`u16`) как индекс в массиве `BiomeDefinitions.definitions`. Свойства биома (name, passable, speed_factor, color) SHALL храниться в `BiomeDef`, а не в enum.

#### Scenario: Индекс совпадает с позицией в конфиге

- **WHEN** загружается дефолтный `src/config/biomes.json`
- **THEN** `BiomeId` 0 соответствует первому биому в массиве `definitions` (Deep Water), а последний индекс — Plains

### Requirement: GridResource

Система SHALL хранить карту как `GridResource { width, height, biome_ids: Vec<u16>, resources: Vec<u8> }`. Поле `resources` зарезервировано под будущую добычу.

#### Scenario: Доступ к клетке по координатам

- **WHEN** запрашивается `biome_ids[y * width + x]` при валидных `(x, y)`
- **THEN** возвращается `BiomeId` клетки

#### Scenario: Координаты вне границ

- **WHEN** координаты вне `width × height`
- **THEN** `cell_at` возвращает biome = 255 (BIOME_NONE), без паники

### Requirement: Детерминированная генерация карты из seed

Система SHALL генерировать сетку биомов детерминированно из `GenerationParams` (seed, scale, octaves, persistence, lacunarity), `width` и `height`. Правила маппинга шума на биомы SHALL читаться из `src/config/biomes.json` (поле `generation` у каждого биома). Один и тот же набор параметров SHALL давать идентичную карту.

#### Scenario: Воспроизводимость карты

- **WHEN** генерируются две сетки с одинаковыми `GenerationParams`, `width`, `height`
- **THEN** `biome_ids` на одинаковых координатах совпадают

#### Scenario: Пространственная когерентность

- **WHEN** генерируется сетка с Perlin FBM
- **THEN** соседние клетки чаще имеют одинаковый биом, чем при случайном распределении

#### Scenario: Разные seed дают разные карты

- **WHEN** генерируются две сетки с разными `seed` при одинаковых размерах
- **THEN** распределение биомов различается

### Requirement: WASM API для создания и получения сетки

Система SHALL предоставлять:
- `create_grid(params: GenerationParams, width, height) -> u32` — дескриптор сетки
- `register_biome_definitions(handle, definitions)` — регистрация свойств биомов для pathfinding/рендера
- `grid_snapshot(handle)` — `{ width, height, biomeIds, resources }`
- `cell_at(handle, x, y)` — точечный запрос клетки

`GenerationParams` SHALL содержать только параметры шума (seed, scale, octaves, persistence, lacunarity). Пороги elevation/moisture SHALL находиться в `biomes.json`, а не в `GenerationParams`.

#### Scenario: Создание сетки

- **WHEN** вызывается `create_grid({ seed: 42, scale: 8.0, octaves: 4 }, width=50, height=50)`
- **THEN** возвращается ненулевой дескриптор, карта сгенерирована по правилам из `biomes.json`

#### Scenario: Snapshot сетки

- **WHEN** вызывается `grid_snapshot(handle)` для валидного дескриптора
- **THEN** возвращается объект с `width`, `height`, массивом `biomeIds` (значения `0..=N-1`, где N — число биомов в конфиге) и `resources`

#### Scenario: Snapshot несуществующего дескриптора

- **WHEN** вызывается `grid_snapshot` с невалидным дескриптором
- **THEN** операция не паникует и возвращает null
