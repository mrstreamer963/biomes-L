## Why

Текущая система биомов захардкожена в enum `Biome` с 4 вариантами (Plains, Forest, Water, Mountain). Это ограничивает расширение игры — добавление нового биома требует изменения enum, перекомпиляции WASM и правок на всех слоях (Rust → TS → Pixi). Игра также растёт и требует нормальной ECS-архитектуры вместо плоского `Vec<Cell>`.

## What Changes

- **BREAKING**: Замена `Biome` enum на динамический реестр биомов (bevy_ecs Resource/Assets)
- **BREAKING**: Замена плоского `Vec<Cell>` на bevy_ecs World с Entity/Component моделью
- **BREAKING**: Изменение wire-формата (`Uint8Array` биомов → массив entity id + компоненты)
- Расширение WASM API для работы с произвольным списком биомов
- Конфиг биомов (определения: name, passable, speed, color) — загружается с JS-стороны
- Рендеринг через Pixi.js получает маппинг biome id → цвет динамически

## Capabilities

### New Capabilities
- `ecs-engine`: ядро игры на bevy_ecs — World, Commands, компоненты, системы, ресурсы
- `dynamic-biomes`: произвольный список биомов с настраиваемыми свойствами (name, passable, speed, color), загружаемый через конфиг

### Modified Capabilities
- `wasm-worker`: bridge между ECS World и TypeScript должен передавать динамические данные (не захардкоженный enum)
- `grid-rendering`: рендеринг должен получать конфиг биомов и использовать динамический маппинг biome id → цвет
- `vue-shell`: StatusPanel должен показывать биомы из конфига, а не захардкоженный список

## Impact

- `engine/src/`: удаление `biome.rs`, переработка `grid.rs` → ECS системы, изменение `wasm_api.rs`
- `engine/Cargo.toml`: добавление `bevy-ecs` (без bevy_render и других модулей)
- `src/wasm/engine.d.ts`: изменение типов GridSnapshot и CellData
- `src/worker/game.worker.ts`: конфиг биомов передаётся из main thread
- `src/composables/useGridSnapshot.ts`: динамические BiomeCounts
- `src/game/MapRenderer.ts`: динамические цвета биомов
- `src/components/StatusPanel.vue`: динамический список биомов
