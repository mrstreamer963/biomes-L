## ADDED Requirements

### Requirement: Funnel Algorithm для генерации waypoint'ов в set_unit_target

Система SHALL в функции `set_unit_target` после A* применять `funnel_algorithm` для генерации waypoint'ов. Waypoint'ы SHALL быть свободными (на вершинах тайлов). `MovementTarget` SHALL устанавливаться на точную позицию клика `(x, y)`, а не на центр тайла.

#### Scenario: Путь через несколько клеток

- **WHEN** юнит в позиции (16, 16) получает команду движения в (55, 55)
- **WHEN** A* вернул путь `[(0,0), (1,0), (1,1)]`
- **WHEN** `set_unit_target` вызван
- **THEN** `Path` компонент содержит waypoint'ы на вершинах тайлов (не в центрах), например `[(32, 32), (55, 55)]`
- **THEN** `MovementTarget` = `Some((55, 55))`

#### Scenario: Прямой путь, MovementTarget = позиция клика

- **WHEN** юнит в позиции (16, 16) получает команду движения в (90, 16)
- **WHEN** A* вернул прямой путь
- **THEN** `MovementTarget` = `Some((90, 16))` (точная позиция клика)

#### Scenario: Путь из одной клетки

- **WHEN** юнит кликает в ту же клетку, где стоит
- **WHEN** A* вернул путь из 1 клетки
- **THEN** waypoints = `[(x, y)]` — без пост-процессинга
- **THEN** `MovementTarget` = `Some((x, y))`

### Requirement: Нет изменений в movement_system

Система SHALL НЕ изменять `movement_system`. Система продолжает обрабатывать waypoint'ы из `Path` и `MovementTarget` как `(f64, f64)` в пикселях.

#### Scenario: Движение по свободным waypoint'ам

- **WHEN** `Path` содержит waypoint'ы на вершинах тайлов (например, (32, 32))
- **WHEN** `movement_system` обрабатывает юнит
- **THEN** юнит движется lerp-ом к (32, 32), затем к `MovementTarget`
