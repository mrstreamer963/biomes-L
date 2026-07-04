export type WorkerStatus = "loading" | "ready" | "error";

export type WorkerMessage =
  | { type: "greeting"; text: string }
  | { type: "status"; status: WorkerStatus; error?: string };

export type MainMessage = { type: "init" };

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
