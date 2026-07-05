## Purpose

Обеспечить рендеринг игровой графики через PixiJS v8 внутри Vue-приложения. TBD — цели будут уточнены в процессе разработки.

## Requirements

### Requirement: Инициализация PixiJS v8
Система SHALL инициализировать PIXI.Application v8 внутри Vue-компонента `GameCanvas.vue` при монтировании.

#### Scenario: Создание Application
- **WHEN** компонент `GameCanvas.vue` смонтирован
- **THEN** создаётся PIXI.Application с настройками по умолчанию и добавляется в DOM

### Requirement: Демо-рендер
Система SHALL отобразить демо-графику (цветной круг) на PixiJS canvas после инициализации.

#### Scenario: Отрисовка круга
- **WHEN** PIXI.Application инициализирован
- **THEN** на сцене отображается как минимум один графический примитив (круг/прямоугольник)

### Requirement: Canvas встраивается в layout
Система SHALL разместить PixiJS canvas внутри Vue-компонента с адаптивным размером.

#### Scenario: Canvas занимает контейнер
- **WHEN** `GameCanvas.vue` отрендерен
- **THEN** canvas занимает 100% ширины и высоты родительского контейнера

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
