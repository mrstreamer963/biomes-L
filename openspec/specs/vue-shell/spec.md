## Purpose

Пользовательский интерфейс на Vue 3 для взаимодействия с WASM-движком.

## Requirements

### Requirement: Отображение статуса воркера

Система SHALL показать статус WebWorker (загружается / готов / ошибка).

#### Scenario: Статус "загружается"

- **WHEN** страница только открыта
- **THEN** отображается "Подключение к игровому движку..."

#### Scenario: Статус "готов"

- **WHEN** worker отправил status: "ready"
- **THEN** индикатор сменяется на "Движок готов"

### Requirement: Отображение биомов из конфига

Система SHALL отображать список биомов из `biomeDefinitions` (импорт из `biomes.json` через `biomeConfig.ts`).

#### Scenario: Список биомов из конфига

- **WHEN** snapshot получен
- **THEN** StatusPanel показывает строку для каждого биома: название, цвет, количество клеток

#### Scenario: Дефолтный конфиг — 7 биомов

- **WHEN** используется дефолтный `biomes.json`
- **THEN** StatusPanel показывает 7 строк

### Requirement: Макет страницы

Форма слева (~30%), PixiJS canvas справа (~70%).

### Requirement: Reactive состояние

`useGridSnapshot` SHALL предоставлять: `width`, `height`, `biomeIds`, `biomeDefinitions`, `biomeCounts`, `units`, `selectedUnitId`, `hoveredCell`, `status`, `error`.

#### Scenario: units ref обновляется

- **WHEN** приходит `{ type: "unit-snapshot", units }`
- **THEN** `units.value` обновляется

### Requirement: Game loop на UI-треде

После status: "ready" система SHALL запустить `requestAnimationFrame` на main thread:
1. Вычислить dt
2. Отправить `{ type: "tick", dt }` в worker
3. Обновить units при получении unit-snapshot

#### Scenario: rAF запущен

- **WHEN** worker готов
- **THEN** main thread отправляет tick ~60 FPS

### Requirement: Остановка game loop

При размонтировании: cancelAnimationFrame, worker.terminate().

### Requirement: Глобальный debug-флаг

Чекбокс Debug включает/выключает waypoints у всех юнитов через `set-unit-debug`.

### Requirement: hoveredCell

Реактивное `{ col, row } | null` — координаты тайла под курсором.

#### Scenario: Показ координат в StatusPanel

- **WHEN** `hoveredCell` не null
- **THEN** отображается "Cell: {col}, {row}"

### Requirement: Обработка кликов по карте

Клик (без drag > 5px): hit-test юнита → выделение; клик по пустой клетке с выделенным юнитом → `sendMoveCommand`.

#### Scenario: Drag не вызывает движение

- **WHEN** pointerdown + перемещение > 5px + pointerup
- **THEN** выполняется pan, `sendMoveCommand` не вызывается
