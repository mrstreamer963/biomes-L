## Purpose

Real-time game loop: UI-тред измеряет время, Worker выполняет tick ECS.

## Requirements

### Requirement: Game loop на UI-треде

Система SHALL запускать game loop через `requestAnimationFrame` на main thread после status: "ready" от worker. Worker SHALL НЕ запускать собственный цикл.

#### Scenario: Старт game loop

- **WHEN** `useGridSnapshot` получает status: "ready"
- **THEN** main thread начинает rAF-цикл, отправляющий tick в worker

### Requirement: Передача dt

Main thread SHALL вычислять `dt = (now - lastTime) / 1000` (с ограничением max ~50ms) и отправлять `{ type: "tick", dt }`.

#### Scenario: Tick сообщение

- **WHEN** UI-тред отправляет `{ type: "tick", dt: 0.016 }`
- **THEN** worker вызывает `tick(handle, 0.016)` и отправляет unit-snapshot

### Requirement: Единый snapshot на кадр

Worker SHALL отправлять unit-snapshot один раз за обработанный tick.

#### Scenario: Snapshot после tick

- **WHEN** tick выполнен в worker
- **THEN** worker отправляет `{ type: "unit-snapshot", units: [...] }`

### Requirement: FPS регулирование

Частота привязана к rAF (~60 FPS). При неактивной вкладке rAF не срабатывает — tick не отправляется.

#### Scenario: Неактивная вкладка

- **WHEN** вкладка в фоне
- **THEN** game loop приостанавливается

### Requirement: Остановка при размонтировании

При unmount Vue-компонента: `cancelAnimationFrame`, `worker.terminate()`.
