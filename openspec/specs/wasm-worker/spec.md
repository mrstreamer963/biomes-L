## Purpose

Обеспечить выполнение Rust-кода в браузере через WASM и WebWorker. TBD — цели будут уточнены в процессе разработки.

## Requirements

### Requirement: WebWorker жизненный цикл
Система SHALL создать WebWorker при старте Vue-приложения, загружающий WASM. При `init` worker SHALL создавать сетку биомов по умолчанию (фиксированные `seed`, `width`, `height`) через `create_grid` и отправлять main thread'у сообщение `grid-snapshot` с snapshot'ом сетки.

#### Scenario: Worker отправляет snapshot сетки
- **WHEN** WebWorker инициализирован и вызвал `create_grid` + `grid_snapshot`
- **THEN** worker отправляет main thread'у сообщение `{ type: "grid-snapshot", width, height, biomes }`, где `biomes` — массив индексов биомов

#### Scenario: Worker сообщает об ошибке движка
- **WHEN** инициализация WASM или генерация сетки завершается ошибкой
- **THEN** worker отправляет сообщение `{ type: "status", status: "error", error }`

### Requirement: Bridge-прокси
Система SHALL предоставить типизированную обёртку `bridge/wasm.ts` для отправки и получения сообщений между main thread и worker, включая новое сообщение `grid-snapshot`.

#### Scenario: Отправка сообщения через bridge
- **WHEN** main thread отправляет сообщение через bridge
- **THEN** worker получает его и обрабатывает

#### Scenario: Получение snapshot сетки через bridge
- **WHEN** worker отправляет сообщение `grid-snapshot`
- **THEN** bridge доставляет его в main thread с типизированными полями `width`, `height`, `biomes`
