## ADDED Requirements

### Requirement: WASM функция greet
Система SHALL предоставить функцию `greet()`, скомпилированную из Rust в WASM, возвращающую строку "Hello World from Rust!".

#### Scenario: Вызов greet через wasm-bindgen
- **WHEN** WebWorker загружает WASM-модуль и вызывает `greet()`
- **THEN** функция возвращает строку "Hello World from Rust!"

### Requirement: WebWorker生命周期
Система SHALL создать WebWorker при старте Vue-приложения, загружающий WASM и вызывающий greet() автоматически.

#### Scenario: Worker отправляет приветствие
- **WHEN** WebWorker инициализирован и вызвал greet()
- **THEN** worker отправляет main thread'у сообщение `{ type: "greeting", text: "Hello World from Rust!" }`

### Requirement: Bridge-прокси
Система SHALL предоставить типизированную обёртку `bridge/wasm.ts` для отправки и получения сообщений между main thread и worker.

#### Scenario: Отправка сообщения через bridge
- **WHEN** main thread отправляет сообщение через bridge
- **THEN** worker получает его и обрабатывает

#### Scenario: Получение сообщения через bridge
- **WHEN** worker отправляет ответное сообщение
- **THEN** bridge доставляет его в main thread с правильным типом
