## MODIFIED Requirements

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