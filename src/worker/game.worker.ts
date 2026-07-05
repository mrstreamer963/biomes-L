import init, { create_grid, grid_snapshot, register_biome_definitions } from "../wasm/engine";
import type { BiomeDefinition } from "../config/biomeConfig";

const DEFAULT_SEED = 42n;
const DEFAULT_WIDTH = 32;
const DEFAULT_HEIGHT = 32;

let ready = false;

self.onmessage = async (e: MessageEvent) => {
  if (e.data.type === "init") {
    try {
      await init();
      ready = true;

      const biomeDefinitions: BiomeDefinition[] = e.data.biomeDefinitions;
      const handle = create_grid(DEFAULT_SEED, DEFAULT_WIDTH, DEFAULT_HEIGHT);

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
      self.postMessage({ type: "status", status: "ready" });
    } catch (err) {
      self.postMessage({
        type: "status",
        status: "error",
        error: String(err),
      });
    }
  }
};

export { ready };
