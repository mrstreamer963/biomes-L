## Purpose

Обеспечить выполнение Rust-кода в браузере через WASM и WebWorker. TBD — цели будут уточнены в процессе разработки.

## Requirements

### Requirement: WASM функция greet
Система SHALL предоставить функцию `greet()`, скомпилированную из Rust в WASM, возвращающую строку "Hello World from Rust!". Эта функция остаётся без изменений для обратной совместимости.

### Requirement: WebWorker жизненный цикл
Система SHALL создать WebWorker при старте Vue-приложения, загружающий WASM, регистрирующий дефолтные определения биомов через `register_biome_definitions` и создающий игровой мир.

#### Scenario: Worker инициализирует ECS World с конфигом биомов
- **WHEN** WebWorker инициализирован
- **THEN** он вызывает `register_biome_definitions(handle, defaultBiomes)` перед `create_grid()`

#### Scenario: Worker отправляет snapshot сетки
- **WHEN** WebWorker инициализирован и вызвал `create_grid` + `grid_snapshot`
- **THEN** worker отправляет main thread'у сообщение `{ type: "grid-snapshot", width, height, biomes }`, где `biomes` — массив индексов биомов

#### Scenario: Worker сообщает об ошибке движка
- **WHEN** инициализация WASM или генерация сетки завершается ошибкой
- **THEN** worker отправляет сообщение `{ type: "status", status: "error", error }`

### Requirement: Конфиг биомов передаётся в worker
Система SHALL передавать массив определений биомов из main thread в worker при инициализации.

#### Scenario: Передача конфига
- **WHEN** main thread отправляет сообщение `{ type: "init", biomeDefinitions: [...] }`
- **THEN** worker вызывает `register_biome_definitions(handle, biomeDefinitions)` и затем `create_grid(seed, w, h)`

### Requirement: Bridge-прокси
Система SHALL предоставить типизированную обёртку `bridge/wasm.ts` для отправки и получения сообщений между main thread и worker, включая новое сообщение `grid-snapshot`.

#### Scenario: Отправка сообщения через bridge
- **WHEN** main thread отправляет сообщение через bridge
- **THEN** worker получает его и обрабатывает

#### Scenario: Получение snapshot сетки через bridge
- **WHEN** worker отправляет сообщение `grid-snapshot`
- **THEN** bridge доставляет его в main thread с типизированными полями `width`, `height`, `biomes`
