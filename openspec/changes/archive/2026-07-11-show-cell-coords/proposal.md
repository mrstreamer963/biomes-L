## Why

Игроку не видно, над какой клеткой карты находится курсор мыши. Это мешает точному позиционированию юнитов и пониманию координат на сетке. Нужно отображать текущие координаты тайла (row, col) в панели статуса в реальном времени при наведении мыши на игровое поле.

## What Changes

- Добавить отслеживание позиции мыши над canvas (pointermove) и конвертацию пиксельных координат в тайловые
- Вывести координаты тайла (col, row) в StatusPanel.vue в реальном времени
- Обновить useGridSnapshot composable — добавить реактивное состояние hoveredCell

## Capabilities

### New Capabilities
- `cursor-coords`: отображение координат тайла (строка, столбец) под курсором мыши в реальном времени

### Modified Capabilities

<!-- No existing specs have requirement changes. -->

## Impact

- `src/components/StatusPanel.vue` — новый блок с координатами
- `src/components/GameCanvas.vue` — подписка на pointermove, вычисление тайловых координат
- `src/composables/useGridSnapshot.ts` — новое реактивное свойство `hoveredCell`
