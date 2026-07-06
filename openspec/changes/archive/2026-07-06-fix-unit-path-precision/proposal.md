## Why

При клике по карте финальная точка пути юнита иногда смещается на 1 ячейку из-за использования `.round()` вместо `.floor()` в `pixel_to_tile`. Также обнаружен дублирующийся обработчик клика в `GameCanvas.vue`, вызывающий двойную отправку команды движения.

## What Changes

- **pixel_to_tile**: заменить `.round()` на `.floor().max(0.0)` — конвертация пиксель->тайл теперь всегда выбирает тайл, содержащий пиксель, а не ближайший
- **movement_system**: заменить `.round()` на `.floor()` в двух inline-вычислениях тайла для определения биома под юнитом и проверки проходимости новой позиции
- **GameCanvas.vue**: удалить прямой `canvas.addEventListener('click', ...)`, оставив только обработку через `MapCamera.handlePointerUp`, которая корректно отличает drag от click

## Capabilities

### New Capabilities
<!-- Нет новых возможностей — только исправление ошибок -->

### Modified Capabilities
- `unit-management`: Requirement "Movement system" — исправлена формула определения тайла по позиции (col = (pos.x / TILE_SIZE).floor(), row = (pos.y / TILE_SIZE).floor())
- `ecs-engine`: Добавлена корректная конвертация пиксельных координат в тайловые через pixel_to_tile с .floor()
- `vue-shell`: Удалён дублирующийся обработчик click на canvas

## Impact

- `engine/src/ecs/pathfinding.rs` — `pixel_to_tile` (signature не меняется)
- `engine/src/ecs/systems.rs` — biome lookup в `movement_system`
- `src/components/GameCanvas.vue` — удаление дублирующегося event listener
