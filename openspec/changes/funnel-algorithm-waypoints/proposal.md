## Why

Waypoints юнитов центрируются по тайлам (центр 32×32 клетки), из-за чего траектория движения — ломаная линия через центры клеток, даже при 8-направленном A*. Визуально юнит ходит «по квадратам», а не по кратчайшей траектории. Funnel Algorithm (Simple Stupid Funnel Algorithm) натягивает путь как струну через коридор клеток, располагая waypoint'ы на углах тайлов — траектория становится кратчайшей.

## What Changes

- `engine/src/ecs/pathfinding.rs`: добавлены `portal_vertices()` (извлекает вершины общего ребра между смежными клетками) и `funnel_algorithm()` (SSFA — пост-процессинг A* пути в свободные waypoint'ы)
- `engine/src/wasm_api.rs`: `set_unit_target` — после A* применяет funnel_algorithm; `MovementTarget` устанавливается на фактическую позицию клика, не на центр тайла
- A* (`find_path`) — без изменений. Продолжает возвращать `Vec<(u32, u32)>` клеток
- ECS-компоненты (`Position`, `Path`, `MovementTarget`) — без изменений
- `movement_system` — без изменений (уже поддерживает lerp между произвольными `(f64, f64)`)

## Capabilities

### New Capabilities

_(нет — изменение реализации в рамках существующих компонентов)_

### Modified Capabilities

- `ecs-engine`: pathfinding — добавлен `funnel_algorithm` как пост-процессинг A* пути
- `unit-management`: `set_unit_target` — waypoints теперь на вершинах тайлов, `MovementTarget` = точная позиция клика (не центр тайла); movement system изменений не требует

## Impact

- `engine/src/ecs/pathfinding.rs`: +2 функции (~100 строк), новые тесты
- `engine/src/wasm_api.rs`: `set_unit_target` (~10 строк изменений), импорт `tile_to_pixel` заменяется на `funnel_algorithm`
- Тесты: добавить тесты funnel_algorithm (прямой путь, поворот, S-образный, диагональ)
