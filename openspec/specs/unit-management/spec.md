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

### Requirement: Funnel Algorithm для генерации waypoint'ов в set_unit_target

Система SHALL в функции `set_unit_target` после A* применять `funnel_algorithm` для генерации waypoint'ов. Waypoint'ы SHALL быть свободными (на вершинах тайлов). `MovementTarget` SHALL устанавливаться на точную позицию клика `(x, y)`, а не на центр тайла.

#### Scenario: Путь через несколько клеток

- **WHEN** юнит в позиции (16, 16) получает команду движения в (55, 55)
- **WHEN** A* вернул путь `[(0,0), (1,0), (1,1)]`
- **WHEN** `set_unit_target` вызван
- **THEN** `Path` компонент содержит waypoint'ы на вершинах тайлов (не в центрах), например `[(32, 32), (55, 55)]`
- **THEN** `MovementTarget` = `Some((55, 55))`

#### Scenario: Прямой путь, MovementTarget = позиция клика

- **WHEN** юнит в позиции (16, 16) получает команду движения в (90, 16)
- **WHEN** A* вернул прямой путь
- **THEN** `MovementTarget` = `Some((90, 16))` (точная позиция клика)

#### Scenario: Путь из одной клетки

- **WHEN** юнит кликает в ту же клетку, где стоит
- **WHEN** A* вернул путь из 1 клетки
- **THEN** waypoints = `[(x, y)]` — без пост-процессинга
- **THEN** `MovementTarget` = `Some((x, y))`

### Requirement: Нет изменений в movement_system

Система SHALL НЕ изменять `movement_system`. Система продолжает обрабатывать waypoint'ы из `Path` и `MovementTarget` как `(f64, f64)` в пикселях.

#### Scenario: Движение по свободным waypoint'ам

- **WHEN** `Path` содержит waypoint'ы на вершинах тайлов (например, (32, 32))
- **WHEN** `movement_system` обрабатывает юнит
- **THEN** юнит движется lerp-ом к (32, 32), затем к `MovementTarget`
