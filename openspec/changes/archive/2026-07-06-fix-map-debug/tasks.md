## 1. MapCamera — фикс перетаскивания

- [x] 1.1 Изменить `pointerDown` с `{x:0, y:0}` на `null`
- [x] 1.2 Добавить проверку `this.pointerDown` в `handlePointerMove`
- [x] 1.3 Сбрасывать `pointerDown` в `null` в `handlePointerUp`

## 2. Debug — глобальный флаг

- [x] 2.1 Изменить `toggleDebug` на работу без аргумента (все юниты)
- [x] 2.2 Убрать `:disabled` с чекбокса и привязку к `selectedUnitId`
- [x] 2.3 Убрать подсказку "Select a unit first"
- [x] 2.4 Заменить `selectedDebug` на `debugOn` — проверка любого юнита
