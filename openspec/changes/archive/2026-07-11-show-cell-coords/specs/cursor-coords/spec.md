## Purpose

Отображать координаты тайла (col, row) под курсором мыши в панели статуса в реальном времени при наведении на игровое поле PixiJS.

## Requirements

### Requirement: Реактивное состояние позиции курсора

Система SHALL предоставить реактивное свойство `hoveredCell` в `useGridSnapshot` composable, содержащее координаты тайла под курсором мыши или `null`, если курсор вне сетки.

#### Scenario: hoveredCell обновляется при движении мыши

- **WHEN** мышь движется над canvas
- **THEN** `hoveredCell.value` содержит `{ col, row }` с актуальными координатами тайла

#### Scenario: hoveredCell сбрасывается при pointerleave

- **WHEN** мышь покидает canvas (pointerleave)
- **THEN** `hoveredCell.value` равен `null`

### Requirement: Конвертация экранных координат в тайловые

Система SHALL вычислять тайловые координаты из экранных через инвертирование трансформации контейнера PixiJS и деление на TILE_SIZE=32.

#### Scenario: Расчёт тайловых координат

- **WHEN** мышь находится над canvas
- **THEN** worldX = (screenX - container.position.x) / container.scale.x, worldY = (screenY - container.position.y) / container.scale.y
- **THEN** col = Math.floor(worldX / 32), row = Math.floor(worldY / 32)

#### Scenario: Границы сетки

- **WHEN** worldX < 0 или worldY < 0
- **WHEN** col >= width или row >= height
- **THEN** hoveredCell равен `null`

### Requirement: Отображение координат в панели статуса

Система SHALL отображать строку вида "Cell: 12, 34" в StatusPanel.vue при наличии `hoveredCell`.

#### Scenario: Показ координат

- **WHEN** `hoveredCell` не null
- **THEN** в StatusPanel отображается "Cell: {col}, {row}"

#### Scenario: Скрытие при null

- **WHEN** `hoveredCell` равен null
- **THEN** строка с координатами не отображается (или отображается "—")

### Requirement: Обработка pointermove в MapCamera

Система SHALL вызывать колбэк `onPointerMove(screenX, screenY)` из MapCamera при движении мыши, а также `onPointerLeave()` при уходе курсора.

#### Scenario: Подписка на pointermove

- **WHEN** мышь движется над canvas
- **THEN** MapCamera вызывает onPointerMove с screen-координатами

#### Scenario: Очистка при pointerleave

- **WHEN** мышь покидает canvas
- **THEN** MapCamera вызывает onPointerLeave
