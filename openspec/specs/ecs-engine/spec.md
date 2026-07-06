## Purpose

ECS-движок на базе bevy_ecs для управления игровым состоянием.

## Requirements

### Requirement: World с Entity-компонентной моделью

Система SHALL использовать bevy_ecs World как хранилище игрового состояния. Добавлены компоненты юнитов. GridResource и BiomeDefinitions хранятся как bevy_ecs Resources (не как Entity).

#### Scenario: Создание World с юнитами

- **WHEN** вызывается `create_grid(seed, width, height)`
- **THEN** создаётся bevy_ecs World с GridResource и BiomeDefinitions как Resources

#### Scenario: Query клеток по биому

- **WHEN** система запрашивает GridResource
- **THEN** она получает плоский массив biome_ids, а не Entity

### Requirement: GridResource как bevy_ecs Resource

Система SHALL хранить `GridResource { width: u32, height: u32, biome_ids: Vec<u16>, resources: Vec<u8> }` как bevy_ecs Resource. Тип SHALL реализовывать `#[derive(Resource)]`.

#### Scenario: Доступ к GridResource из системы

- **WHEN** bevy_ecs система запрашивает Res<GridResource>
- **THEN** она получает размеры карты и массивы biome_ids/resources

**Context**: Координаты на карте измеряются в пикселях. Тайловая сетка имеет `TILE_SIZE = 32.0`. Конвертация пиксель→тайл использует `(x / TILE_SIZE).floor()`, гарантируя что пиксель `x ∈ [col*32, (col+1)*32)` попадает в тайл `col`.

#### Scenario: pixel_to_tile с .floor()

- **WHEN** пиксельная координата x = 31.9
- **WHEN** вызывается `pixel_to_tile(x, y)`
- **THEN** возвращается col = floor(31.9 / 32) = 0 (тайл, в котором находится пиксель)

#### Scenario: pixel_to_tile с .floor() на границе тайла

- **WHEN** пиксельная координата x = 32.0
- **WHEN** вызывается `pixel_to_tile(x, y)`
- **THEN** возвращается col = floor(32.0 / 32) = 1 (следующий тайл)

#### Scenario: pixel_to_tile с отрицательными координатами

- **WHEN** пиксельная координата x = -1.0
- **WHEN** вызывается `pixel_to_tile(x, y)`
- **THEN** возвращается col = 0 (без panic при касте f64→u32)

### Requirement: BiomeDefinitions как bevy_ecs Resource

Система SHALL хранить `BiomeDefinitions` как bevy_ecs Resource. Тип SHALL реализовывать `#[derive(Resource)]`.

#### Scenario: Доступ к BiomeDefinitions из системы

- **WHEN** bevy_ecs система запрашивает Res<BiomeDefinitions>
- **THEN** она получает доступ к определениям биомов (speed_factor, passable)

### Requirement: Выделение юнита

Система SHALL предоставить функцию `set_unit_selected(handle, unit_id, selected)` для изменения компонента Selected на Entity.

#### Scenario: Выделение через API

- **WHEN** вызывается `set_unit_selected(handle, 1, true)`
- **THEN** у Entity с id=1 компонент Selected = true

### Requirement: 8-directional A* pathfinding

Система SHALL предоставить A* pathfinding с 8 направлениями движения (прямые и диагонали). Алгоритм SHALL использовать Chebyshev distance как эвристику. Стоимость шага SHALL быть: прямой сосед = 1.0, диагональный сосед = √2 ≈ 1.414. Внутренние счётчики (f_score, g_score) SHALL быть f64.

#### Scenario: Путь по прямой без препятствий

- **WHEN** вызывается `find_path(grid, defs, (0, 0), (3, 3))` на пустой проходимой карте
- **THEN** возвращается путь `[(0,0), (1,1), (2,2), (3,3)]` (диагональ из 3 шагов, не 6 шагов как при 4-directional)

#### Scenario: Путь через одноклеточный проход (стенки вокруг)

- **WHEN** вызывается `find_path(grid, defs, (0,1), (2,1))` где тайлы (0,0), (0,2), (1,0), (1,2), (2,0), (2,2) непроходимы
- **THEN** возвращается путь `[(0,1), (1,1), (2,1)]` (прямая, без попытки пройти по диагонали через угол стенки)

### Requirement: Проверка проходимости диагонального шага (Diagonally-adjacent passability)

Система SHALL при диагональном шаге проверять проходимость ОБОИХ смежных прямых тайлов. Если хотя бы один из них непроходим, диагональный шаг SHALL быть отклонён (не добавляется в open set). Это предотвращает "прорезание углов" препятствий.

#### Scenario: Диагональ блокируется стенкой слева

- **WHEN** путь идёт из (0,1) в (1,2), и тайл (0,2) непроходим (но (1,1) проходим)
- **THEN** диагональный ход в (1,2) отклонён, A* выбирает обходной путь

#### Scenario: Все смежные проходимы — диагональ разрешена

- **WHEN** путь идёт из (0,0) в (1,1), и оба тайла (0,1) и (1,0) проходимы
- **THEN** диагональный ход в (1,1) разрешён