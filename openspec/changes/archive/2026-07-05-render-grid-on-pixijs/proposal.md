## Why

Заменить красный круг-демо в PixiJS на рендер карты биомов. Сетка 32×32 уже генерируется в WASM и приходит в main thread, но никак не визуализируется — игра до сих пор показывает заглушку.

## What Changes

- PixiJS canvas вместо круга отрисовывает тайловую карту из цветных квадратов по данным `grid-snapshot`
- Каждый биом (Plains, Forest, Water, Mountain) имеет свой цвет
- Добавляется тонкая сетка между тайлами для читаемости
- Карта помещается в `worldContainer` с поддержкой pan и zoom (через колесо мыши и drag)
- `GameCanvas.vue` получает snapshot через `useGridSnapshot` и передаёт в `MapRenderer`

## Capabilities

### New Capabilities
- `grid-rendering`: Отрисовка сетки биомов на PixiJS — цветные тайлы, сетка, camera (pan/zoom)

### Modified Capabilities

<!-- Спецификация pixi-renderer остаётся без изменений — она описывает инициализацию PixiJS, что уже сделано. Новая функциональность — отдельная capability. -->

## Impact

- `src/components/GameCanvas.vue` — полностью переписывается сцена (вместо круга — карта)
- Новый файл `src/game/MapRenderer.ts` — логика отрисовки сетки
- Новый файл `src/game/MapCamera.ts` — pan/zoom камера
- `useGridSnapshot` уже существует и готов предоставлять данные — интерфейс не меняется
- Все изменения — только frontend, WASM/worker слой не трогается
