## MODIFIED Requirements

### Requirement: WASM функция greet

**FROM:**
Система SHALL предоставить функцию `greet()`, скомпилированную из Rust в WASM, возвращающую строку "Hello World from Rust!".

**TO:**
Система SHALL предоставить функцию `greet()`, скомпилированную из Rust в WASM, возвращающую строку "Hello World from Rust!". Эта функция остаётся без изменений для обратной совместимости.

### Requirement: WebWorker жизненный цикл

**FROM:**
Система SHALL создать WebWorker при старте Vue-приложения, загружающий WASM и вызывающий greet() автоматически.

**TO:**
Система SHALL создать WebWorker при старте Vue-приложения, загружающий WASM, регистрирующий дефолтные определения биомов через `register_biome_definitions` и создающий игровой мир.

#### Scenario: Worker инициализирует ECS World с конфигом биомов

- **WHEN** WebWorker инициализирован
- **THEN** он вызывает `register_biome_definitions(handle, defaultBiomes)` перед `create_grid()`

### Requirement: Bridge-прокси

**(без изменений)**

## ADDED Requirements

### Requirement: Конфиг биомов передаётся в worker

Система SHALL передавать массив определений биомов из main thread в worker при инициализации.

#### Scenario: Передача конфига

- **WHEN** main thread отправляет сообщение `{ type: "init", biomeDefinitions: [...] }`
- **THEN** worker вызывает `register_biome_definitions(handle, biomeDefinitions)` и затем `create_grid(seed, w, h)`
