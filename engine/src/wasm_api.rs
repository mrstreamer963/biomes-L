use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

use bevy_ecs::prelude::{Entity, World};
use bevy_ecs::schedule::Schedule;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

fn log(msg: &str) {
    let m: JsValue = msg.into();
    js_sys::Function::new_no_args("console.log(arguments[0])")
        .call1(&JsValue::NULL, &m)
        .unwrap_or_default();
}

use crate::ecs::*;
use crate::ecs::systems::movement_system;
use crate::ecs::pathfinding::{avoid_corner_clipping, ensure_passable_waypoints, find_path, funnel_algorithm, pixel_to_tile, tile_to_pixel};
use crate::noise;

static NEXT_HANDLE: AtomicU32 = AtomicU32::new(1);
static NEXT_UNIT_ID: AtomicU32 = AtomicU32::new(1);

struct GridStore {
    grid: GridResource,
    definitions: Option<BiomeDefinitions>,
    world: World,
    schedule: Schedule,
    unit_map: HashMap<u32, Entity>,
}

impl GridStore {
    fn new(grid: GridResource) -> Self {
        let mut world = World::new();
        world.insert_resource(GameTime::new());
        let mut schedule = Schedule::default();
        schedule.add_systems(movement_system);

        Self {
            grid,
            definitions: None,
            world,
            schedule,
            unit_map: HashMap::new(),
        }
    }
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
    let grid = GridResource::new(width, height, biome_ids, resources);
    STORE.with(|s| {
        s.borrow_mut().insert(handle, GridStore::new(grid));
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
                let biome_defs = BiomeDefinitions::new(defs);
                store.world.insert_resource(biome_defs.clone());
                store.definitions = Some(biome_defs);
                // Also insert grid as resource
                store.world.insert_resource(GridResource {
                    width: store.grid.width,
                    height: store.grid.height,
                    biome_ids: store.grid.biome_ids.clone(),
                    resources: store.grid.resources.clone(),
                });
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

// ─── Unit API ─────────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub fn create_unit(handle: u32, x: f64, y: f64, unit_type: &str) -> u32 {
    STORE.with(|s| {
        let mut s = s.borrow_mut();
        let Some(store) = s.get_mut(&handle) else {
            return 0;
        };

        let unit_id = NEXT_UNIT_ID.fetch_add(1, Ordering::Relaxed);

        let (base_speed, unit_kind) = match unit_type {
            "Soldier" => (80.0, UnitKind::Soldier),
            _ => (140.0, UnitKind::Scout),
        };

        let entity = store.world.spawn((
            Position { x, y },
            MovementTarget(None),
            Path(Vec::new()),
            BaseSpeed(base_speed),
            MovementStatus::default(),
            Health(100, 100),
            Team(0),
            unit_kind,
            Selected(false),
            DebugFlag(true),
        )).id();

        store.unit_map.insert(unit_id, entity);
        unit_id
    })
}

#[wasm_bindgen]
pub fn set_unit_target(handle: u32, unit_id: u32, x: f64, y: f64) {
    STORE.with(|s| {
        let mut s = s.borrow_mut();
        let Some(store) = s.get_mut(&handle) else {
            return;
        };

        if let Some(&entity) = store.unit_map.get(&unit_id) {
            // Get unit's current position
            let pos = match store.world.get::<Position>(entity) {
                Some(p) => *p,
                None => return,
            };

            // Convert to tile coordinates
            let start_tile = pixel_to_tile(pos.x, pos.y);
            let end_tile = pixel_to_tile(x, y);

            // Snap movement target to tile center
            let tile_center = tile_to_pixel(end_tile.0, end_tile.1);

            // Get grid and definitions for pathfinding
            let grid = store.world.get_resource::<GridResource>().unwrap();
            let defs = store.world.get_resource::<BiomeDefinitions>().unwrap();

            // Find path
            let path_cells = find_path(&grid, &defs, start_tile, end_tile);

            if let Some(cells) = path_cells {
                let funnel_waypoints = funnel_algorithm(&cells, (pos.x, pos.y), tile_center);

                let safe_waypoints = ensure_passable_waypoints(&funnel_waypoints, &cells, &grid, &defs);

                let centered_waypoints: Vec<(f64, f64)> = safe_waypoints.iter().map(|&(wx, wy)| {
                    let (tx, ty) = pixel_to_tile(wx, wy);
                    tile_to_pixel(tx, ty)
                }).collect();

                // Prevent diagonal clipping of impassable corners
                let clipped = avoid_corner_clipping(&centered_waypoints, &grid, &defs);

                let path_waypoints: Vec<(f64, f64)> = if clipped.len() > 1 {
                    clipped[1..].to_vec()
                } else {
                    clipped
                };

                // Movement target is the center of the clicked tile
                if let Some(mut target) = store.world.get_mut::<MovementTarget>(entity) {
                    target.0 = Some(tile_center);
                }

                // Set the path (waypoints at tile centers)
                if let Some(mut path) = store.world.get_mut::<Path>(entity) {
                    path.0 = path_waypoints;
                    let fmt: Vec<String> = path.0.iter().map(|(x,y)| format!("({:.0},{:.0})", x, y)).collect();
                    let msg: JsValue = format!("PATH: {:?}", fmt).into();
                    js_sys::Function::new_no_args("console.log(arguments[0])")
                        .call1(&JsValue::NULL, &msg)
                        .unwrap_or_default();
                }
            } else {
                // No path found — clear target and path
                js_sys::Function::new_no_args("console.log('NO PATH')")
                    .call0(&JsValue::NULL)
                    .unwrap_or_default();
                if let Some(mut target) = store.world.get_mut::<MovementTarget>(entity) {
                    target.0 = None;
                }
                if let Some(mut path) = store.world.get_mut::<Path>(entity) {
                    path.0.clear();
                }
            }
        }
    })
}

#[wasm_bindgen]
pub fn set_unit_debug(handle: u32, unit_id: u32, debug: bool) {
    STORE.with(|s| {
        let mut s = s.borrow_mut();
        let Some(store) = s.get_mut(&handle) else {
            return;
        };

        if let Some(&entity) = store.unit_map.get(&unit_id) {
            if let Some(mut flag) = store.world.get_mut::<DebugFlag>(entity) {
                flag.0 = debug;
            }
        }
    })
}

#[wasm_bindgen]
pub fn set_unit_selected(handle: u32, unit_id: u32, selected: bool) {
    STORE.with(|s| {
        let mut s = s.borrow_mut();
        let Some(store) = s.get_mut(&handle) else {
            return;
        };

        if let Some(&entity) = store.unit_map.get(&unit_id) {
            if let Some(mut sel) = store.world.get_mut::<Selected>(entity) {
                sel.0 = selected;
            }
        }
    })
}

#[derive(serde::Serialize)]
struct UnitSnapshot {
    id: u32,
    x: f64,
    y: f64,
    health: u32,
    max_health: u32,
    unit_type: String,
    team: u8,
    selected: bool,
    debug: bool,
    path: Vec<(f64, f64)>,
    target: Option<(f64, f64)>,
}

#[wasm_bindgen]
pub fn tick(handle: u32, dt: f64) -> JsValue {
    STORE.with(|s| {
        let mut s = s.borrow_mut();
        let Some(store) = s.get_mut(&handle) else {
            return JsValue::NULL;
        };

        // Update game time
        store.world.insert_resource(GameTime { delta: dt });

        // Run ECS systems
        store.schedule.run(&mut store.world);

        // Build snapshot
        let mut snapshots: Vec<UnitSnapshot> = Vec::new();
        for (&id, &entity) in &store.unit_map {
            let pos = store.world.get::<Position>(entity).unwrap();
            let health = store.world.get::<Health>(entity).unwrap();
            let kind = store.world.get::<UnitKind>(entity).unwrap();
            let team = store.world.get::<Team>(entity).unwrap();
            let selected = store.world.get::<Selected>(entity).unwrap();
            let debug = store.world.get::<DebugFlag>(entity).unwrap();
            let path = store.world.get::<Path>(entity).unwrap();
            let target = store.world.get::<MovementTarget>(entity).unwrap();

            snapshots.push(UnitSnapshot {
                id,
                x: pos.x,
                y: pos.y,
                health: health.0,
                max_health: health.1,
                unit_type: match kind {
                    UnitKind::Scout => "Scout".to_string(),
                    UnitKind::Soldier => "Soldier".to_string(),
                },
                team: team.0,
                selected: selected.0,
                debug: debug.0,
                path: path.0.clone(),
                target: target.0,
            });
        }

        serde_wasm_bindgen::to_value(&snapshots).unwrap_or(JsValue::NULL)
    })
}

#[wasm_bindgen]
pub fn spawn_starting_units(handle: u32) {
    STORE.with(|s| {
        let mut s = s.borrow_mut();
        let Some(store) = s.get_mut(&handle) else {
            return;
        };

        let cx = (store.grid.width as f64 * 32.0) / 2.0;
        let cy = (store.grid.height as f64 * 32.0) / 2.0;
        let center_col = (store.grid.width / 2) as usize;
        let center_row = (store.grid.height / 2) as usize;

        // Find a passable cell near center for spawning
        let mut spawn_x = cx;
        let mut spawn_y = cy;
        let mut found = false;

        let radius = 5i32;
        'search: for dr in -radius..=radius {
            for dc in -radius..=radius {
                let r = (center_row as i32 + dr).max(0).min(store.grid.height as i32 - 1) as usize;
                let c = (center_col as i32 + dc).max(0).min(store.grid.width as i32 - 1) as usize;
                let idx = r * (store.grid.width as usize) + c;
                let biome_id = store.grid.biome_ids.get(idx).copied().unwrap_or(0);
                let passable = store.definitions.as_ref()
                    .and_then(|d| d.definitions.get(biome_id as usize))
                    .map(|d| d.passable)
                    .unwrap_or(true);
                if passable {
                    spawn_x = c as f64 * 32.0 + 16.0;
                    spawn_y = r as f64 * 32.0 + 16.0;
                    found = true;
                    break 'search;
                }
            }
        }

        if !found {
            spawn_x = cx;
            spawn_y = cy;
        }

        let unit_id_1 = NEXT_UNIT_ID.fetch_add(1, Ordering::Relaxed);
        let unit_id_2 = NEXT_UNIT_ID.fetch_add(1, Ordering::Relaxed);
        let unit_id_3 = NEXT_UNIT_ID.fetch_add(1, Ordering::Relaxed);

        let e1 = store.world.spawn((
            Position { x: spawn_x - 20.0, y: spawn_y },
            MovementTarget(None),
            Path(Vec::new()),
            BaseSpeed(140.0),
            MovementStatus::default(),
            Health(80, 80),
            Team(0),
            UnitKind::Scout,
            Selected(false),
            DebugFlag(true),
        )).id();
        let e2 = store.world.spawn((
            Position { x: spawn_x + 20.0, y: spawn_y },
            MovementTarget(None),
            Path(Vec::new()),
            BaseSpeed(140.0),
            MovementStatus::default(),
            Health(80, 80),
            Team(0),
            UnitKind::Scout,
            Selected(false),
            DebugFlag(true),
        )).id();
        let e3 = store.world.spawn((
            Position { x: spawn_x, y: spawn_y - 25.0 },
            MovementTarget(None),
            Path(Vec::new()),
            BaseSpeed(80.0),
            MovementStatus::default(),
            Health(150, 150),
            Team(0),
            UnitKind::Soldier,
            Selected(false),
            DebugFlag(true),
        )).id();

        store.unit_map.insert(unit_id_1, e1);
        store.unit_map.insert(unit_id_2, e2);
        store.unit_map.insert(unit_id_3, e3);
    })
}

#[wasm_bindgen]
pub fn unit_count(handle: u32) -> u32 {
    STORE.with(|s| {
        let s = s.borrow();
        let Some(store) = s.get(&handle) else {
            return 0;
        };
        store.unit_map.len() as u32
    })
}