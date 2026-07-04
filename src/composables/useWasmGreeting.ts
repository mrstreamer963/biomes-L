import { ref, onMounted, onUnmounted } from "vue";
import { createWasmBridge, type WorkerStatus } from "../bridge/wasm";

export function useWasmGreeting() {
  const greeting = ref<string>("");
  const status = ref<WorkerStatus>("loading");
  const error = ref<string | null>(null);

  let bridge: ReturnType<typeof createWasmBridge> | null = null;
  let cleanup: (() => void) | null = null;

  onMounted(() => {
    bridge = createWasmBridge();

    cleanup = bridge.onMessage((msg) => {
      if (msg.type === "greeting") {
        greeting.value = msg.text;
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

  return { greeting, status, error };
}
