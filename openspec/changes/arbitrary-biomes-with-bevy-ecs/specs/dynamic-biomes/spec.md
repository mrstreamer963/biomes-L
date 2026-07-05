## ADDED Requirements

### Requirement: Реестр определений биомов

Система SHALL предоставить Resource `BiomeDefinitions` в bevy_ecs World, содержащий произвольный список определений биомов. Каждое определение включает:
- `name: String` — человекочитаемое имя
- `passable: bool` — проходимость
- `speed_factor: f32` — множитель скорости (0.0 для непроходимых)
- `color: u32` — hex-цвет для рендеринга (0xRRGGBB)

#### Scenario: Регистрация биомов

- **WHEN** вызывается `register_biome_definitions(handle, definitions)` с массивом из JS
- **THEN** BiomeDefinitions Resource обновляется, сохраняя порядок определений (индекс в массиве = BiomeId)

#### Scenario: Обновление определений

- **WHEN** вызывается `register_biome_definitions` с новым массивом
- **THEN** BiomeDefinitions Resource заменяется полностью

### Requirement: Доступ к свойствам биома по BiomeId

Система SHALL предоставить методы на BiomeDefinitions для получения свойств биома по id:
- `get_name(id) -> Option<&str>`
- `is_passable(id) -> bool` (false для неизвестного id)
- `speed_factor(id) -> f32` (0.0 для неизвестного id)
- `get_color(id) -> u32` (0x000000 для неизвестного id)

#### Scenario: Получение свойств

- **WHEN** система вызывает `biome_defs.get_name(0)`
- **THEN** возвращается имя первого биома из зарегистрированного списка

### Requirement: JS-конфиг биомов

Система SHALL предоставить JS-константу `DEFAULT_BIOME_DEFINITIONS` в отдельном файле `src/config/biomeConfig.ts`, содержащую массив определений биомов, которые передаются в WASM при инициализации через `register_biome_definitions`.

#### Scenario: Дефолтный конфиг

- **WHEN** приложение стартует
- **THEN** biomeConfig.ts экспортирует массив с теми же 4 биомами (Plains, Forest, Water, Mountain) с их текущими свойствами и цветами
