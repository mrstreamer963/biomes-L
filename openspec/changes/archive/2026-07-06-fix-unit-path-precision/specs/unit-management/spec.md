## MODIFIED Requirements

### Requirement: Movement system

Система SHALL запускать movement_system каждый tick. Система SHALL для каждого юнита с MovementTarget(Some(target)):
1. Вычислить направление и расстояние до цели
2. Если расстояние < 2.0 px → установить MovementStatus.idling = true, очистить Target
3. Определить биом под текущей позицией юнита (col = (pos.x / TILE_SIZE).floor(), row = (pos.y / TILE_SIZE).floor())
4. Получить speed_factor из BiomeDefinitions
5. Если speed_factor == 0 (непроходимый биом) → установить idling = true, не двигаться
6. Иначе вычислить шаг = base_speed * speed_factor * dt
7. Проверить, что новая позиция не на непроходимом биоме: ncol = (nx / TILE_SIZE).floor(), nrow = (ny / TILE_SIZE).floor()
8. Если новая позиция проходима — обновить Position

**Изменение**: Формулы определения тайла по позиции изменены с `.round()` на `.floor()` — как в определении биома под юнитом (п.3), так и в проверке проходимости новой позиции (п.7). Это гарантирует, что юнит в позиции x ∈ [0, 32) считается находящимся в тайле col=0, а не col=1.

#### Scenario: Юнит движется к цели по Plains

- **WHEN** юнит Scout (BaseSpeed=140) на Plains (speed_factor=1.0) имеет MovementTarget(Some(100, 100)) и Position(50, 50)
- **WHEN** tick(handle, 1.0) вызван
- **THEN** Position приблизился к (100, 100) на 140 px

#### Scenario: Юнит останавливается перед Water

- **WHEN** юнит движется к цели за Water
- **WHEN** новая позиция попадает на Water (passable=false)
- **THEN** MovementStatus.idling = true, Position не меняется

#### Scenario: Юнит останавливается у цели

- **WHEN** юнит в 1 px от MovementTarget
- **WHEN** tick(handle, dt) вызван
- **THEN** MovementTarget = None, MovementStatus.idling = true

#### Scenario: Биом на границе тайла определяется корректно

- **WHEN** юнит в позиции (15.5, 15.5) — центр левой половины тайла (0, 0)
- **WHEN** tick(handle, dt) вызван
- **THEN** биом определяется как тайл col = floor(15.5/32) = 0, row = floor(15.5/32) = 0
