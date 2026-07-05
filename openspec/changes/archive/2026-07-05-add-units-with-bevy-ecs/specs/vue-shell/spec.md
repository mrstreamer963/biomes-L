## ADDED Requirements

### Requirement: Reactive состояние юнитов

Система SHALL предоставить реактивное состояние `units` в `useGridSnapshot` composable, обновляемое при получении `unit-snapshot`.

#### Scenario: units ref обновляется

- **WHEN** main thread получает `{ type: "unit-snapshot", units: [...] }`
- **THEN** `useGridSnapshot().units.value` содержит актуальный массив юнитов

### Requirement: Game loop на UI-треде

Система SHALL запустить requestAnimationFrame цикл на UI-треде после инициализации. Каждый кадр:
1. Вычислить dt = (now - lastTime) / 1000
2. Отправить `{ type: "tick", dt }` в Worker
3. После получения `unit-snapshot` — обновить состояние

#### Scenario: rAF запущен

- **WHEN** приложение готово (worker отправил status: "ready")
- **THEN** запускается requestAnimationFrame цикл, отправляющий tick каждые ~16ms

### Requirement: Остановка game loop

Система SHALL остановить game loop при демонтировании компонента.

#### Scenario: clean up

- **WHEN** компонент размонтирован
- **THEN** rAF отменён, worker завершён