## Purpose

ECS-компоненты и WASM API для управления юнитами в игре.

## Requirements

### Requirement: Unit ECS компоненты

Система SHALL определить следующие bevy_ecs компоненты для юнитов:
- `Position { x: f64, y: f64 }` — координаты в пикселях на карте
- `MovementTarget(Option<(f64, f64)>)` — целевая точка (пиксели), None = стоит
- `BaseSpeed(f64)` — базовая скорость в px/s
- `MovementStatus { speed_multiplier: f64, idling: bool }` — состояние движения
- `Health(u32, u32)` — текущее и максимальное HP
- `Team(u8)` — 0 = игрок
- `UnitKind(Scout | Soldier)` — тип юнита
- `Selected(bool)` — выделен игроком

#### Scenario: Создание юнита с компонентами

- **WHEN** вызывается `create_unit(handle, x, y, "Scout")`
- **THEN** в ECS World создаётся Entity с Position, MovementTarget(None), BaseSpeed(140), MovementStatus { multiplier: 1.0, idling: true }, Health(80, 80), Team(0), UnitKind(Scout), Selected(false)

### Requirement: Ресурсы ECS

Система SHALL предоставить bevy_ecs Resource:
- `GameTime { delta: f64 }` — время в секундах с последнего tick
- `GridResource` и `BiomeDefinitions` (существующие типы) SHALL быть помечены `#[derive(Resource)]` и храниться в World

#### Scenario: GameTime обновляется каждый tick

- **WHEN** вызывается `tick(handle, 0.016)`
- **THEN** `GameTime.delta` в World равно 0.016

### Requirement: Movement system

Система SHALL запускать movement_system каждый tick. Система SHALL для каждого юнита с MovementTarget(Some(target)):
1. Вычислить направление и расстояние до цели
2. Если расстояние < 2.0 px → установить MovementStatus.idling = true, очистить Target
3. Определить биом под текущей позицией юнита (col = pos.x/32, row = pos.y/32)
4. Получить speed_factor из BiomeDefinitions
5. Если speed_factor == 0 (непроходимый биом) → установить idling = true, не двигаться
6. Иначе вычислить шаг = base_speed * speed_factor * dt
7. Проверить, что новая позиция не на непроходимом биоме
8. Если новая позиция проходима — обновить Position

#### Scenario: Юнит движется к цели по Plains

- **WHEN** юнит Scout (BaseSpeed=140) на Plains (speed_factor=1.0) имеет MovementTarget(Some(100, 100)) и Position(50, 50)
- **WHEN** tick(handle, 1.0) вызван
- **THEN** Position приблизился к (100, 100) на 140 px

#### Scenario: Юнит останавливается перед Water

- **WHEN** юнит движется к цели за Water
- **WHEN** новая позиция попадает на Water (passable=false)
- **THEN** MovementStatus.idling = true, Position не меняется

#### Scenario: Юнит останавливается у цели

- **WHEN** юнит в 1 px от MovementTarget
- **WHEN** tick(handle, dt) вызван
- **THEN** MovementTarget = None, MovementStatus.idling = true

### Requirement: Snapshot юнитов

Система SHALL предоставить функцию, собирающую snapshot всех юнитов из World для передачи в JS.

#### Scenario: Snapshot после tick

- **WHEN** tick(handle, dt) вызван
- **THEN** возвращается массив объектов с полями: id, x, y, health, maxHealth, unitType, team, selected

### Requirement: Стартовые юниты

Система SHALL спавнить 3 юнитов при старте: 2 Scout и 1 Soldier. Юниты SHALL появиться в центре карты на Plains (если центральная клетка проходима).

#### Scenario: Спавн стартовых юнитов

- **WHEN** `spawn_starting_units(handle)` вызван после создания грида
- **THEN** в World есть 3 Entity с UnitKind, и их позиции находятся в центре карты на проходимом биоме