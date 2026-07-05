## Context

Текущая архитектура: Vue + PixiJS (рендер) ↔ Web Worker ↔ WASM (Rust engine). В WASM-крейте `engine` есть генерация карты (noise, биомы) и data-структуры для хранения грида. Папка `ecs/` названа исторически — там просто data-модули, не ECS. Нужно добавить юниты с перспективой роста: скоро появятся combat, AI, status effects, отряды. Чтобы не переписывать каждый раз, кладём фундамент сразу на bevy_ecs.

### Ключевые ограничения

- Всё в WASM → bevy_ecs должен работать в wasm32-unknown-unknown
- Real-time, не пошагово
- Только один игрок (пока)
- Рендер — PixiJS на UI-треде, физика/ECS — в Worker на WASM
- Юниты двигаются по карте непрерывно (f64), не прыгают по клеткам

## Data Flow

```
requestAnimationFrame (UI thread)
  │
  │  performance.now() → dt
  │
  ▼
postMessage({ type: "tick", dt }) ──────────────────▶ Worker
                                                        │
                                                        ▼
                                                      WASM: tick(handle, dt)
                                                        │
                                                        ├─ world.run_systems()
                                                        │  └─ movement_system
                                                        │     └─ biome_lookup(query pos → grid cell → speed)
                                                        │
                                                        └─ snapshot_units()
                                                           └─ Vec<{ id, x, y, hp, max_hp, type, team, selected }>
                                                        │
                                                        ▼
                                                      postMessage({ type: "unit-snapshot", units })
  ◀───────────────────────────────────────────────────
  │
  ▼
UnitManager.update(units)
  │
  ├─ update PixiJS graphics (circles, health bars, selection ring)
  └─ update reactive state (current selection, unit count)

click on canvas
  │
  ├─ hit test (reverse order, last unit on top)
  │   ├─ hit on unit → select(id)
  │   └─ hit on empty cell + has selected → postMessage({ type: "set-unit-target", unitId, x, y })
  │
  └─ click on another unit → switch selection
```

## ECS World Layout

```
World
├── Resource: GridResource { width, height, biome_ids, resources }
├── Resource: BiomeDefinitions { definitions: Vec<BiomeDef> }
├── Resource: GameTime { delta: f64 }
│
├── Entity: unit-1 (id=1)
│   ├── Position { x: 4096.0, y: 4096.0 }
│   ├── MovementTarget(Some(8192.0, 4096.0))
│   ├── BaseSpeed(120.0)
│   ├── MovementStatus { multiplier: 1.0, idling: false }
│   ├── Health(100, 100)
│   ├── Team(0)
│   ├── UnitKind(Scout)
│   └── Selected(false)
│
├── Entity: unit-2 (id=2) ... (Solder)
└── Entity: unit-3 (id=3) ... (Scout)
```

## Decisions

### Decision 1: bevy_ecs 0.19 standalone без bevy
- **Варианты**: (a) bevy_ecs standalone, (b) свой UnitStore, (c) legion-ecs, (d) shipyard
- **Выбор**: bevy_ecs standalone
- **Почему**: bevy_ecs — самая популярная ECS в Rust, активно поддерживается, из коробки работает в WASM. Query API удобен для фильтрации (ByChange, With, Without). Resource injection — идеально для GridResource/BiomeDefinitions. В перспективе (combat, AI, effects) ECS окупится. Свой store пришлось бы переписывать.

### Decision 2: ID = Entity + AtomicU32
- У bevy_ecs Entity — это {generation, index}. Для передачи через WASM boundary (JS) нужен плоский u32.
- Храним `HashMap<u32, Entity>` на Rust стороне. `NEXT_UNIT_ID` — AtomicU32.
- JS оперирует только u32-идентификаторами.

### Decision 3: tick() вместо continuous system
- `tick()` вызывается из JS worker'а с dt (секунды). Внутри:
  1. Обновить `GameTime.resource_mut().delta = dt`
  2. `world.run_systems()` — выполняет movement_system
  3. Собрать snapshot всех юнитов
  4. Вернуть snapshot как JsValue
- Не делаем бесконечный loop в WASM — JS worker контролирует тайминг.

### Decision 4: Выделение на JS, не на Rust
- `Selected(bool)` — компонент в ECS, но меняется из JS (через `set_unit_selected(handle, id, bool)`).
- Hit test — в PixiJS (Graphics их не поддерживает hitTest, будем считать вручную: расстояние от точки клика до центра юнита).
- Rust не занимается UI-логикой.

### Decision 5: Размеры и координаты
- Карта: `width * 32` x `height * 32` пикселей (как сейчас)
- Позиция юнита — f64 пиксели от левого-верхнего угла карты
- Для определения биома под юнитом: `col = (pos.x / 32.0).round() as u32`, `row = (pos.y / 32.0).round() as u32`
- Диаметр юнита: ~24px (Scout = 20px, Soldier = 26px)
- BaseSpeed: Soldier = 80 px/s, Scout = 140 px/s

### Decision 6: Компонентное представление MovementStatus
- Вместо одного enum `MovementStatus` используем:
  - `MovementStatus` с `speed_multiplier: f64` и `idling: bool` — состояние
  - Если `idling == true` — юнит не двигается (независимо от Target)
  - Если биом под ногами непроходим — `idling = true` (Stuck)
- Проще, чем enum (не нужно match во всех системах)

## ECS Components Summary

```rust
#[derive(Component)]
pub struct Position { pub x: f64, pub y: f64 }

#[derive(Component)]
pub struct MovementTarget(pub Option<(f64, f64)>);

#[derive(Component)]
pub struct BaseSpeed(pub f64);

#[derive(Component, Default)]
pub struct MovementStatus {
    pub speed_multiplier: f64,  // модификатор от биома
    pub idling: bool,           // true = стоит (arrived или stuck)
}

#[derive(Component)]
pub struct Health(pub u32, pub u32);  // current, max

#[derive(Component)]
pub struct Team(pub u8);  // 0 = игрок

#[derive(Component)]
pub enum UnitKind { Scout, Soldier }

#[derive(Component)]
pub struct Selected(pub bool);
```

## movement_system

```rust
fn movement_system(
    time: Res<GameTime>,
    grid: Res<GridResource>,
    defs: Res<BiomeDefinitions>,
    mut query: Query<(&mut Position, &MovementTarget, &BaseSpeed, &mut MovementStatus)>,
) {
    let dt = time.delta;
    for (mut pos, target, speed, mut status) in query.iter_mut() {
        let (tx, ty) = match &target.0 {
            Some(t) => *t,
            None => continue,
        };
        let dx = tx - pos.x;
        let dy = ty - pos.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < 2.0 {
            status.idling = true;
            continue;
        }
        // Определяем биом под юнитом
        let col = (pos.x / 32.0).round() as u32;
        let row = (pos.y / 32.0).round() as u32;
        let idx = (row as usize) * (grid.width as usize) + (col as usize);
        let biome_id = grid.biome_ids.get(idx).copied().unwrap_or(0);
        let speed_factor = defs.definitions.get(biome_id as usize)
            .map(|d| d.speed_factor).unwrap_or(1.0);
        if speed_factor == 0.0 {
            // Непроходимый биом — стоим
            status.idling = true;
            continue;
        }
        status.speed_multiplier = speed_factor;
        status.idling = false;
        let step = speed.0 * speed_factor * dt;
        let nx = pos.x + (dx / dist) * step;
        let ny = pos.y + (dy / dist) * step;
        // Проверяем новый биом
        let ncol = (nx / 32.0).round() as u32;
        let nrow = (ny / 32.0).round() as u32;
        let nidx = (nrow as usize) * (grid.width as usize) + (ncol as usize);
        let nbiome = grid.biome_ids.get(nidx).copied().unwrap_or(0);
        let npassable = defs.definitions.get(nbiome as usize)
            .map(|d| d.passable).unwrap_or(false);
        if !npassable {
            status.idling = true;
            continue;
        }
        pos.x = nx;
        pos.y = ny;
    }
}
```

## UnitManager (PixiJS)

- Хранит `Container` с Graphics для всех юнитов
- `update(units: UnitData[])` — перерисовывает все graphics
- Каждый юнит: круг с цветом команды, health bar над ним, рамка выделения
- `hitTest(x, y)` → `{ unitId } | null` — проверка расстояния до центра каждого юнита (reverse order)

## WASM API

```rust
// Новые функции
pub fn create_unit(handle: u32, x: f64, y: f64, unit_type: &str) -> u32;
pub fn set_unit_target(handle: u32, unit_id: u32, x: f64, y: f64);
pub fn set_unit_selected(handle: u32, unit_id: u32, selected: bool);
pub fn tick(handle: u32, dt: f64) -> JsValue;  // → Vec<UnitSnapshot>
pub fn spawn_starting_units(handle: u32);
pub fn unit_count(handle: u32) -> u32;

// Snapshot структура
// { id: u32, x: f64, y: f64, health: u32, maxHealth: u32, unitType: String, team: u8, selected: bool }
```

## Risks / Trade-offs

- **WASM размер**: bevy_ecs добавит ~200-400KB к бандлу. Для игры это приемлемо.
- **Performance**: movement_system на 5000+ юнитов может быть узким. Пока неактуально (< 100 юнитов). В будущем — change query filter, par_iter.
- **Границы карты**: юниты могут уйти за край карты — нужно clamp в movement_system.
- **Синхронизация dt**: rAF на UI-треде ≠ время в Worker. Используем `performance.now()` на UI-треде и передаём dt в Worker. Это не критично для одного игрока.
- **hit test в PixiJS**: PixiJS Graphics не имеет встроенного hitTest. Ручная проверка по расстоянию — O(n). Для 50+ юнитов можно SpatialHash, но пока не нужно.

## Open Questions

- Типы юнитов для старта: Scout (быстрый, 80 HP, 20px) и Soldier (медленный, 150 HP, 26px) — OK?
- Цвета команд: игрок = зелёный, будущие AI = красный?
- Что делать, если юнит стартует на непроходимом биоме? spawn_starting_units ищет Plains в центре карты.