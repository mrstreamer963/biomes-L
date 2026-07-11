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

### Requirement: Глобальный debug-флаг

Система SHALL предоставить чекбокс Debug, который включает/выключает показ waypoints у всех юнитов сразу, без необходимости выделять юнит.

#### Scenario: Включение debug без выделения

- **WHEN** пользователь включает чекбокс Debug без выделенного юнита
- **THEN** waypoints отображаются у всех юнитов

#### Scenario: Выключение debug

- **WHEN** пользователь выключает чекбокс Debug
- **THEN** waypoints скрываются у всех юнитов

#### Scenario: Чекбокс активен всегда

- **WHEN** пользователь открыл приложение
- **THEN** чекбокс Debug доступен для нажатия, независимо от наличия выбранного юнита

### Requirement: Reactive состояние координат курсора

Система SHALL предоставить реактивное свойство `hoveredCell` в `useGridSnapshot` composable, содержащее координаты тайла под курсором мыши или `null`, если курсор вне сетки.

#### Scenario: hoveredCell обновляется при движении мыши

- **WHEN** мышь движется над canvas
- **THEN** `hoveredCell.value` содержит `{ col, row }` с актуальными координатами тайла

#### Scenario: hoveredCell сбрасывается при pointerleave

- **WHEN** мышь покидает canvas (pointerleave)
- **THEN** `hoveredCell.value` равен `null`

### Requirement: Отображение координат тайла в панели статуса

Система SHALL отображать координаты тайла (col, row) под курсором мыши в StatusPanel.vue.

#### Scenario: Показ координат

- **WHEN** `hoveredCell` не null
- **THEN** в StatusPanel отображается "Cell: {col}, {row}"

#### Scenario: Скрытие при null

- **WHEN** `hoveredCell` равен null
- **THEN** строка с координатами не отображается (отображается "—")

### Requirement: Обработка кликов по карте

Система SHALL обрабатывать клики по PixiJS canvas для определения действий игрока:
1. При клике без drag — определить мировые координаты: worldX = (screenX - container.position.x) / container.scale.x
2. Выполнить hit-test по юнитам в мировых координатах
3. Если клик по юниту — выделить его
4. Если клик по пустой клетке и есть выделенный юнит — отправить команду движения
5. При drag (перемещение > 5px между pointerdown и pointerup) — не считать кликом, выполнять pan

**Изменение**: Удалён дублирующийся `canvas.addEventListener('click', ...)`. Обработка клика выполняется только через `MapCamera.handlePointerUp`, который отличает drag от click через флаг `wasDrag`. Это устраняет двойной вызов `sendMoveCommand` при каждом клике.

#### Scenario: Клик по юниту выделяет его

- **WHEN** пользователь кликает (pointerdown + pointerup без перемещения) по юниту
- **THEN** юнит выделяется (selectedUnitId устанавливается)

#### Scenario: Клик по пустой клетке двигает юнит

- **WHEN** пользователь кликает по пустой клетке
- **WHEN** есть выделенный юнит
- **THEN** `sendMoveCommand` вызывается ровно один раз

#### Scenario: Drag не вызывает команду движения

- **WHEN** пользователь делает pointerdown, перемещает мышь > 5px, затем pointerup
- **THEN** `sendMoveCommand` НЕ вызывается, выполняется pan камеры
