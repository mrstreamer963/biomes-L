import { ref, computed, shallowRef, onMounted, onUnmounted, type ComputedRef, type Ref, type ShallowRef } from "vue";
import { createWasmBridge, type WorkerStatus } from "../bridge/wasm";
import type { UnitData } from "../wasm/engine";
import { DEFAULT_BIOME_DEFINITIONS, DEFAULT_GENERATION_PARAMS, type BiomeDefinition } from "../config/biomeConfig";

export type BiomeCounts = Record<number, number>;

export interface GridSnapshotState {
  width: Ref<number>;
  height: Ref<number>;
  biomeIds: Ref<Uint16Array | null>;
  biomeDefinitions: Ref<BiomeDefinition[]>;
  status: Ref<WorkerStatus>;
  error: Ref<string | null>;
  units: ShallowRef<UnitData[]>;
  selectedUnitId: Ref<number | null>;
  biomeCounts: ComputedRef<BiomeCounts>;
  hoveredCell: Ref<{ col: number; row: number } | null>;
  speed: Ref<number>;
  paused: Ref<boolean>;
  selectUnit: (unitId: number) => void;
  clearSelection: () => void;
  sendMoveCommand: (unitId: number, x: number, y: number) => void;
  toggleDebug: () => void;
  setSpeed: (speed: number) => void;
  togglePause: () => void;
}

let instance: GridSnapshotState | null = null;

export function createGridSnapshot(): GridSnapshotState {
  if (instance) return instance;

  const width = ref<number>(0);
  const height = ref<number>(0);
  const biomeIds = ref<Uint16Array | null>(null);
  const biomeDefinitions = ref<BiomeDefinition[]>(DEFAULT_BIOME_DEFINITIONS);
  const status = ref<WorkerStatus>("loading");
  const error = ref<string | null>(null);
  const units = shallowRef<UnitData[]>([]);
  const selectedUnitId = ref<number | null>(null);
  const hoveredCell = ref<{ col: number; row: number } | null>(null);
  const speed = ref<number>(1);
  const paused = ref<boolean>(false);

  let bridge: ReturnType<typeof createWasmBridge> | null = null;
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
        const dt = Math.min((time - lastTime) / 1000, 0.05);
        if (!paused.value) {
          bridge.postMessage({ type: "tick", dt: dt * speed.value });
        }
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

  function toggleDebug() {
    const newDebug = !units.value?.some((u) => u.debug);
    for (const u of units.value ?? []) {
      bridge?.postMessage({ type: "set-unit-debug", unitId: u.id, debug: newDebug });
    }
  }

  function setSpeed(newSpeed: number) {
    speed.value = newSpeed;
  }

  function togglePause() {
    paused.value = !paused.value;
  }

  function handleKeydown(e: KeyboardEvent) {
    const target = e.target as HTMLElement | null;
    if (target && (target.tagName === "INPUT" || target.tagName === "TEXTAREA")) return;

    switch (e.code) {
      case "Digit1":
        setSpeed(1);
        break;
      case "Digit2":
        setSpeed(5);
        break;
      case "Digit3":
        setSpeed(10);
        break;
      case "Space":
        e.preventDefault();
        togglePause();
        break;
    }
  }

  onMounted(() => {
    window.addEventListener("keydown", handleKeydown);

    bridge = createWasmBridge();

    const cleanup = bridge.onMessage((msg) => {
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

    onUnmounted(() => {
      window.removeEventListener("keydown", handleKeydown);
      stopGameLoop();
      cleanup();
      bridge?.terminate();
    });
  });

  instance = {
    width, height, biomeIds, biomeDefinitions, biomeCounts, status, error,
    units, selectedUnitId, hoveredCell, speed, paused,
    selectUnit, clearSelection, sendMoveCommand, toggleDebug, setSpeed, togglePause,
  };
  (window as any).__snapshot = instance;
  return instance;
}

export function useGridSnapshot(): GridSnapshotState {
  if (!instance) {
    throw new Error("useGridSnapshot() must be called after createGridSnapshot() from App.vue");
  }
  return instance;
}