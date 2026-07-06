import init, { create_grid, grid_snapshot, register_biome_definitions, spawn_starting_units, tick, set_unit_target, set_unit_debug } from "../wasm/engine";
import type { BiomeDefinition, GenerationParams } from "../config/biomeConfig";
import { DEFAULT_GENERATION_PARAMS } from "../config/biomeConfig";

const DEFAULT_WIDTH = 50;
const DEFAULT_HEIGHT = 50;

let ready = false;
let handle = 0;

self.onmessage = async (e: MessageEvent) => {
  if (e.data.type === "init") {
    try {
      await init();
      ready = true;

      const biomeDefinitions: BiomeDefinition[] = e.data.biomeDefinitions;
      const generationParams: GenerationParams = e.data.generationParams ?? DEFAULT_GENERATION_PARAMS;
      handle = create_grid(generationParams, DEFAULT_WIDTH, DEFAULT_HEIGHT);

      if (biomeDefinitions) {
        register_biome_definitions(handle, biomeDefinitions);
      }

      const snapshot = grid_snapshot(handle);
      if (snapshot === null) {
        throw new Error(`grid_snapshot returned null for handle ${handle}`);
      }

      self.postMessage({
        type: "grid-snapshot",
        width: snapshot.width,
        height: snapshot.height,
        biomeIds: snapshot.biomeIds,
        resources: snapshot.resources,
      });

      // Spawn starting units after grid is initialized
      spawn_starting_units(handle);

      self.postMessage({ type: "status", status: "ready" });
    } catch (err) {
      self.postMessage({
        type: "status",
        status: "error",
        error: String(err),
      });
    }
  } else if (e.data.type === "tick") {
    if (handle === 0) return;
    const dt = e.data.dt;
    const units = tick(handle, dt);
    if (units) {
      self.postMessage({ type: "unit-snapshot", units });
    }
  } else if (e.data.type === "set-unit-target") {
    if (handle === 0) return;
    set_unit_target(handle, e.data.unitId, e.data.x, e.data.y);
  } else if (e.data.type === "set-unit-debug") {
    if (handle === 0) return;
    set_unit_debug(handle, e.data.unitId, e.data.debug);
  }
};

export { ready };