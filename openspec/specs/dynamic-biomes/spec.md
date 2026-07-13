## Purpose

TBD — управление произвольным набором определений биомов из JS-конфига.

## Requirements

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

### Requirement: Конфиг биомов

Система SHALL предоставить единый файл `src/config/biomes.json` как источник истины для:
- определений биомов (`definitions`), каждое из которых включает свойства (name, passable, speed, color) и опциональные условия генерации (`generation`)
- параметров шума (`generationParams`: seed, scale, octaves, persistence, lacunarity)

Условия генерации задаются в том же объекте биома через поле `generation` с опциональными порогами (elevationLt, elevationGt, elevationGte, moistureGt). Порядок проверки правил совпадает с порядком биомов в массиве `definitions` (индекс 0 проверяется первым). Биом с пустым `generation: {}` срабатывает как fallback, когда до него дошла очередь.

TypeScript импортирует JSON через `src/config/biomeConfig.ts` и передаёт определения в WASM при инициализации через `register_biome_definitions`. Rust читает тот же JSON через `include_str!` в `engine/src/ecs/biome_config.rs`.

#### Scenario: Дефолтный конфиг

- **WHEN** приложение стартует
- **THEN** `biomes.json` содержит 7 биомов с их свойствами, цветами и условиями генерации, и TS/Rust используют одинаковые данные из этого файла

#### Scenario: Добавление биома

- **WHEN** в `biomes.json` добавляется одна запись в `definitions` с полем `generation`
- **THEN** новый биом появляется на карте без изменения Rust-кода
