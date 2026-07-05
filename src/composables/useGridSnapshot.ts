import { ref, computed, shallowRef, onMounted, onUnmounted } from "vue";
import { createWasmBridge, type WorkerStatus } from "../bridge/wasm";
import type { UnitData } from "../wasm/engine";
import { DEFAULT_BIOME_DEFINITIONS, DEFAULT_GENERATION_PARAMS, type BiomeDefinition } from "../config/biomeConfig";

export type BiomeCounts = Record<number, number>;

export function useGridSnapshot() {
  const width = ref<number>(0);
  const height = ref<number>(0);
  const biomeIds = ref<Uint16Array | null>(null);
  const biomeDefinitions = ref<BiomeDefinition[]>(DEFAULT_BIOME_DEFINITIONS);
  const status = ref<WorkerStatus>("loading");
  const error = ref<string | null>(null);
  const units = shallowRef<UnitData[]>([]);
  const selectedUnitId = ref<number | null>(null);

  let bridge: ReturnType<typeof createWasmBridge> | null = null;
  let cleanup: (() => void) | null = null;
  let rafId: number | null = null;
  let lastTime: number | null = null;

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

  function startGameLoop() {
    lastTime = null;

    function tick(time: number) {
      if (!bridge) return;

      if (lastTime !== null) {
        const dt = Math.min((time - lastTime) / 1000, 0.05); // cap at 50ms
        bridge.postMessage({ type: "tick", dt });
      }

      lastTime = time;
      rafId = requestAnimationFrame(tick);
    }

    rafId = requestAnimationFrame(tick);
  }

  function stopGameLoop() {
    if (rafId !== null) {
      cancelAnimationFrame(rafId);
      rafId = null;
    }
    lastTime = null;
  }

  function selectUnit(unitId: number) {
    selectedUnitId.value = unitId;
  }

  function clearSelection() {
    selectedUnitId.value = null;
  }

  function sendMoveCommand(unitId: number, x: number, y: number) {
    bridge?.postMessage({ type: "set-unit-target", unitId, x, y });
  }

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
        if (msg.status === "ready") {
          startGameLoop();
        }
      } else if (msg.type === "unit-snapshot") {
        units.value = msg.units;
      }
    });

    bridge.postMessage({
      type: "init",
      biomeDefinitions: DEFAULT_BIOME_DEFINITIONS,
      generationParams: DEFAULT_GENERATION_PARAMS,
    });
  });

  onUnmounted(() => {
    stopGameLoop();
    cleanup?.();
    bridge?.terminate();
  });

  return {
    width, height, biomeIds, biomeDefinitions, biomeCounts, status, error,
    units, selectedUnitId,
    selectUnit, clearSelection, sendMoveCommand,
  };
}