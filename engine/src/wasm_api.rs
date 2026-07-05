use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use bevy_ecs::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::ecs::*;
use crate::rng::Lcg;

static NEXT_HANDLE: AtomicU32 = AtomicU32::new(1);

thread_local! {
    static WORLDS: RefCell<HashMap<u32, World>> = RefCell::new(HashMap::new());
}

const BIOME_NONE: u8 = 255;

#[wasm_bindgen]
pub fn create_grid(seed: u64, width: u32, height: u32) -> u32 {
    let mut world = World::new();
    let mut rng = Lcg::new(seed);

    let mut entities = Vec::with_capacity((width as usize) * (height as usize));

    for y in 0..height {
        for x in 0..width {
            let v = rng.next_u32() & 0xFF;
            let biome_id = match v {
                0..=63 => 2u16,
                64..=159 => 3u16,
                160..=231 => 1u16,
                _ => 0u16,
            };
            let entity = world
                .spawn((Position { x, y }, BiomeId(biome_id), Resources(0)))
                .id();
            entities.push(entity);
        }
    }

    world.insert_resource(GridResource::new(width, height, entities, rng));

    let handle = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    WORLDS.with(|w| w.borrow_mut().insert(handle, world));
    handle
}

#[wasm_bindgen]
pub fn register_biome_definitions(handle: u32, definitions: JsValue) -> bool {
    WORLDS.with(|w| {
        let mut w = w.borrow_mut();
        let Some(world) = w.get_mut(&handle) else {
            return false;
        };
        match serde_wasm_bindgen::from_value::<Vec<BiomeDef>>(definitions) {
            Ok(defs) => {
                world.insert_resource(BiomeDefinitions::new(defs));
                true
            }
            Err(_) => false,
        }
    })
}

#[wasm_bindgen]
pub fn biome_definitions(handle: u32) -> JsValue {
    WORLDS.with(|w| {
        let w = w.borrow();
        let Some(world) = w.get(&handle) else {
            return JsValue::NULL;
        };
        let Some(defs) = world.get_resource::<BiomeDefinitions>() else {
            return JsValue::NULL;
        };
        serde_wasm_bindgen::to_value(&defs.definitions).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn grid_snapshot(handle: u32) -> JsValue {
    WORLDS.with(|w| {
        let w = w.borrow();
        let Some(world) = w.get(&handle) else {
            return JsValue::NULL;
        };
        let grid = world.resource::<GridResource>();

        let len = grid.entities.len();
        let mut biome_ids = Vec::with_capacity(len);
        let mut resources = Vec::with_capacity(len);

        for &entity in &grid.entities {
            let biome = world
                .get::<BiomeId>(entity)
                .map(|b| b.0)
                .unwrap_or(0);
            let res = world
                .get::<Resources>(entity)
                .map(|r| r.0)
                .unwrap_or(0);
            biome_ids.push(biome);
            resources.push(res);
        }

        let biome_arr = js_sys::Uint16Array::new_with_length(biome_ids.len() as u32);
        biome_arr.copy_from(&biome_ids);

        let res_arr = js_sys::Uint8Array::new_with_length(resources.len() as u32);
        res_arr.copy_from(&resources);

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
    WORLDS.with(|w| {
        let w = w.borrow();
        let Some(world) = w.get(&handle) else {
            return cell_obj(BIOME_NONE, 0);
        };
        let grid = world.resource::<GridResource>();
        if x >= grid.width || y >= grid.height {
            return cell_obj(BIOME_NONE, 0);
        }
        let idx = (y as usize) * (grid.width as usize) + (x as usize);
        let Some(&entity) = grid.entities.get(idx) else {
            return cell_obj(BIOME_NONE, 0);
        };
        let biome = world
            .get::<BiomeId>(entity)
            .map(|b| b.0 as u8)
            .unwrap_or(BIOME_NONE);
        let res = world
            .get::<Resources>(entity)
            .map(|r| r.0)
            .unwrap_or(0);
        cell_obj(biome, res)
    })
}

fn cell_obj(biome: u8, resources: u8) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"biome".into(), &(biome as f64).into()).unwrap();
    js_sys::Reflect::set(&obj, &"resources".into(), &(resources as f64).into()).unwrap();
    obj.into()
}
