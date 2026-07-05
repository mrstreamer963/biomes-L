## MODIFIED Requirements

### Requirement: Генерация карты через системы

Система SHALL заменить `biome_id_from_lcg_value()` на noise-функцию, использующую Perlin noise (FBM) с двумя слоями (elevation + moisture). Система генерации SHALL принимать `GenerationParams` для настройки параметров шума и порогов маппинга.

#### Scenario: Система генерации с noise
- **WHEN** World создан и инициализирован с `GenerationParams`
- **THEN** система создаёт Entity для каждой клетки, присваивая `BiomeId` на основе elevation/moisture noise и пороговых значений

#### Scenario: Детерминизм генерации
- **WHEN** две системы запускаются с одинаковым seed и параметрами
- **THEN** все Entity имеют идентичные `BiomeId` на соответствующих координатах
