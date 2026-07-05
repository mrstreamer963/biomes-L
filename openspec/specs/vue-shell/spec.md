## Purpose

Обеспечить пользовательский интерфейс на Vue 3 для взаимодействия с WASM-движком. TBD — цели будут уточнены в процессе разработки.

## Requirements

### Requirement: Отображение статуса воркера
Система SHALL показать статус WebWorker (загружается/готов/ошибка) на форме.

#### Scenario: Статус "загружается"
- **WHEN** страница только открыта
- **THEN** отображается индикатор "Подключение к игровому движку..."

#### Scenario: Статус "готов"
- **WHEN** worker инициализирован и WASM загружен
- **THEN** индикатор сменяется на "Движок готов"

#### Scenario: Статус "ошибка"
- **WHEN** worker не смог загрузить WASM
- **THEN** отображается сообщение об ошибке

### Requirement: Отображение биомов из конфига
Система SHALL отображать список биомов и их количество из динамического конфига, а не из захардкоженного списка.

#### Scenario: Список биомов из конфига
- **WHEN** приложение загружено и snapshot получен
- **THEN** StatusPanel показывает строку для каждого биома из `biomeDefinitions`, с его названием, цветом и количеством клеток

#### Scenario: Изменение конфига
- **WHEN** biomeDefinitions содержит 5 биомов
- **THEN** StatusPanel показывает 5 строк (а не 4)

### Requirement: Макет страницы
Система SHALL отобразить форму слева и PixiJS canvas справа, с минимальной стилизацией (CSS, без UI-библиотек).

#### Scenario: Две колонки
- **WHEN** приложение загружено
- **THEN** форма занимает ~30% ширины слева, canvas — ~70% справа

### Requirement: Reactive состояние юнитов

Система SHALL предоставить реактивное состояние `units` в `useGridSnapshot` composable, обновляемое при получении `unit-snapshot`.

#### Scenario: units ref обновляется

- **WHEN** main thread получает `{ type: "unit-snapshot", units: [...] }`
- **THEN** `useGridSnapshot().units.value` содержит актуальный массив юнитов

### Requirement: Game loop на UI-треде

Система SHALL запустить requestAnimationFrame цикл на UI-треде после инициализации. Каждый кадр:
1. Вычислить dt = (now - lastTime) / 1000
2. Отправить `{ type: "tick", dt }` в Worker
3. После получения `unit-snapshot` — обновить состояние

#### Scenario: rAF запущен

- **WHEN** приложение готово (worker отправил status: "ready")
- **THEN** запускается requestAnimationFrame цикл, отправляющий tick каждые ~16ms

### Requirement: Остановка game loop

Система SHALL остановить game loop при демонтировании компонента.

#### Scenario: clean up

- **WHEN** компонент размонтирован
- **THEN** rAF отменён, worker завершён
