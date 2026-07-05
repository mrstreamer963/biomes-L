## ADDED Requirements

### Requirement: Game loop в Web Worker

Система SHALL запустить real-time game loop внутри Web Worker после инициализации WASM и спавна стартовых юнитов.

#### Scenario: Старт game loop

- **WHEN** worker завершил инициализацию и вызвал `spawn_starting_units`
- **THEN** worker запускает цикл с `setTimeout(0)` или `setInterval`, который вызывает `tick(handle, dt)` каждый кадр

### Requirement: Передача dt

Система SHALL измерять время между кадрами на UI-треде (через `performance.now()`) и передавать дельту в Worker через сообщение `tick`.

#### Scenario: Tick сообщение

- **WHEN** UI-тред отправляет `{ type: "tick", dt: 0.016 }`
- **THEN** worker вызывает `tick(handle, 0.016)` и получает snapshot юнитов

### Requirement: Единый snapshot на кадр

Система SHALL отправлять snapshot юнитов из Worker в main thread только один раз за кадр (после tick).

#### Scenario: Snapshot после tick

- **WHEN** tick в Worker выполнен
- **THEN** Worker отправляет `{ type: "unit-snapshot", units: [...] }` в main thread

### Requirement: FPS регулирование

Система SHALL запускать game loop с частотой, привязанной к requestAnimationFrame UI-треда (не чаще 60 FPS).

#### Scenario: rAF привязка

- **WHEN** requestAnimationFrame на UI-треде срабатывает
- **THEN** main thread вычисляет dt и отправляет tick в Worker
- **WHEN** вкладка неактивна (rAF не срабатывает)
- **THEN** game loop останавливается (dt не отправляется)