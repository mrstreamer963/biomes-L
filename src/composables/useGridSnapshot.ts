import { ref, computed, onMounted, onUnmounted } from "vue";
import { createWasmBridge, type WorkerStatus } from "../bridge/wasm";

export type BiomeCounts = {
  plains: number;
  forest: number;
  water: number;
  mountain: number;
};

/**
 * Owns the worker lifecycle and exposes a reactive view of the latest grid
 * snapshot received from the engine. The worker creates the default grid on
 * init and posts a `grid-snapshot` message; this composable stores it and
 * derives per-biome counts for the status panel.
 */
export function useGridSnapshot() {
  const width = ref<number>(0);
  const height = ref<number>(0);
  const biomes = ref<Uint8Array | null>(null);
  const status = ref<WorkerStatus>("loading");
  const error = ref<string | null>(null);

  let bridge: ReturnType<typeof createWasmBridge> | null = null;
  let cleanup: (() => void) | null = null;

  const biomeCounts = computed<BiomeCounts>(() => {
    const counts: BiomeCounts = {
      plains: 0,
      forest: 0,
      water: 0,
      mountain: 0,
    };
    const arr = biomes.value;
    if (!arr) return counts;
    for (let i = 0; i < arr.length; i++) {
      switch (arr[i]) {
        case 0:
          counts.plains++;
          break;
        case 1:
          counts.forest++;
          break;
        case 2:
          counts.water++;
          break;
        case 3:
          counts.mountain++;
          break;
      }
    }
    return counts;
  });

  onMounted(() => {
    bridge = createWasmBridge();

    cleanup = bridge.onMessage((msg) => {
      if (msg.type === "grid-snapshot") {
        width.value = msg.width;
        height.value = msg.height;
        // Copy out of the worker-transferred buffer so the ref owns a
        // stable array independent of any future transfer.
        biomes.value = new Uint8Array(msg.biomes);
      } else if (msg.type === "status") {
        status.value = msg.status;
        if (msg.error) {
          error.value = msg.error;
        }
      }
    });

    bridge.postMessage({ type: "init" });
  });

  onUnmounted(() => {
    cleanup?.();
    bridge?.terminate();
  });

  return { width, height, biomes, biomeCounts, status, error };
}
