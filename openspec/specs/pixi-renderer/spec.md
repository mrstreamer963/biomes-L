## Purpose

Обеспечить рендеринг игровой графики через PixiJS v8 внутри Vue-приложения. TBD — цели будут уточнены в процессе разработки.

## Requirements

### Requirement: Инициализация PixiJS v8
Система SHALL инициализировать PIXI.Application v8 внутри Vue-компонента `GameCanvas.vue` при монтировании.

#### Scenario: Создание Application
- **WHEN** компонент `GameCanvas.vue` смонтирован
- **THEN** создаётся PIXI.Application с настройками по умолчанию и добавляется в DOM

### Requirement: Демо-рендер
Система SHALL отобразить демо-графику (цветной круг) на PixiJS canvas после инициализации.

#### Scenario: Отрисовка круга
- **WHEN** PIXI.Application инициализирован
- **THEN** на сцене отображается как минимум один графический примитив (круг/прямоугольник)

### Requirement: Canvas встраивается в layout
Система SHALL разместить PixiJS canvas внутри Vue-компонента с адаптивным размером.

#### Scenario: Canvas занимает контейнер
- **WHEN** `GameCanvas.vue` отрендерен
- **THEN** canvas занимает 100% ширины и высоты родительского контейнера
