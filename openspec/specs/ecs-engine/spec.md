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
