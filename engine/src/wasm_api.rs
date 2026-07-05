use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::ecs::*;
use crate::noise;

static NEXT_HANDLE: AtomicU32 = AtomicU32::new(1);

struct GridStore {
    grid: GridResource,
    definitions: Option<BiomeDefinitions>,
}

thread_local! {
    static STORE: RefCell<HashMap<u32, GridStore>> = RefCell::new(HashMap::new());
}

const BIOME_NONE: u8 = 255;

#[wasm_bindgen]
pub fn create_grid(params: JsValue, width: u32, height: u32) -> u32 {
    let params: GenerationParams = serde_wasm_bindgen::from_value(params)
        .expect("failed to deserialize GenerationParams");
    let elevation_noise = noise::Noise::new(params.seed as i32);
    let moisture_noise = noise::Noise::new(params.seed.wrapping_add(1000) as i32);

    let len = (width as usize) * (height as usize);
    let mut biome_ids = Vec::with_capacity(len);
    let resources = vec![0u8; len];

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 / width.max(1) as f64;
            let ny = y as f64 / height.max(1) as f64;
            let elevation = (elevation_noise.fbm(
                nx, ny,
                params.octaves, params.lacunarity, params.persistence,
                params.scale,
            ) + 1.0) / 2.0;
            let moisture = (moisture_noise.fbm(
                nx, ny,
                params.octaves, params.lacunarity, params.persistence,
                params.scale,
            ) + 1.0) / 2.0;
            biome_ids.push(biome_from_noise(elevation, moisture, &params));
        }
    }

    let handle = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    STORE.with(|s| {
        s.borrow_mut().insert(
            handle,
            GridStore {
                grid: GridResource::new(width, height, biome_ids, resources),
                definitions: None,
            },
        )
    });
    handle
}

#[wasm_bindgen]
pub fn register_biome_definitions(handle: u32, definitions: JsValue) -> bool {
    STORE.with(|s| {
        let mut s = s.borrow_mut();
        let Some(store) = s.get_mut(&handle) else {
            return false;
        };
        match serde_wasm_bindgen::from_value::<Vec<BiomeDef>>(definitions) {
            Ok(defs) => {
                store.definitions = Some(BiomeDefinitions::new(defs));
                true
            }
            Err(_) => false,
        }
    })
}

#[wasm_bindgen]
pub fn biome_definitions(handle: u32) -> JsValue {
    STORE.with(|s| {
        let s = s.borrow();
        let Some(store) = s.get(&handle) else {
            return JsValue::NULL;
        };
        match &store.definitions {
            Some(defs) => {
                serde_wasm_bindgen::to_value(&defs.definitions).unwrap_or(JsValue::NULL)
            }
            None => JsValue::NULL,
        }
    })
}

#[wasm_bindgen]
pub fn grid_snapshot(handle: u32) -> JsValue {
    STORE.with(|s| {
        let s = s.borrow();
        let Some(store) = s.get(&handle) else {
            return JsValue::NULL;
        };
        let grid = &store.grid;

        let biome_arr = js_sys::Uint16Array::new_with_length(grid.biome_ids.len() as u32);
        biome_arr.copy_from(&grid.biome_ids);

        let res_arr = js_sys::Uint8Array::new_with_length(grid.resources.len() as u32);
        res_arr.copy_from(&grid.resources);

        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"width".into(), &(grid.width as f64).into()).unwrap();
        js_sys::Reflect::set(&obj, &"height".into(), &(grid.height as f64).into()).unwrap();
        js_sys::Reflect::set(&obj, &"biomeIds".into(), &biome_arr.into()).unwrap();
        js_sys::Reflect::set(&obj, &"resources".into(), &res_arr.into()).unwrap();
        obj.into()
    })
}

#[wasm_bindgen]
pub fn cell_at(handle: u32, x: u32, y: u32) -> JsValue {
    STORE.with(|s| {
        let s = s.borrow();
        let Some(store) = s.get(&handle) else {
            return cell_obj(BIOME_NONE, 0);
        };
        let grid = &store.grid;
        if x >= grid.width || y >= grid.height {
            return cell_obj(BIOME_NONE, 0);
        }
        let idx = (y as usize) * (grid.width as usize) + (x as usize);
        let biome = grid.biome_ids.get(idx).copied().unwrap_or(0) as u8;
        let res = grid.resources.get(idx).copied().unwrap_or(0);
        cell_obj(biome, res)
    })
}

fn cell_obj(biome: u8, resources: u8) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"biome".into(), &(biome as f64).into()).unwrap();
    js_sys::Reflect::set(&obj, &"resources".into(), &(resources as f64).into()).unwrap();
    obj.into()
}
