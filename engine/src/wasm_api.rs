//! WASM-facing API for the engine. Compiled only for `wasm32`.
//!
//! Grids live in WASM memory owned by the worker. `create_grid` returns an
//! opaque `u32` handle into a `thread_local` registry; `grid_snapshot` and
//! `cell_at` look the grid up by handle. Keeping the registry in
//! `thread_local` avoids passing Rust references across the wasm boundary.
//!
//! Unknown handles and out-of-bounds coordinates never panic — they return
//! a null/empty result or a sentinel `biome = 255` respectively.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::grid::Grid;

/// Next handle to hand out. Handles start at 1 so that 0 can never alias a
/// real grid (callers can treat 0 as "invalid").
static NEXT_HANDLE: AtomicU32 = AtomicU32::new(1);

thread_local! {
    static GRIDS: RefCell<HashMap<u32, Grid>> = RefCell::new(HashMap::new());
}

/// Sentinel biome index returned by [`cell_at`] for out-of-bounds
/// coordinates. Chosen to be outside the valid `0..=3` range.
const BIOME_NONE: u8 = 255;

/// Generate a grid from `(seed, width, height)` and register it. Returns a
/// non-zero handle.
#[wasm_bindgen]
pub fn create_grid(seed: u64, width: u32, height: u32) -> u32 {
    let grid = Grid::generate(seed, width, height);
    let handle = NEXT_HANDLE.fetch_add(1, Ordering::Relaxed);
    GRIDS.with(|g| g.borrow_mut().insert(handle, grid));
    handle
}

/// Snapshot of a grid as a plain JS object:
/// `{ width: number, height: number, biomes: Uint8Array }`.
///
/// Returns `null` for an unknown handle (no panic).
#[wasm_bindgen]
pub fn grid_snapshot(handle: u32) -> JsValue {
    GRIDS.with(|g| {
        let g = g.borrow();
        let Some(grid) = g.get(&handle) else {
            return JsValue::NULL;
        };
        // Gather the biome indices into a Vec<u8> while holding the borrow,
        // then build the Uint8Array after releasing it.
        let biomes: Vec<u8> = grid.cells.iter().map(|c| c.biome as u8).collect();
        let arr = js_sys::Uint8Array::new_with_length(biomes.len() as u32);
        arr.copy_from(&biomes);

        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"width".into(), &(grid.width as f64).into()).unwrap();
        js_sys::Reflect::set(&obj, &"height".into(), &(grid.height as f64).into()).unwrap();
        js_sys::Reflect::set(&obj, &"biomes".into(), &arr.into()).unwrap();
        obj.into()
    })
}

/// Look up a single cell by `(x, y)`. Returns `{ biome: number, resources: number }`,
/// with `biome = 255` for out-of-bounds coordinates. Never panics.
#[wasm_bindgen]
pub fn cell_at(handle: u32, x: u32, y: u32) -> JsValue {
    GRIDS.with(|g| {
        let g = g.borrow();
        let Some(grid) = g.get(&handle) else {
            // Unknown handle: treat every coordinate as out-of-bounds.
            return cell_obj(BIOME_NONE, 0);
        };
        match grid.cell_at(x, y) {
            Some(cell) => cell_obj(cell.biome as u8, cell.resources),
            None => cell_obj(BIOME_NONE, 0),
        }
    })
}

/// Build a `{ biome, resources }` JS object.
fn cell_obj(biome: u8, resources: u8) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"biome".into(), &(biome as f64).into()).unwrap();
    js_sys::Reflect::set(&obj, &"resources".into(), &(resources as f64).into()).unwrap();
    obj.into()
}

