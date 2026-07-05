import { ref, computed, onMounted, onUnmounted } from "vue";
import { createWasmBridge, type WorkerStatus } from "../bridge/wasm";
import { DEFAULT_BIOME_DEFINITIONS, type BiomeDefinition } from "../config/biomeConfig";

export type BiomeCounts = Record<number, number>;

export function useGridSnapshot() {
  const width = ref<number>(0);
  const height = ref<number>(0);
  const biomeIds = ref<Uint16Array | null>(null);
  const biomeDefinitions = ref<BiomeDefinition[]>(DEFAULT_BIOME_DEFINITIONS);
  const status = ref<WorkerStatus>("loading");
  const error = ref<string | null>(null);

  let bridge: ReturnType<typeof createWasmBridge> | null = null;
  let cleanup: (() => void) | null = null;

  const biomeCounts = computed<BiomeCounts>(() => {
    const counts: BiomeCounts = {};
    const arr = biomeIds.value;
    if (!arr) return counts;
    for (let i = 0; i < arr.length; i++) {
      const id = arr[i];
      counts[id] = (counts[id] ?? 0) + 1;
    }
    return counts;
  });

  onMounted(() => {
    bridge = createWasmBridge();

    cleanup = bridge.onMessage((msg) => {
      if (msg.type === "grid-snapshot") {
        width.value = msg.width;
        height.value = msg.height;
        biomeIds.value = new Uint16Array(msg.biomeIds);
      } else if (msg.type === "status") {
        status.value = msg.status;
        if (msg.error) {
          error.value = msg.error;
        }
      }
    });

    bridge.postMessage({ type: "init", biomeDefinitions: DEFAULT_BIOME_DEFINITIONS });
  });

  onUnmounted(() => {
    cleanup?.();
    bridge?.terminate();
  });

  return { width, height, biomeIds, biomeDefinitions, biomeCounts, status, error };
}
