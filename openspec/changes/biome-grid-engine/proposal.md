## Why

Сейчас движок — это демо: Rust отдаёт `greet()`, а PixiJS рисует круг. Игровой логики нет, и `ideas.md` начинается с биомов как фундамента всей игры (проходимость, скорость, ресурсы). Чтобы двигаться к юнитам, командам и алертам, нужен первый реальный игровой объект — сетка биомов в Rust-движке, доступная из main thread через WASM.

## What Changes

- Вводится тип `Biome` (вид биома: например, `Plains`, `Forest`, `Water`, `Mountain`) с атрибутами: проходимость (`passable`) и множитель скорости прохождения (`speed_factor`).
- Вводится тип `Cell`, хранящий биом клетки и (зарезервированное) поле ресурсов для добычи.
- Вводится структура `Grid` (двумерная сетка ячеек фиксированного размера `width × height`).
- Добавляется генератор карты: детерминированная генерация из seed (псевдослучайное распределение биомов по клеткам).
- **BREAKING**: `greet()` удаляется из публичного WASM-API (демо-функция больше не нужна); worker вместо приветствия возвращает snapshot сетки.
- Добавляются WASM-функции: `create_grid(seed, width, height)` → дескриптор; `grid_snapshot(handle)` → сериализованное представление сетки для main thread; `cell_at(handle, x, y)` → данные клетки.
- WebWorker при `init` создаёт сетку по умолчанию и отправляет snapshot в main thread.

## Capabilities

### New Capabilities
- `biome-grid`: структура игровой сетки биомов в Rust-движке — типы биомов, клетки, сетка, генерация из seed и доступ к данным клетки.

### Modified Capabilities
- `wasm-worker`: worker при `init` больше не вызывает `greet()`; вместо этого создаёт сетку по умолчанию и отправляет snapshot сетки как сообщение `grid-snapshot`. Удаляется зависимость worker'а от функции `greet`.

## Impact

- **Rust-движок** (`engine/src/lib.rs`): замена `greet()` на типы и функции сетки; добавление генератора. Рост сложности crate'а `engine`.
- **WASM-связка** (`src/wasm/engine.d.ts`, пересобирается из `engine.js`): новый TypeScript-типы для grid-API; `greet` исчезает из биндингов.
- **Worker** (`src/worker/game.worker.ts`): логика `init` меняется — вызов `create_grid` + `grid_snapshot` вместо `greet`.
- **Bridge** (`src/bridge/wasm.ts`): расширяется тип `WorkerMessage` сообщением `grid-snapshot`; `MainMessage` остаётся.
- **Vue** (`StatusPanel.vue`, `useWasmGreeting.ts`): приветствие заменяется на отображение сводки сетки (размер, количество биомов); composable переименовывается/перепрофилируется.
- **Сборка**: `scripts/build-wasm.sh` без изменений (crate-type `cdylib` уже настроен);新增 типы не требуют новых зависимостей Rust на этом этапе (генератор на `rand`-аналоге внутри crate'а или ручной LCG, чтобы не тянуть `rand`).
- **Спецификации**: новый spec `biome-grid`; delta к `wasm-worker`.
