## 1. Rust → WASM

- [x] 1.1 Создать `Cargo.toml` в корне (workspace) c членом `engine`
- [x] 1.2 Создать `engine/Cargo.toml` c зависимостями `wasm-bindgen`
- [x] 1.3 Реализовать `engine/src/lib.rs` с `#[wasm_bindgen] pub fn greet() -> String`
- [x] 1.4 Обновить `scripts/build-wasm.sh` под новую структуру (engine → src/wasm/)
- [x] 1.5 Добавить `target/` в `.gitignore`
- [x] 1.6 Собрать WASM (`./scripts/build-wasm.sh --dev`), убедиться что компилируется

## 2. Vite + Vue проект

- [x] 2.1 Создать `package.json` с зависимостями: `vue`, `vite`, `@vitejs/plugin-vue`, `pixi.js` (v8), `typescript`
- [x] 2.2 Создать `vite.config.ts` с плагином Vue и алиасами
- [x] 2.3 Создать `tsconfig.json` и `tsconfig.node.json`
- [x] 2.4 Создать `index.html` с точкой входа `/src/main.ts`
- [x] 2.5 Создать `src/main.ts` — bootstrap Vue приложения
- [x] 2.6 Создать `src/App.vue` — корневой компонент с layout (30%/70%)
- [x] 2.7 Установить зависимости и проверить `npm run dev`

## 3. WebWorker + WASM bridge

- [x] 3.1 Создать `src/worker/game.worker.ts` — загрузка WASM, вызов greet(), postMessage
- [x] 3.2 Создать `src/bridge/wasm.ts` — типизированная обёртка над Worker
- [x] 3.3 Создать `src/composables/useWasmGreeting.ts` — реактивный composable для подписки на приветствие
- [x] 3.4 Протестировать связку: worker → greet() → postMessage → Vue

## 4. PixiJS v8 рендер

- [x] 4.1 Создать `src/components/GameCanvas.vue` — инициализация PIXI.Application в onMounted
- [x] 4.2 Добавить цветной круг как демо-графику
- [x] 4.3 Подключить GameCanvas.vue в App.vue

## 5. Форма с авто-заполнением

- [x] 5.1 Создать `src/components/StatusPanel.vue` — форма с полем ввода, статусом воркера, индикатором
- [x] 5.2 Подключить `useWasmGreeting` composable и биндить приветствие в форму
- [x] 5.3 Стилизовать (CSS, без библиотек)
- [x] 5.4 Проверить: при загрузке поле автоматом заполняется "Hello World from Rust!"
