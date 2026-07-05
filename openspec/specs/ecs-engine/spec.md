## Purpose

TBD — ECS-движок на базе bevy_ecs для управления игровым состоянием.

## Requirements

### Requirement: World с Entity-компонентной моделью

Система SHALL использовать bevy_ecs World как единственное хранилище игрового состояния. Все клетки карты — Entity с компонентами. Основные типы компонентов:
- `Position { x: u32, y: u32 }` — координаты клетки
- `BiomeId(u16)` — id биома из реестра (0 = первый биом из конфига)
- `Resources(u8)` — количество ресурсов на клетке

#### Scenario: Создание World

- **WHEN** вызывается `create_grid(seed, width, height)`
- **THEN** создаётся bevy_ecs World, и для каждой клетки спавнится Entity с компонентами Position, BiomeId, Resources

#### Scenario: Query клеток по биому

- **WHEN** система запрашивает все клетки с определённым BiomeId
- **THEN** bevy_ecs Query возвращает только соответствующие Entity

### Requirement: GridResource как точка входа

Система SHALL хранить `GridResource { width: u32, height: u32, entities: Vec<Entity> }` как bevy_ecs Resource для доступа к структуре карты.

#### Scenario: Доступ к GridResource

- **WHEN** системе нужны размеры карты или список всех Entity клеток
- **THEN** она получает их через `world.resource::<GridResource>()`

### Requirement: Генерация карты через системы

Система SHALL заменить `biome_id_from_lcg_value()` на noise-функцию, использующую Perlin noise (FBM) с двумя слоями (elevation + moisture). Система генерации SHALL принимать `GenerationParams` для настройки параметров шума и порогов маппинга. Генерация SHALL использовать `Commands::spawn()` для создания Entity.

#### Scenario: Система генерации с noise

- **WHEN** World создан и инициализирован с `GenerationParams`
- **THEN** система создаёт Entity для каждой клетки, присваивая `BiomeId` на основе elevation/moisture noise и пороговых значений

#### Scenario: Детерминизм генерации

- **WHEN** две системы запускаются с одинаковым seed и параметрами
- **THEN** все Entity имеют идентичные `BiomeId` на соответствующих координатах
