## Purpose

Выполнение Rust-кода в браузере через WASM и WebWorker.

## Requirements

### Requirement: Init сообщение

Система SHALL поддерживать инициализацию worker через:
- Main → Worker: `{ type: "init", biomeDefinitions, generationParams }`

Worker SHALL:
1. Загрузить WASM (`init()`)
2. Вызвать `create_grid(generationParams, width, height)` — генерация по правилам из встроенного `biomes.json`
3. Вызвать `register_biome_definitions(handle, biomeDefinitions)` — свойства для pathfinding/рендера
4. Вызвать `grid_snapshot(handle)` и отправить snapshot
5. Вызвать `spawn_starting_units(handle)`

#### Scenario: Worker инициализирует ECS

- **WHEN** WebWorker получает `init` с дефолтными данными из `biomeConfig.ts`
- **THEN** карта создана, биомы зарегистрированы, юниты заспавнены

#### Scenario: Worker отправляет snapshot сетки

- **WHEN** инициализация завершена
- **THEN** worker отправляет `{ type: "grid-snapshot", width, height, biomeIds, resources }`

### Requirement: Tick сообщение

- Main → Worker: `{ type: "tick", dt: number }`
- Worker → Main: `{ type: "unit-snapshot", units: UnitData[] }`

#### Scenario: Tick через bridge

- **WHEN** main thread отправляет `{ type: "tick", dt: 0.016 }`
- **THEN** WASM вызывает `tick(handle, 0.016)` и worker отправляет unit-snapshot

### Requirement: set-unit-target сообщение

- Main → Worker: `{ type: "set-unit-target", unitId, x, y }`

#### Scenario: Установка цели

- **WHEN** main thread отправляет `{ type: "set-unit-target", unitId: 1, x: 500, y: 300 }`
- **THEN** WASM вызывает `set_unit_target(handle, 1, 500.0, 300.0)`

### Requirement: set-unit-debug сообщение

- Main → Worker: `{ type: "set-unit-debug", unitId, debug: boolean }`

### Requirement: Bridge-прокси

Система SHALL предоставить типизированную обёртку `bridge/wasm.ts` для сообщений: `init`, `tick`, `grid-snapshot`, `unit-snapshot`, `set-unit-target`, `set-unit-debug`.

#### Scenario: Получение grid-snapshot

- **WHEN** worker отправляет grid-snapshot
- **THEN** `biomeIds` доставляется как `Uint16Array`, `resources` как `Uint8Array`

### Requirement: Worker не запускает свой game loop

Worker SHALL обрабатывать `tick` только по запросу из main thread (requestAnimationFrame на UI-треде).

#### Scenario: Worker ждёт tick

- **WHEN** worker инициализирован и status = "ready"
- **THEN** worker не вызывает `tick` самостоятельно, ожидает сообщения от main thread
