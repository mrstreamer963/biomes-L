## Purpose

PixiJS-рендер юнитов: кружки, health bars, выделение, waypoints, клики.

## Requirements

### Requirement: UnitManager отрисовывает юнитов

#### Scenario: Отрисовка

- **WHEN** `update(units)` с массивом UnitData
- **THEN** Scout = круг ~20px, Soldier ~26px, цвет Team 0 (зелёный), health bar сверху

#### Scenario: Обновление позиций

- **WHEN** `update(units)` с новыми координатами
- **THEN** круги перемещаются

### Requirement: Выделение юнитов

Single selection: один выделенный юнит с белой рамкой.

#### Scenario: Смена выделения

- **WHEN** выбран A, затем клик по B
- **THEN** выделение на B

### Requirement: Health bar

Полоска HP = current / max над юнитом.

### Requirement: Debug waypoints

При `unit.debug === true` SHALL отображаться путь (waypoints) юнита.

### Requirement: Hit test и команды

#### Scenario: Клик по юниту

- **WHEN** клик в радиусе юнита
- **THEN** юнит выделяется

#### Scenario: Клик по пустой клетке

- **WHEN** есть выделенный юнит, клик по пустой области
- **THEN** `set-unit-target` с координатами в пикселях карты
