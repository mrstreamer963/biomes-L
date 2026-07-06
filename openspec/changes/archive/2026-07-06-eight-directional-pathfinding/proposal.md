## Why

A* использует 4 направления, из-за чего пути получаются ступенчатыми (зигзаги 90°). Даже при lerp-движении между waypoints траектория остаётся неестественной. Переход на 8 направлений даёт гладкие диагональные пути без изменения системы waypoints.

## What Changes

- A* с 4 направлений → 8 направлений (добавлены диагонали)
- Эвристика Manhattan → Chebyshev (максимум из dx, dy)
- Стоимость шага: прямой сосед = 1, диагональный = √2 (1.414)
- `f_score`, `g_score`: `u32` → `f64` для точности с дробными cost
- **BREAKING**: `Node.f_score` меняет тип, возвращаемый путь остаётся `Vec<(u32, u32)>`
- `tile_to_pixel` и waypoints — без изменений (остаются центрами тайлов)
- Проверка проходимости диагонального шага: оба смежных прямых тайла должны быть проходимы (Diagonally-adjacent passability check)

## Capabilities

### New Capabilities

- <none>

### Modified Capabilities

- `ecs-engine`: pathfinding — A* переходит на 8 направлений, Chebyshev эвристика, f64 стоимости, проверка проходимости стенок на диагоналях
- `unit-management`: movement system — lerp-движение уже поддерживает любые направления, изменений не требуется

## Impact

- `engine/src/ecs/pathfinding.rs`: Node.f_score → f64, DIRECTIONS → 8, heuristic → Chebyshev, g_cost = 1.0/1.414, проверка diagonally-adjacent
- `engine/src/ecs/systems.rs`: возможны мелкие правки типов (если f64 где-то не совместим)
- Тесты pathfinding: обновить ожидаемые пути (теперь короче, с диагоналями)
