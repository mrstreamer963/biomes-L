## 1. MapRenderer

- [x] 1.1 Создать `src/game/MapRenderer.ts` с функцией `createTileMap(biomes: Uint8Array, width: number, height: number): PIXI.Container` — создаёт и возвращает контейнер с Graphics-тайлами
- [x] 1.2 Определить константу `TILE_SIZE = 32` и мапу цветов биомов `BIOME_COLORS`
- [x] 1.3 Каждый тайл — `new Graphics().rect(0, 0, TILE_SIZE, TILE_SIZE).fill({ color })` с позицией `(x * TILE_SIZE, y * TILE_SIZE)`
- [x] 1.4 Добавить обводку тайлов: `rect(...).stroke({ width: 1, color: 0xffffff, alpha: 0.1 })`

## 2. MapCamera (pan / zoom)

- [x] 2.1 Создать `src/game/MapCamera.ts` с классом `MapCamera`, который принимает `PIXI.Container` (world container)
- [x] 2.2 Реализовать pan: подписка на `pointerdown` / `pointermove` / `pointerup` на канвасе, смещение `container.position`
- [x] 2.3 Реализовать zoom: подписка на `wheel`, изменение `container.scale` с клампингом (0.25–3), смещение позиции для zoom-to-cursor

## 3. Интеграция в GameCanvas.vue

- [x] 3.1 Добавить вызов `useGridSnapshot()` в `GameCanvas.vue`
- [x] 3.2 Удалить код с красным кругом
- [x] 3.3 В `watch(biomes, ...)` передавать snapshot в MapRenderer и добавлять результат на сцену
- [x] 3.4 Инициализировать MapCamera с world container после рендера карты
- [x] 3.5 Показывать "Loading..." на канвасе пока snapshot не получен

## 4. Проверка

- [x] 4.1 Собрать WASM (`bash scripts/build-wasm.sh --dev`)
- [x] 4.2 Запустить dev-сервер и проверить: карта отображается, pan/zoom работают
