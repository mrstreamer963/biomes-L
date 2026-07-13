## Purpose

Рендеринг игровой графики через PixiJS v8 внутри Vue-приложения.

## Requirements

### Requirement: Инициализация PixiJS v8

Система SHALL инициализировать `PIXI.Application` в `GameCanvas.vue` при монтировании.

#### Scenario: Создание Application

- **WHEN** `GameCanvas.vue` смонтирован
- **THEN** создаётся PIXI.Application и добавляется в DOM

### Requirement: Отрисовка тайловой карты

Система SHALL отображать карту биомов через `MapRenderer.createTileMap(biomeIds, biomeDefinitions, width, height)` после получения grid-snapshot.

#### Scenario: Карта поверх canvas

- **WHEN** `biomeIds` и `biomeDefinitions` доступны
- **THEN** на сцене отображается цветная тайловая сетка `TILE_SIZE = 32`

### Requirement: Canvas встраивается в layout

Canvas SHALL занимать 100% родительского контейнера.

### Requirement: UnitManager рендер

`UnitManager` SHALL отображаться поверх тайлов: круги юнитов, health bar, рамка выделения, waypoints в debug-режиме.

#### Scenario: UnitManager поверх тайлов

- **WHEN** тайловая карта создана
- **THEN** `UnitManager` добавлен в world container поверх тайлов

#### Scenario: Обновление каждый кадр

- **WHEN** приходит unit-snapshot
- **THEN** `UnitManager.update(units)` перерисовывает юнитов

### Requirement: Hit test на canvas

Pointerdown/up на canvas: преобразование в мировые координаты с учётом камеры, hit-test юнитов (reverse order).

### Requirement: Pan только при зажатой кнопке

Pan начинается после pointerdown + перемещение > 5px. Движение без нажатой кнопки не двигает карту.
