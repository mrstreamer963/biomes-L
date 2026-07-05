## ADDED Requirements

### Requirement: UnitManager рендер

Система SHALL добавить `UnitManager` поверх тайловой карты. UnitManager отображает юнитов как круги с health bar и рамкой выделения.

#### Scenario: UnitManager поверх тайлов

- **WHEN** тайловая карта создана
- **THEN** UnitManager.addTo(worldContainer) добавляет слой юнитов поверх тайлов

#### Scenario: Обновление юнитов каждый кадр

- **WHEN** приходит `unit-snapshot` с новыми данными
- **THEN** UnitManager.update(units) перерисовывает круги, health bars, рамки

### Requirement: Hit test на canvas

Система SHALL обрабатывать pointerdown на canvas для определения, попал ли клик по юниту.

#### Scenario: Определение попадания

- **WHEN** игрок кликает на canvas
- **THEN** координаты клика преобразуются в мировые координаты (с учётом позиции и масштаба камеры)
- **THEN** проверяется каждый юнит (reverse order): расстояние от клика до центра юнита < радиуса юнита
- **THEN** возвращается первый попавшийся юнит, или null