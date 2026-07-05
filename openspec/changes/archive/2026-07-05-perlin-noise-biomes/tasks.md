## 1. Rust: Perlin noise implementation

- [x] 1.1 Создать `engine/src/noise.rs` с реализацией `perlin_2d(x: f64, y: f64, seed: i32) -> f64`, включая permutation table на 256 элементов, инициализируемую из seed
- [x] 1.2 Реализовать `fbm(x: f64, y: f64, octaves: u32, lacunarity: f64, persistence: f64, scale: f64, seed: i32) -> f64` с нормализацией результата к `[-1, 1]`
- [x] 1.3 Добавить модуль `noise` в `engine/src/lib.rs` (pub mod noise)
- [x] 1.4 Написать unit-тесты: детерминизм, zero в (0,0), FBM с 1 октавой = perlin_2d

## 2. Rust: Generation logic

- [x] 2.1 Создать структуру `GenerationParams` в `engine/src/ecs/mod.rs`, с полями: seed, scale, octaves, persistence, lacunarity, elevation_low, elevation_high, moisture_high и дефолтными значениями
- [x] 2.2 Реализовать `biome_from_noise(elevation: f64, moisture: f64, thresholds: &GenerationParams) -> u16` — маппинг двух noise-слоёв на biome_id
- [x] 2.3 Заменить `biome_id_from_lcg_value` на новую логику в `ecs/mod.rs`, оставив старую функцию помеченной `#[deprecated]` для обратной совместимости тестов
- [x] 2.4 Обновить тест `generate_is_reproducible` для проверки детерминизма noise-генерации
- [x] 2.5 Удалить старый тест `biome_id_mapping` (LCG-специфичный)

## 3. Rust: WASM API integration

- [x] 3.1 Обновить сигнатуру `create_grid` в `wasm_api.rs`: `create_grid(params: JsValue, width: u32, height: u32) -> u32`, десериализуя `GenerationParams` из JsValue
- [x] 3.2 Реализовать в `create_grid` генерацию двух noise-слоёв (elevation, moisture) через `fbm` и маппинг через `biome_from_noise`
- [x] 3.3 Проверить, что `grid_snapshot` и `cell_at` работают без изменений

## 4. TypeScript: types and config

- [x] 4.1 Обновить `src/wasm/engine.d.ts`: изменить тип `create_grid` на `create_grid(params: GenerationParams, width: number, height: number) => number`, добавить интерфейс `GenerationParams`
- [x] 4.2 Добавить интерфейс `GenerationParams` в `engine.d.ts` с полями: seed (bigint), scale, octaves, persistence, lacunarity, elevationLow, elevationHigh, moistureHigh (все number, опциональные)
- [x] 4.3 Добавить дефолтный конфиг генерации в `biomeConfig.ts` — экспортировать `DEFAULT_GENERATION_PARAMS`

## 5. TypeScript: worker integration

- [x] 5.1 Обновить `src/worker/game.worker.ts`: передавать `GenerationParams` в сообщении `init`, использовать их при вызове `create_grid`
- [x] 5.2 Обновить `src/bridge/wasm.ts`: добавить поле `generationParams` в тип `MainMessage`, передавать из main thread в worker

## 6. Build and verify

- [x] 6.1 Собрать WASM: запустить `scripts/build-wasm.sh`, убедиться в отсутствии ошибок
- [x] 6.2 Запустить приложение (`npm run dev`), проверить что карта отображается с пространственно-когерентными биомами (остаётся вручную)
- [x] 6.3 Запустить Rust-тесты: `cargo test -p engine`, убедиться что все тесты проходят
