## ADDED Requirements

### Requirement: Tick сообщение bridge

Система SHALL поддерживать новое сообщение `tick` между main thread и worker:

- Main → Worker: `{ type: "tick", dt: number }`
- Worker → Main: `{ type: "unit-snapshot", units: UnitData[] }`

#### Scenario: Tick через bridge

- **WHEN** main thread отправляет `{ type: "tick", dt: 0.016 }`
- **THEN** WASM вызывает `tick(handle, 0.016)`, возвращает snapshot юнитов
- **THEN** worker отправляет `{ type: "unit-snapshot", units: [...] }`

### Requirement: set-unit-target сообщение

Система SHALL поддерживать сообщение `set-unit-target`:

- Main → Worker: `{ type: "set-unit-target", unitId: number, x: number, y: number }`

#### Scenario: Установка цели через bridge

- **WHEN** main thread отправляет `{ type: "set-unit-target", unitId: 1, x: 500, y: 300 }`
- **THEN** WASM вызывает `set_unit_target(handle, 1, 500.0, 300.0)`

### Requirement: Worker game loop

Worker SHALL после инициализации:
1. Вызвать `spawn_starting_units(handle)`
2. Запустить game loop, получающий tick из main thread

#### Scenario: Worker ждёт tick

- **WHEN** worker инициализирован и готов
- **THEN** worker не запускает свой цикл, а ожидает tick сообщения от main thread

## MODIFIED Requirements

### Requirement: WebWorker жизненный цикл

Система SHALL создать WebWorker при старте Vue-приложения, загружающий WASM, регистрирующий определения биомов, создающий World, спавнящий стартовые юниты и ожидающий tick сообщений.

#### Scenario: Worker инициализирует ECS World с юнитами

- **WHEN** WebWorker инициализирован
- **THEN** он вызывает `register_biome_definitions(handle, defaultBiomes)`, `create_grid()`, затем `spawn_starting_units(handle)`

#### Scenario: Worker отправляет snapshot сетки

- **WHEN** WebWorker инициализирован и вызвал `create_grid` + `grid_snapshot`
- **THEN** worker отправляет `{ type: "grid-snapshot", width, height, biomeIds, resources }`

### Requirement: Bridge-прокси

Система SHALL предоставить типизированную обёртку `bridge/wasm.ts` для отправки и получения сообщений между main thread и worker, включая сообщения `grid-snapshot`, `unit-snapshot`, `tick`, `set-unit-target`.

#### Scenario: Отправка tick через bridge

- **WHEN** main thread вызывает `bridge.postMessage({ type: "tick", dt })`
- **THEN** worker получает сообщение и обрабатывает

#### Scenario: Получение unit-snapshot через bridge

- **WHEN** worker отправляет `{ type: "unit-snapshot", units }`
- **THEN** bridge доставляет его в main thread с типизированными полями