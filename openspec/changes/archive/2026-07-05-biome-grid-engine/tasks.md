## 1. Rust: доменные типы

- [x] 1.1 Создать модуль `engine/src/biome.rs`: `#[repr(u8)] pub enum Biome { Plains=0, Forest=1, Water=2, Mountain=3 }` с `as u8`.
- [x] 1.2 Реализовать `Biome::passable() -> bool` и `Biome::speed_factor() -> f32` через `match` (Plains=1.0, Forest=0.6, Water/Mountain — `passable=false`).
- [x] 1.3 Создать модуль `engine/src/grid.rs`: `pub struct Cell { pub biome: Biome, pub resources: u8 }` (default `resources=0`).
- [x] 1.4 Реализовать `pub struct Grid { width: u32, height: u32, cells: Vec<Cell> }` с индексацией `(x,y) -> y*width+x`.
- [x] 1.5 Реализовать `Grid::cell_at(x,y) -> Option<&Cell>` (возврат `None` вне границ, без паники).

## 2. Rust: генератор карты

- [x] 2.1 Создать модуль `engine/src/rng.rs`: простая LCG-структура `pub struct Lcg { state: u64 }` с `next_u32()` (константы из детерминированного набора).
- [x] 2.2 Реализовать `Grid::generate(seed: u64, width: u32, height: u32) -> Grid`: инициализировать LCG seed'ом, для каждой клетки получить значение и выбрать биом по порогам (Plains/Forest/Water/Mountain).
- [x] 2.3 Юнит-тест в `engine/`: одинаковое `(seed,width,height)` → идентичные биомы на тех же координатах (воспроизводимость); разные seed → разные распределения.

## 3. Rust: WASM API

- [x] 3.1 В `engine/src/lib.rs` (или `wasm_api.rs`) реализовать `thread_local` хранилище `HashMap<u32, Grid>` с атомарным счётчиком дескрипторов.
- [x] 3.2 Реализовать `#[wasm_bindgen] pub fn create_grid(seed: u64, width: u32, height: u32) -> u32` — генерация, регистрация, возврат дескриптора.
- [x] 3.3 Реализовать `#[wasm_bindgen] pub fn grid_snapshot(handle: u32) -> JsValue` (или объект с полями): `{ width, height, biomes: Uint8Array }`; для неизвестного дескриптора — null/пустой результат без паники.
- [x] 3.4 Реализовать `#[wasm_bindgen] pub fn cell_at(handle: u32, x: u32, y: u32) -> JsValue`: `{ biome: u8, resources: u8 }`, для координат вне границ `biome=255`.
- [x] 3.5 Удалить `#[wasm_bindgen] pub fn greet()`.
- [x] 3.6 `cargo test` (нативный target) проходит; `cargo build --target wasm32-unknown-unknown` (или через `scripts/build-wasm.sh`) собирается без ошибок.

## 4. Сборка WASM и типы

- [x] 4.1 Запустить `scripts/build-wasm.sh` (dev), убедиться что `src/wasm/engine.js` и `engine_bg.wasm` обновились.
- [x] 4.2 Обновить `src/wasm/engine.d.ts`: убрать `greet`, добавить сигнатуры `create_grid`, `grid_snapshot`, `cell_at` с корректными TS-типами (`Uint8Array`/объекты).

## 5. Worker и bridge

- [x] 5.1 В `src/worker/game.worker.ts`: после `init()` вызвать `create_grid(default_seed, default_width, default_height)` (константы из модуля), затем `grid_snapshot`, отправить `postMessage({ type: "grid-snapshot", width, height, biomes })`.
- [x] 5.2 Убрать из worker'а вызов `greet` и сообщение `greeting`.
- [x] 5.3 В `src/bridge/wasm.ts`: добавить к `WorkerMessage` вариант `{ type: "grid-snapshot"; width: number; height: number; biomes: Uint8Array }`; убрать вариант `greeting`.
- [x] 5.4 Сохранить путь для ошибок: при провале генерации отправлять `{ type: "status", status: "error", error }`.

## 6. Vue-сторона

- [x] 6.1 Переименовать/перепрофилировать `src/composables/useWasmGreeting.ts` → `useGridSnapshot.ts` (или аналог): реактивно хранить `width`, `height`, `biomes`, `status`.
- [x] 6.2 Обновить `src/components/StatusPanel.vue`: вместо поля greeting показывать сводку сетки (размер, количество биомов по типам).
- [x] 6.3 Проверить, что `App.vue` layout не сломан; убрать импорт старого composable, где более не нужен.

## 7. Проверка

- [x] 7.1 `npm run dev` запускается без ошибок сборки/типов.
- [x] 7.2 В браузере: worker сообщает `ready`, StatusPanel показывает сводку сетки (ненулевые счётчики биомов), размеры совпадают с константами.
- [x] 7.3 Воспроизводимость: перезагрузка страницы даёт ту же карту (тот же seed).
- [x] 7.4 `vue-tsc` (`npm run build`-этап типов) проходит без ошибок.
