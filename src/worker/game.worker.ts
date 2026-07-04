import init, { greet } from "../wasm/engine";

let ready = false;

self.onmessage = async (e: MessageEvent) => {
  if (e.data.type === "init") {
    try {
      await init();
      ready = true;
      const text = greet();
      self.postMessage({ type: "greeting", text });
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
