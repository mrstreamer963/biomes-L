## 1. Engine Dependency Setup

- [x] 1.1 Add `bevy_ecs = { version = "0.19", default-features = false }` to `engine/Cargo.toml`
- [x] 1.2 Verify `cargo build` works (engine/ directory)

## 2. Rust ECS Components & Resources

- [x] 2.1 Create `engine/src/ecs/components.rs` with all unit components
- [x] 2.2 Add `#[derive(Resource)]` to `GridResource` in `engine/src/ecs/grid_resource.rs`
- [x] 2.3 Add `#[derive(Resource)]` to `BiomeDefinitions` in `engine/src/ecs/biome_definitions.rs`
- [x] 2.4 Add `GameTime` resource struct in `engine/src/ecs/mod.rs`
- [x] 2.5 Register new modules (`components`, `systems`) in `engine/src/ecs/mod.rs`
- [x] 2.6 Create `engine/src/ecs/systems.rs` with `movement_system`

## 3. Rust ECS World Integration & WASM API

- [x] 3.1 Add ECS `World` and `HashMap<u32, Entity>` to `GridStore` in `wasm_api.rs`
- [x] 3.2 Add `NEXT_UNIT_ID` AtomicU32 counter
- [x] 3.3 Implement `create_unit(handle, x, y, unit_type) -> u32`
- [x] 3.4 Implement `set_unit_target(handle, unit_id, x, y)`
- [x] 3.5 Implement `set_unit_selected(handle, unit_id, selected)`
- [x] 3.6 Implement `tick(handle, dt) -> JsValue` (run systems + snapshot)
- [x] 3.7 Implement `spawn_starting_units(handle)` — 2 Scouts + 1 Soldier в центре карты
- [x] 3.8 Implement `unit_count(handle) -> u32`
- [x] 3.9 Build and verify WASM compiles: `wasm-pack build engine/`

## 4. TypeScript WASM Type Definitions

- [x] 4.1 Add `UnitData` interface to `src/wasm/engine.d.ts`
- [x] 4.2 Add function signatures: `create_unit`, `set_unit_target`, `set_unit_selected`, `tick`, `spawn_starting_units`, `unit_count`

## 5. Worker Bridge & Messages

- [x] 5.1 Add `UnitData` type and tick/set-unit-target/unit-snapshot messages to `src/bridge/wasm.ts`

## 6. Worker Game Loop

- [x] 6.1 Update `src/worker/game.worker.ts`:
  - After init: call `spawn_starting_units(handle)`
  - Handle `tick` message: call `tick(handle, dt)`, postMessage `unit-snapshot`
  - Handle `set-unit-target` message: call `set_unit_target(handle, unitId, x, y)`

## 7. Composables

- [x] 7.1 Add `units` reactive ref to `src/composables/useGridSnapshot.ts`
- [x] 7.2 Handle `unit-snapshot` message in message handler
- [x] 7.3 Start rAF game loop when worker status is "ready"
- [x] 7.4 Stop game loop and cleanup on unmount

## 8. PixiJS Unit Manager

- [x] 8.1 Create `src/game/UnitManager.ts`:
  - Container with Graphics for circles, health bars, selection rings
  - `update(units: UnitData[])` method
  - `hitTest(worldX, worldY) -> {unitId} | null`
  - Selection state management

## 9. MapRenderer Integration

- [x] 9.1 Update `src/game/MapRenderer.ts` to import UnitManager
- [x] 9.2 Create UnitManager instance and add its container to worldContainer
- [x] 9.3 Watch `unitData` and call `unitManager.update()`
- [x] 9.4 Add pointer event listener for unit selection and movement commands

## 10. Build & Verify

- [x] 10.1 Run `wasm-pack build engine/` and ensure no errors
- [x] 10.2 Run `npm run dev` and verify game loads
- [x] 10.3 Verify 3 units visible on the map at start
- [x] 10.4 Click a unit → highlight appears
- [x] 10.5 Click on a tile → unit moves there with correct biome speed
- [x] 10.6 Try moving into Water → unit stops at the edge
- [x] 10.7 Health bars visible above units