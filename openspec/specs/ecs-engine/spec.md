## Purpose

ECS-движок на базе bevy_ecs для управления игровым состоянием: карта, биомы, юниты, pathfinding.

## Requirements

### Requirement: World с Entity-компонентной моделью

Система SHALL использовать bevy_ecs World. Юниты — Entity с компонентами. `GridResource` и `BiomeDefinitions` хранятся как Resources (не Entity).

#### Scenario: Создание World

- **WHEN** вызывается `create_grid(params, width, height)` и `register_biome_definitions(handle, defs)`
- **THEN** World содержит GridResource и BiomeDefinitions как Resources

#### Scenario: Query клеток по биому

- **WHEN** система запрашивает GridResource
- **THEN** она получает плоский массив `biome_ids: Vec<u16>`

### Requirement: GridResource как bevy_ecs Resource

Система SHALL хранить `GridResource { width: u32, height: u32, biome_ids: Vec<u16>, resources: Vec<u8> }` как `#[derive(Resource)]`.

#### Scenario: Доступ к GridResource из системы

- **WHEN** система запрашивает `Res<GridResource>`
- **THEN** она получает размеры карты и массивы biome_ids/resources

**Context**: Координаты на карте в пикселях. `TILE_SIZE = 32`. Конвертация: `(x / TILE_SIZE).floor()`.

#### Scenario: pixel_to_tile с .floor()

- **WHEN** x = 31.9, вызывается `pixel_to_tile(x, y)`
- **THEN** col = 0

#### Scenario: pixel_to_tile на границе тайла

- **WHEN** x = 32.0, вызывается `pixel_to_tile(x, y)`
- **THEN** col = 1

### Requirement: BiomeDefinitions как bevy_ecs Resource

Система SHALL хранить `BiomeDefinitions` как Resource с методами доступа по id и имени.

#### Scenario: Доступ из movement system

- **WHEN** movement_system запрашивает `Res<BiomeDefinitions>`
- **THEN** она получает `speed_factor` и `is_passable` для BiomeId под юнитом

### Requirement: Выделение юнита

Система SHALL предоставить `set_unit_selected(handle, unit_id, selected)`.

#### Scenario: Выделение через API

- **WHEN** вызывается `set_unit_selected(handle, 1, true)`
- **THEN** у Entity с id=1 компонент Selected = true

### Requirement: 8-directional A* pathfinding

Система SHALL предоставить A* с 8 направлениями, Chebyshev-эвристикой, стоимостью прямого шага 1.0 и диагонального √2.

#### Scenario: Путь по диагонали

- **WHEN** `find_path(grid, defs, (0, 0), (3, 3))` на пустой проходимой карте
- **THEN** путь короче 6 шагов (не 4-directional)

### Requirement: Проверка проходимости диагонального шага

При диагональном шаге система SHALL проверять проходимость обоих смежных прямых тайлов.

#### Scenario: Диагональ блокируется стенкой

- **WHEN** путь из (0,1) в (1,2), тайл (0,2) непроходим
- **THEN** диагональный ход отклонён

#### Scenario: Диагональ разрешена

- **WHEN** путь из (0,0) в (1,1), тайлы (0,1) и (1,0) проходимы
- **THEN** диагональный ход разрешён

### Requirement: Funnel Algorithm для пост-процессинга A* пути

Система SHALL предоставить функцию `funnel_algorithm(cells, start, end) -> Vec<(f64, f64)>`, которая принимает путь A* как последовательность смежных клеток и возвращает waypoint'ы на вершинах тайлов (свободное позиционирование, не центры тайлов).

#### Scenario: Прямой коридор без препятствий

- **WHEN** A* вернул путь `[(0,0), (1,0), (2,0), (3,0)]` (прямая вправо по строке 0)
- **WHEN** start = (16, 16), end = (100, 16)
- **THEN** `funnel_algorithm` возвращает `[(16, 16), (100, 16)]` — без дополнительных waypoint'ов, т.к. порталы не сужают воронку

#### Scenario: Путь с поворотом

- **WHEN** A* вернул путь `[(0,0), (0,1), (0,2)]` (вниз по колонке 0)
- **WHEN** start = (16, 16), end = (16, 80)
- **THEN** `funnel_algorithm` возвращает `[(16, 16), (16, 80)]` — только start и end, без дополнительных waypoint'ов (прямой путь)

#### Scenario: L-образный путь (один поворот)

- **WHEN** A* вернул путь `[(0,0), (1,0), (1,1)]` (вправо, затем вниз)
- **WHEN** start = (16, 16), end = (48, 48)
- **THEN** `funnel_algorithm` возвращает `[(16, 16), (32, 32), (48, 48)]` — waypoint на общем угле тайлов (32, 32)

#### Scenario: S-образный путь (два поворота)

- **WHEN** A* вернул путь с двумя поворотами
- **THEN** `funnel_algorithm` возвращает waypoint'ы на вершинах, где воронка сужается

#### Scenario: Путь из 1 клетки

- **WHEN** клеток в пути ≤ 1
- **THEN** `funnel_algorithm` возвращает `[end]` (без пост-процессинга)

#### Scenario: Путь из 2 клеток

- **WHEN** клеток в пути = 2
- **THEN** `funnel_algorithm` возвращает `[start, end]` (без пост-процессинга)

#### Scenario: Диагональный путь

- **WHEN** A* вернул диагональный путь `[(0,0), (1,1), (2,2)]`
- **WHEN** start = (16, 16), end = (80, 80)
- **THEN** `funnel_algorithm` возвращает `[(16, 16), (80, 80)]` — диагональные порталы — одна точка, воронка не сужается
