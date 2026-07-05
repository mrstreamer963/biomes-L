## Why

Игра — стратегия с командами юнитов, картой биомов и будущим геймплеем (combat, exploration, AI). Сейчас есть только генерация карты и рендер тайлов. Чтобы двигаться дальше, нужна основа для юнитов: ECS с bevy_ecs, real-time движение с учётом скорости биома, и базовая система управления.

## What Changes

- Добавление `bevy_ecs = "0.19"` в Rust-движок (WASM-крейт `engine`)
- Создание компонентов ECS: `Position`, `MovementTarget`, `BaseSpeed`, `MovementStatus`, `Health`, `Team`, `UnitKind`, `Selected`
- Создание ресурсов: `GameTime`, `GridResource` (уже есть — добавить `#[derive(Resource)]`), `BiomeDefinitions` (уже есть — добавить `#[derive(Resource)]`)
- Создание `movement_system`: движение юнита к цели с учётом скорости биома, остановка у непроходимых
- Расширение WASM API: `create_unit`, `set_unit_target`, `tick`, `spawn_starting_units`
- Разделение `GridStore`: каждый инстанс хранит ECS `World`
- На TypeScript стороне: гейм-луп в Worker, `UnitManager` для PixiJS рендера, хендлинг кликов/выделения
- При старте — спавн 3 юнитов в центре карты (на подходящем биоме)

## Capabilities

### New Capabilities

- `unit-management`: ECS-компоненты, системы движения, WASM API для создания и управления юнитами
- `unit-rendering`: PixiJS-рендер юнитов (кружки, health bars, выделение), хендлинг кликов (выбрать → клик по карте = движение)
- `game-loop`: Real-time цикл в Web Worker, tick() → physics → snapshot → rAF отрисовка

### Modified Capabilities

- `ecs-engine`: переименовать/переосмыслить — текущий `ecs/` содержит data-структуры (не ECS). Добавить настоящую ECS на bevy_ecs, но сохранить существующие типы как bevy_ecs Resources
- `wasm-worker`: расширить список сообщений: `tick`, `set-unit-target`, `unit-snapshot`
- `pixi-renderer`: рендер юнитов поверх тайлов, интерактив (click → select → move)
- `vue-shell`: добавить состояние выбранного юнита, обновление units ref

## Impact

- `engine/Cargo.toml` — новая зависимость `bevy_ecs = "0.19"`
- `engine/src/ecs/` — 2 новых файла, модификация существующих
- `engine/src/wasm_api.rs` — 4 новые функции, расширение GridStore
- `src/wasm/engine.d.ts` — новые типы (`UnitData`, функции)
- `src/bridge/wasm.ts` — новые типы сообщений
- `src/game/UnitManager.ts` — новый файл
- `src/game/MapRenderer.ts` — подключить UnitManager
- `src/worker/game.worker.ts` — game loop
- `src/composables/useGridSnapshot.ts` — units state