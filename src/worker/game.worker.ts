import init, { create_grid, grid_snapshot } from "../wasm/engine";

// Default map configuration. Kept as module constants so the same seed is
// used on every init — a page reload reproduces the same map.
const DEFAULT_SEED = 42n;
const DEFAULT_WIDTH = 32;
const DEFAULT_HEIGHT = 32;

let ready = false;

self.onmessage = async (e: MessageEvent) => {
  if (e.data.type === "init") {
    try {
      await init();
      ready = true;

      const handle = create_grid(DEFAULT_SEED, DEFAULT_WIDTH, DEFAULT_HEIGHT);
      const snapshot = grid_snapshot(handle);
      if (snapshot === null) {
        throw new Error(`grid_snapshot returned null for handle ${handle}`);
      }

      self.postMessage({
        type: "grid-snapshot",
        width: snapshot.width,
        height: snapshot.height,
        biomes: snapshot.biomes,
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

// `ready` is exported for potential future ping/health messages; the value
// is currently unused by main thread but kept to preserve the module's
// health-check seam without expanding the message protocol now.
export { ready };
