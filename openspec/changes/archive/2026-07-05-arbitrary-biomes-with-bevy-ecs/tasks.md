## 1. Rust: bevy_ecs dependency и базовая структура

- [x] 1.1 Добавить `bevy-ecs = { version = "0.14", default-features = false }` в engine/Cargo.toml
- [x] 1.2 Создать `engine/src/ecs/` модуль с компонентами (Position, BiomeId, Resources)
- [x] 1.3 Создать `engine/src/ecs/grid_resource.rs` с GridResource (width, height, entities, rng)
- [x] 1.4 Создать `engine/src/ecs/biome_definitions.rs` с BiomeDefinitions Resource
- [x] 1.5 Экспортировать модули из `lib.rs`

## 2. Rust: Система генерации карты на ECS

- [x] 2.1 Написать ECS-систему `generate_biomes_system`, заменяющую `Grid::generate()` — итеративно спавнит Entity с Position, BiomeId, Resources
- [x] 2.2 Убедиться, что RNG (Lcg) сохраняет детерминизм в новом pipeline
- [x] 2.3 Написать тесты на генерацию: seed → ожидаемое распределение биомов

## 3. Rust: BiomeDefinitions Resource и методы

- [x] 3.1 Реализовать BiomeDefinitions: хранение Vec<BiomeDef>, методы get_name, is_passable, speed_factor, get_color
- [x] 3.2 Реализовать BiomeDef: name, passable, speed_factor, color
- [x] 3.3 Написать тесты на BiomeDefinitions

## 4. Rust: WASM API — реестр биомов

- [x] 4.1 Добавить `register_biome_definitions(handle: u32, definitions: JsValue) -> bool` в wasm_api.rs
- [x] 4.2 Парсинг массива JS-объектов в Vec<BiomeDef> через serde-wasm-bindgen (или вручную)
- [x] 4.3 Сохранять BiomeDefinitions в World как Resource

## 5. Rust: WASM API — ECS snapshot

- [x] 5.1 Переписать `grid_snapshot()`: итерировать GridResource.entities, собирать компоненты biome_id и resources
- [x] 5.2 Сформировать wire-формат (BiomeId + Resources на клетку) — используем два Uint8Array или структурированный массив
- [x] 5.3 Добавить `biome_definitions(handle: u32) -> JsValue` — экспорт определений биомов в TS

## 6. Rust: Удаление старого кода

- [x] 6.1 Удалить `engine/src/biome.rs` (enum Biome)
- [x] 6.2 Удалить `engine/src/grid.rs` (старый Vec<Cell>)
- [x] 6.3 Обновить тесты: убрать `mod biome` и `mod grid`, переписать на ECS-тесты

## 7. TS: Конфиг биомов

- [x] 7.1 Создать `src/config/biomeConfig.ts` с `DEFAULT_BIOME_DEFINITIONS` — массив из 4 определений (Plains, Forest, Water, Mountain)
- [x] 7.2 Экспортировать `BiomeDefinition` type (name, passable, speed, color)

## 8. TS: Обновление types (engine.d.ts)

- [x] 8.1 Добавить `BiomeDefinition` в engine.d.ts
- [x] 8.2 Обновить `GridSnapshot`: biomes меняется на biomeIds: Uint16Array + resources: Uint8Array (или структурированный объект)
- [x] 8.3 Добавить функцию `register_biome_definitions(handle: number, definitions: BiomeDefinition[]): boolean` в .d.ts

## 9. TS: Worker bridge

- [x] 9.1 Обновить WorkerMessage: init принимает biomeDefinitions
- [x] 9.2 Обновить game.worker.ts: при инициализации вызывать register_biome_definitions
- [x] 9.3 Обновить bridge/wasm.ts под новые типы сообщений

## 10. TS: Vue composable useGridSnapshot

- [x] 10.1 Переписать BiomeCounts как Record<number, number> (динамические ключи)
- [x] 10.2 Передать biomeDefinitions в composable для отображения
- [x] 10.3 Обновить computed biomeCounts под новый формат данных

## 11. TS: Рендеринг (MapRenderer)

- [x] 11.1 MapRenderer принимает biomeDefinitions вместо захардкоженной BIOME_COLORS
- [x] 11.2 Для biome_id без определения использовать чёрный цвет (0x000000)

## 12. TS: StatusPanel

- [x] 12.1 Переписать статический rows на динамический список из biomeDefinitions
- [x] 12.2 Отображать все биомы из конфига с их цветами и количеством

## 13. Сборка и проверка

- [x] 13.1 `cargo build --target wasm32-unknown-unknown` — успешная компиляция
- [x] 13.2 `wasm-pack build` — генерация WASM
- [x] 13.3 `npm run dev` — приложение запускается, карта отображается
- [x] 13.4 Проверить: добавление 5-го биома в biomeConfig.ts → появляется на карте и в панели
