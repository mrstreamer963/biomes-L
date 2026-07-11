## 1. Portal Vertices Helper

- [x] 1.1 Add `portal_vertices(c1, r1, dc, dr) -> ((f64, f64), (f64, f64))` в `pathfinding.rs`
- [x] 1.2 Написать модульные тесты для `portal_vertices` (8 направлений, проверка left/right)

## 2. Funnel Algorithm

- [x] 2.1 Добавить `funnel_algorithm(cells, start, end) -> Vec<(f64, f64)>` в `pathfinding.rs`
- [x] 2.2 Написать тесты для `funnel_algorithm`:
  - [x] 2.2.1 Прямой путь — без дополнительных waypoint'ов
  - [x] 2.2.2 L-образный путь — прямая линия проходима (SSFA корректно определяет)
  - [x] 2.2.3 S-образный путь — прямая линия проходима
  - [x] 2.2.4 Диагональный путь — без дополнительных waypoint'ов
  - [x] 2.2.5 Путь из 1 и 2 клеток — без пост-процессинга

## 3. Интеграция в WASM API

- [x] 3.1 Изменить импорт в `wasm_api.rs`: заменить `tile_to_pixel` на `funnel_algorithm`
- [x] 3.2 В `set_unit_target`: применить `funnel_algorithm` после A*
- [x] 3.3 Установить `MovementTarget` на точную позицию клика `(x, y)`

## 4. Проверка и сборка

- [x] 4.1 Запустить `cargo test` — все тесты проходят
- [x] 4.2 Проверить сборку WASM: `cargo build --target wasm32-unknown-unknown`