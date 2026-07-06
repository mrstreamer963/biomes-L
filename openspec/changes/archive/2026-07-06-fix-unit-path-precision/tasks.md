## 1. ECS Engine — Исправление округления в pathfinding и movement

- [x] 1.1 `pixel_to_tile` в `engine/src/ecs/pathfinding.rs`: заменить `.round()` на `.floor().max(0.0)`
- [x] 1.2 Biome lookup в `movement_system` (`engine/src/ecs/systems.rs`): заменить `.round()` на `.floor()` для определения биома под юнитом
- [x] 1.3 Проверка проходимости новой позиции в `movement_system`: заменить `.round()` на `.floor()`
- [x] 1.4 Собрать (`cargo check`) и убедиться в отсутствии ошибок компиляции

## 2. Vue Shell — Удаление дублирующегося click-обработчика

- [x] 2.1 Удалить `canvas.addEventListener('click', ...)` в `src/components/GameCanvas.vue`
- [x] 2.2 Убедиться, что `MapCamera.handlePointerUp` корректно вызывает `handleClick` для non-drag кликов

## 3. Верификация

- [x] 3.1 Проверить сборку `cargo check` в engine/
- [x] 3.2 Проверить линтинг TypeScript части (`npm run lint` или аналог)
