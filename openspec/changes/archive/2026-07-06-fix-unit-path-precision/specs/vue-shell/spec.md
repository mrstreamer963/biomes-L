## MODIFIED Requirements

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
