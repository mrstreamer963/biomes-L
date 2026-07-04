## Context

Заготовка проекта Biomes. Сейчас — пустой репозиторий с идеями и OpenSpec. Нужно создать минимальную работающую связку: Vue интерфейс, PixiJS рендер, игровая логика на Rust в WASM/WebWorker.

## Goals / Non-Goals

**Goals:**
- Корневой Cargo.toml (workspace) + корневой package.json
- Rust крейт `engine/`, компилируемый в WASM через wasm32-unknown-unknown + wasm-bindgen
- WebWorker, загружающий WASM и вызывающий greet()
- Vue 3 приложение с авто-заполняемой формой приветствия
- PixiJS v8 Application в Vue-компоненте
- Скрипт сборки WASM (`scripts/build-wasm.sh`) под new structure

**Non-Goals:**
- Bevy ECS или любая игровая логика — только greet()
- Роутинг, stores (Pinia), i18n — только минимальный Vue
- Ассеты, спрайты для PixiJS — только цветной фон/круг
- SharedWorker — пока обычный WebWorker

## Decisions

### 1. Структура монорепозитория

Корневые Cargo.toml (workspace) и package.json — вместо вложенных packages/. Это упрощает CI и сборку.

```
biomes-L/
├── Cargo.toml          → workspace.members = ["engine"]
├── package.json        → Vite проект
├── scripts/
├── engine/             → Rust крейт
└── src/                → Vue/PixiJS
    ├── worker/
    └── wasm/           → сгенерировано build-wasm.sh
```

### 2. Raw wasm32 + wasm-bindgen вместо wasm-pack

wasm-pack удобен, но даёт меньше контроля. Используем:
- `cargo build --target wasm32-unknown-unknown`
- `wasm-bindgen --target web` для генерации JS-прокси

Это уже заложено в `scripts/build-wasm.sh`.

### 3. Vite inline worker

WebWorker создаётся через `new Worker(new URL('./worker/game.worker.ts', import.meta.url), { type: 'module' })`. Vite собирает воркер как отдельный chunk.

### 4. Типизированный bridge

Прокси-обёртка `bridge/wasm.ts` с type-защитой сообщений. Воркер и main thread общаются через discriminated union:

```ts
type WorkerMessage = { type: "greeting"; text: string }
type MainMessage = { type: "greet"; name?: string }
```

### 5. PixiJS v8 — импорт без конфигурации

PixiJS v8 — ES модуль, подключается напрямую. Canvas монтируется в `GameCanvas.vue` через `onMounted`.

## Risks / Trade-offs

| Риск | Митигация |
|------|-----------|
| `wasm-bindgen` не подружится с Vite worker | Использовать `--target web`, импорт через динамический import() внутри worker |
| PixiJS v8 может иметь breaking changes vs v7 | Фиксируем версию в package.json; документация PixiJS 8 уже стабильна |
| Корневой Cargo.toml может конфликтовать с нейминговыми тулами | Добавить `./target` в `.gitignore` |
| Vite может не собрать .wasm в worker | Тестируем; fallback — грузить WASM через fetch() и instantiate вручную |
