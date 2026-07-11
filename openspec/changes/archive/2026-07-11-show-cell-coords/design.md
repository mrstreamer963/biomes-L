## Context

Игрок не видит координаты клетки под курсором. Тайловая сетка 50×50 (TILE_SIZE=32px), камера поддерживает pan/zoom. Сейчас клик обрабатывается через MapCamera.handlePointerUp, но hover-информация (pointermove) никуда не передаётся.

useGridSnapshot — синглтон-композабл, через который GameCanvas и StatusPanel обмениваются состоянием. Панель статуса (StatusPanel.vue) уже отображает размер сетки, список биомов и debug-флаг.

## Goals / Non-Goals

**Goals:**
- При движении мыши над canvas отображать в StatusPanel координаты тайла (col, row) под курсором
- Координаты обновляются в реальном времени, без задержек
- При уходе мыши за пределы canvas — очищать отображение

**Non-Goals:**
- Подсветка клетки под курсором (hover highlight)
- Клик или выделение клетки по hover
- Отображение информации о содержимом клетки (биом, ресурсы)

## Decisions

- **Хранение hoveredCell в useGridSnapshot**: добавляем `hoveredCell: Ref<{ col: number; row: number } | null>`. Это единственный путь передачи данных от GameCanvas к StatusPanel, и он уже существует.
- **Конвертация пикселей → тайл**: формула `col = floor((worldX) / TILE_SIZE)`, `row = floor((worldY) / TILE_SIZE)`. Координаты world вычисляются из screen через инвертирование трансформации контейнера (как в handleClick).
- **Подписка на pointermove в MapCamera**: расширяем handlePointerMove колбэком `onPointerMove(screenX, screenY)`. Либо добавляем отдельный listener в GameCanvas.vue. Первый вариант компактнее — не плодить listener'ы.
- **Очистка при pointerleave**: в handlePointerLeave вызываем колбэк с `null`.
- **Пропуск при drag**: во время панорамирования координаты не обновляем (или обновляем — без разницы, это не влияет на производительность).

## Risks / Trade-offs

- [Performance] pointermove может генерировать много событий → достаточно лёгких вычислений (2 деления), проблем быть не должно
- [Edge case] Выход курсора за границы сетки (worldX < 0, worldY < 0, >= width*32, >= height*32) — показывать null, чтобы не вводить в заблуждение
- [Edge case] При зуме координаты считаются корректно, т.к. world-координаты уже учитывают scale и position контейнера
