## MODIFIED Requirements

### Requirement: Детерминированная генерация карты из seed

Система SHALL генерировать сетку биомов детерминированно из числового `seed`, `GenerationParams`, `width` и `height`. Один и тот же набор `(seed, GenerationParams, width, height)` SHALL давать идентичную карту. Генерация SHALL использовать Perlin noise (FBM) вместо LCG для определения биома каждой клетки.

#### Scenario: Воспроизводимость карты
- **WHEN** генерируются две сетки с одинаковыми `seed`, `GenerationParams`, `width`, `height`
- **THEN** биомы на одинаковых координатах в обеих сетках совпадают

#### Scenario: Пространственная когерентность
- **WHEN** генерируется сетка с noise-генерацией
- **THEN** соседние клетки имеют одинаковый биом чаще, чем при случайном распределении (коэффициент более 0.3 против ~0.25 для случайного)

### Requirement: WASM API для создания и получения сетки

Система SHALL предоставлять WASM-функцию `create_grid(params: GenerationParams, width: u32, height: u32)`, возвращающую числовой дескриптор сетки. `GenerationParams` включает seed, scale, octaves и пороговые значения для маппинга шума на биомы.

#### Scenario: Создание сетки с параметрами
- **WHEN** вызывается `create_grid({ seed: 42, scale: 8.0, octaves: 4 }, width=8, height=8)`
- **THEN** возвращается ненулевой числовой дескриптор

#### Scenario: Обратная совместимость snapshot
- **WHEN** вызывается `grid_snapshot(handle)` для сетки, созданной с noise
- **THEN** возвращается объект с `width`, `height` и массивом байтов, где каждое значение — корректный индекс биома (0..=3)
