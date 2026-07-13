import { ref, type Ref } from "vue";
import { tacticalBlendFor, resolveViewMode, type ViewMode } from "../game/viewBlend";

export interface ViewModeState {
  zoomScale: Ref<number>;
  blend: Ref<number>;
  viewMode: Ref<ViewMode>;
  setZoomScale: (scale: number) => void;
}

let instance: ViewModeState | null = null;

export function createViewMode(): ViewModeState {
  if (instance) return instance;

  const zoomScale = ref<number>(1);
  const blend = ref<number>(0);
  const viewMode = ref<ViewMode>("detailed");

  function setZoomScale(scale: number) {
    zoomScale.value = scale;
    blend.value = tacticalBlendFor(scale);
    viewMode.value = resolveViewMode(scale, viewMode.value);
  }

  instance = { zoomScale, blend, viewMode, setZoomScale };
  return instance;
}

export function useViewMode(): ViewModeState {
  if (!instance) {
    throw new Error("useViewMode() must be called after createViewMode() from App.vue");
  }
  return instance;
}
