## MODIFIED Requirements

### Requirement: Цветовая схема биомов

**FROM:**
Система SHALL отображать каждый биом своим цветом для визуальной идентификации.
- Plains (0) → 0x7ec850
- Forest (1) → 0x2d5a27
- Water (2) → 0x3b82f6
- Mountain (3) → 0x8b7355

**TO:**
Система SHALL отображать каждый биом цветом из динамического конфига биомов, полученного из WASM или JS-конфига, используя поле `color` из определения биома.

#### Scenario: Цвет из конфига

- **WHEN** клетка имеет biomeId = N
- **THEN** цвет клетки берётся из `biomeDefinitions[N].color`, а не из захардкоженной таблицы

#### Scenario: Неизвестный biomeId

- **WHEN** biomeId отсутствует в biomeDefinitions
- **THEN** клетка заливается чёрным цветом (0x000000)
