## Purpose

ECS-компоненты и WASM API для управления юнитами.

## Requirements

### Requirement: Unit ECS компоненты

- `Position { x, y }` — пиксели
- `MovementTarget(Option<(f64, f64)>)`
- `BaseSpeed(f64)` — px/s
- `MovementStatus { speed_multiplier, idling }`
- `Health(current, max)`
- `Team(u8)` — 0 = игрок
- `UnitKind(Scout | Soldier)`
- `Selected(bool)`

#### Scenario: Создание Scout

- **WHEN** `create_unit(handle, x, y, "Scout")`
- **THEN** Entity с BaseSpeed(140), Health(80, 80), UnitKind(Scout)

### Requirement: Ресурсы ECS

- `GameTime { delta }`
- `GridResource`, `BiomeDefinitions` — Resources в World

#### Scenario: GameTime обновляется

- **WHEN** `tick(handle, 0.016)`
- **THEN** `GameTime.delta == 0.016`

### Requirement: Movement system

Каждый tick movement_system SHALL:
1. Вычислить направление к MovementTarget
2. При расстоянии < 2px — idling, очистить target
3. Определить биом: `col = floor(pos.x / TILE_SIZE)`, `row = floor(pos.y / TILE_SIZE)`
4. Получить `speed_factor` из BiomeDefinitions по biome_ids
5. При speed_factor == 0 — не двигаться
6. Проверить проходимость новой позиции перед обновлением Position

#### Scenario: Движение по проходимому биому

- **WHEN** Scout на Plains (speed_factor=1.0), target далеко, tick(dt=1.0)
- **THEN** Position приблизился к цели на ~140 px

#### Scenario: Остановка перед Water

- **WHEN** новая позиция на непроходимом биоме (Water, Mountain, …)
- **THEN** idling = true, Position не меняется

#### Scenario: Биом на границе тайла

- **WHEN** юнит в (15.5, 15.5)
- **THEN** тайл col=0, row=0 (floor, не round)

### Requirement: Snapshot юнитов

`tick` SHALL возвращать массив с полями: id, x, y, health, maxHealth, unitType, team, selected, debug, waypoints.

### Requirement: Стартовые юниты

`spawn_starting_units` SHALL создать 2 Scout и 1 Soldier в центре карты на проходимом биоме.

#### Scenario: Спавн

- **WHEN** `spawn_starting_units(handle)` после create_grid
- **THEN** 3 Entity в центре, на проходимой клетке (если центр проходим)
