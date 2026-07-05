## Context

Текущая архитектура: плоский `Vec<Cell>` в `grid.rs`, где `Cell` содержит `biome: Biome` (enum с 4 вариантами) и `resources: u8`. Генерация карты — LCG PRNG. Весь игровой код — в одной WASM-функции. На фронтенде — Uint8Array биомов, захардкоженный маппинг id → цвет.

Проблема: каждое расширение (новый биом, новые свойства клеток) требует правки enum, перекомпиляции, синхронизации TS-типов.

## Goals / Non-Goals

**Goals:**
- Заменить `Biome` enum на динамический реестр (bevy_ecs Resource/Assets)
- Внедрить bevy_ecs как ядро игровой логики (World + Entities + Components + Systems)
- Cell → Entity с компонентами (BiomeComponent, ResourcesComponent, PositionComponent)
- Конфиг биомов загружается с JS-стороны как массив определений
- WASM API возвращает динамические данные (массив определений биомов, snapshot клеток через компоненты)
- Сохранить производительность (WASM, WebWorker, structured clone)

**Non-Goals:**
- Полный переход на bevy (не нужен bevy_render, bevy_asset, bevy_app — только bevy_ecs)
- Изменение RNG (LCG остаётся)
- Изменение системы рендеринга (Pixi.js остаётся)
- Многопоточность ECS (single-threaded World в WASM)

## Decisions

### Decision 1: bevy_ecs standalone (без bevy_app)

**Выбор:** Использовать `bevy_ecs` как библиотеку (core ECS), без `bevy_app`, `bevy_schedule`, `bevy_render`.

**Rationale:** `bevy_ecs` — минимальная зависимость (World, Entity, Commands, Components, Resources). `bevy_app` добавляет runtime/системы, которые конфликтуют с WASM-моделью (single-threaded, event loop из JS). Управление будем делать вручную через `World::run_system_once()`.

**Alternative considered:** Самописная ECS → отклонено, bevy_ecs industry-standard, проверен в WASM.

### Decision 2: Cell → Entity с компонентами

**Выбор:** Каждая клетка — Entity с набором компонентов: `Position { x, y }`, `BiomeId(u16)`, `Resources(u8)`. Карта — `Vec<Entity>` в GridResource.

**Rationale:** Позволяет добавлять новые компоненты без изменения существующих структур. Query-запросы дают гибкость (например, "все Water клетки", "клетки с ресурсами > 0").

**Alternative considered:** Одна компонента `CellComponent` с кучей полей → отклонено, теряется гибкость ECS.

### Decision 3: Динамический BiomeId вместо enum

**Выбор:** `BiomeId(u16)`, ссылающийся на `BiomeDefinitions` Resource.

**Rationale:** Позволяет иметь до 65535 типов биомов. Определения биомов передаются с JS-стороны как `[{ name, passable, speed, color }]`. Rust хранит только id, маппинг id → свойства — в Resource.

**Alternative considered:** String-based id → отклонено, дорого для WASM.

### Decision 4: WASM API — два вызова

**Выбор:** Два WASM API вызова:
1. `register_biome_definitions(handle, JsValue)` — передать массив определений биомов
2. `grid_snapshot(handle)` → `{ width, height, biomes: Map<number, { biome_id, resources }> }` или структурированный массив

**Rationale:** Конфиг биомов отделён от данных клеток. Snapshot возвращает массив entity-данных, которые TS может интерпретировать с помощью полученного ранее конфига.

### Decision 5: Конфиг биомов как JS-константа

**Выбор:** Определения биомов — JS-константа в `biomeConfig.ts`, передаваемая в WASM при инициализации.

**Rationale:** Простейший источник truth. В будущем можно заменить на JSON с сервера.

## Risks / Trade-offs

1. **[Размер WASM]** bevy_ecs увеличит размер .wasm → Mitigation: использовать `bevy_ecs` с отключёнными фичами (default-features = false), только core ECS
2. **[Производительность]** Entity + компоненты вместо плоского массива → Mitigation: для snapshot используем итерацию по компонентам (sparse set), что O(n) как и сейчас
3. **[Сложность]** ECS сложнее плоской модели → Mitigation: документируем паттерны, сохраняем простые wrapper-функции в wasm_api
4. **[Совместимость]** Изменение wire-формата ломает текущий TS-код → Mitigation: все изменения осознанно BREAKING, обновляем TS-типы синхронно

## Migration Plan

1. Добавить `bevy-ecs` в Cargo.toml
2. Создать ECS-компоненты (Position, BiomeId, Resources)
3. Создать BiomeDefinitions Resource
4. Переписать Grid::generate() на создание Entity + компоненты
5. Обновить wasm_api: register_biome_definitions, grid_snapshot
6. Обновить TS-типы и bridge
7. Обновить useGridSnapshot и MapRenderer

## Open Questions

- Какой именно wire-формат snapshot? (пока PlanarSnapshot: параллельные Uint16Array/biome_id + Uint8Array/resources, или массив структур)
- Нужен ли biomeId = 255/NONE для out-of-bounds?
