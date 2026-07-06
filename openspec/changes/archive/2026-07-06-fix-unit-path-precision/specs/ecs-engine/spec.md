## MODIFIED Requirements

### Requirement: GridResource как bevy_ecs Resource

Система SHALL хранить `GridResource { width: u32, height: u32, biome_ids: Vec<u16>, resources: Vec<u8> }` как bevy_ecs Resource. Тип SHALL реализовывать `#[derive(Resource)]`.

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
