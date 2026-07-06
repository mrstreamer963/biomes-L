## Why

Карта двигается при простом движении мыши без нажатой кнопки, что делает управление невозможным. Также debug-флаг для показа waypoints требует выделения юнита, что неудобно при отладке.

## What Changes

- **Фикс MapCamera**: `pointerDown` инициализируется как `null` вместо `{x:0, y:0}`. `handlePointerMove` игнорирует движение, если не было `pointerdown`. `handlePointerUp` сбрасывает `pointerDown` в `null`.
- **Глобальный debug-флаг**: чекбокс Debug больше не требует выделенного юнита. При включении — показывает waypoints на всех юнитах сразу. При выключении — убирает со всех.
- **Убрано ограничение**: удалён `:disabled` с чекбокса, удалена подсказка "Select a unit first".

## Capabilities

### New Capabilities

_Нет новых capability._

### Modified Capabilities

- `pixi-renderer`: изменить обработку pointer-событий в MapCamera — перетаскивание только при зажатой кнопке
- `vue-shell`: изменить поведение debug-флага на глобальное, убрать привязку к selectedUnitId

## Impact

- `src/game/MapCamera.ts` — изменение логики `handlePointerMove`
- `src/components/StatusPanel.vue` — упрощение debug-секции
- `src/composables/useGridSnapshot.ts` — `toggleDebug` принимает и обрабатывает `null` (все юниты)
