## Purpose

Отображение тайловой карты биомов на PixiJS canvas с pan и zoom.

## Requirements

### Requirement: Отрисовка сетки биомов

Система SHALL отображать тайловую карту после получения `grid-snapshot` от WASM-воркера.

#### Scenario: Карта отображается после получения данных

- **WHEN** `useGridSnapshot` получает `{ type: "grid-snapshot", width, height, biomeIds, resources }`
- **THEN** на канвасе отрисовывается сетка из `width × height` цветных квадратов через `MapRenderer.createTileMap`

#### Scenario: При загрузке показывается индикатор

- **WHEN** snapshot ещё не получен
- **THEN** отображается "Loading..." поверх canvas

### Requirement: Цветовая схема биомов

Система SHALL брать цвет из `biomeDefinitions[biomeId].color`, а не из захардкоженной таблицы.

#### Scenario: Цвет из конфига

- **WHEN** клетка имеет biomeId = N
- **THEN** цвет = `biomeDefinitions[N].color`

#### Scenario: Неизвестный biomeId

- **WHEN** biomeId отсутствует в biomeDefinitions
- **THEN** клетка заливается чёрным (0x000000)

### Requirement: Сетка между тайлами

Система SHALL отображать тонкие линии между тайлами (1px, белый, alpha 0.1).

### Requirement: Pan канваса

Система SHALL позволять перемещать карту перетаскиванием мышью.

#### Scenario: Drag перемещает карту

- **WHEN** пользователь зажимает ЛКМ и двигает мышь > 5px
- **THEN** карта смещается (pan)

### Requirement: Zoom канваса

Система SHALL поддерживать zoom колесом мыши с центром в позиции курсора, диапазон scale 0.25–3.0.
