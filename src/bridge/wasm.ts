import type { BiomeDefinition, GenerationParams } from "../config/biomeConfig";
import type { UnitData } from "../wasm/engine";

export type WorkerStatus = "loading" | "ready" | "error";

export type WorkerMessage =
  | { type: "grid-snapshot"; width: number; height: number; biomeIds: Uint16Array; resources: Uint8Array }
  | { type: "status"; status: WorkerStatus; error?: string }
  | { type: "unit-snapshot"; units: UnitData[] };

export type MainMessage =
  | { type: "init"; biomeDefinitions: BiomeDefinition[]; generationParams: GenerationParams }
  | { type: "tick"; dt: number }
  | { type: "set-unit-target"; unitId: number; x: number; y: number }
  | { type: "set-unit-debug"; unitId: number; debug: boolean };

export interface WasmBridge {
  postMessage(msg: MainMessage): void;
  onMessage(cb: (msg: WorkerMessage) => void): () => void;
  terminate(): void;
}

export function createWasmBridge(): WasmBridge {
  const worker = new Worker(
    new URL("../worker/game.worker.ts", import.meta.url),
    { type: "module" }
  );

  return {
    postMessage(msg: MainMessage) {
      worker.postMessage(msg);
    },
    onMessage(cb: (msg: WorkerMessage) => void) {
      const handler = (e: MessageEvent<WorkerMessage>) => {
        cb(e.data);
      };
      worker.addEventListener("message", handler);
      return () => worker.removeEventListener("message", handler);
    },
    terminate() {
      worker.terminate();
    },
  };
}
