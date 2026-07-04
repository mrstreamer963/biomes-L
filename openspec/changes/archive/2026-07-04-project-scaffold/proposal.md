## Why

Создать заготовку проекта для игры Biomes: фронтенд на Vite + Vue + PixiJS v8 и бэкенд на Rust, компилируемый в WASM и работающий как WebWorker. Это фундамент, на котором будет строиться вся игровая логика.

## What Changes

- Инициализация Vite + Vue 3 проекта с корневым `package.json`
- Инициализация Rust workspace с корневым `Cargo.toml` и крейтом `engine/`
- Интеграция PixiJS v8 в Vue-компонент
- Rust → WASM → WebWorker: функция `greet()`, возвращающая "Hello World"
- Автоматическое отображение приветствия от WASM на форме при загрузке
- Обновление `scripts/build-wasm.sh` под новую структуру

## Capabilities

### New Capabilities

- `wasm-worker`: Rust-код, скомпилированный в WASM, работающий в WebWorker с двусторонней связью через postMessage
- `pixi-renderer`: Инициализация PixiJS v8 в Vue-компоненте с демо-рендером
- `vue-shell`: Vue-приложение с формой, автоматически заполняемой данными от WASM-воркера

### Modified Capabilities

*(нет — первый коммит в проекте)*

## Impact

- **Новые зависимости**: `vite`, `vue`, `pixi.js` (npm); `wasm-bindgen`, `wasm32-unknown-unknown` (Rust)
- **Инфраструктура сборки**: скрипт `scripts/build-wasm.sh` будет обновлён; Vite-конфиг — с алиасами и поддержкой WebWorker
- **Базовая структура**: корневые `Cargo.toml`, `package.json`, `vite.config.ts`, `tsconfig.json`
