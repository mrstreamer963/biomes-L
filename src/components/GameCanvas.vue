<script setup lang="ts">
import { ref, watch, onMounted } from "vue";
import { useGridSnapshot } from "../composables/useGridSnapshot";
import { createTileMap } from "../game/MapRenderer";
import { MapCamera } from "../game/MapCamera";
import { UnitManager } from "../game/UnitManager";

const container = ref<HTMLDivElement>();
const isLoading = ref(true);

const { width, height, biomeIds, biomeDefinitions, units, selectedUnitId, selectUnit, clearSelection, sendMoveCommand } = useGridSnapshot();

watch(biomeIds, (data) => {
  if (data) {
    isLoading.value = false;
  }
});

onMounted(async () => {
  const { Application, Container } = await import("pixi.js");
  const app = new Application();
  await app.init({
    resizeTo: container.value!,
    background: 0x1a1a2e,
  });
  container.value!.appendChild(app.canvas as HTMLCanvasElement);

  const canvas = app.canvas as HTMLCanvasElement;
  const worldContainer = new Container();
  app.stage.addChild(worldContainer);

  let camera: MapCamera | null = null;
  let unitManager: UnitManager | null = null;

  // Drag detection state (to distinguish click from pan)
  let pointerDownPos = { x: 0, y: 0 };
  let isDragging = false;

  canvas.addEventListener("pointerdown", (e: PointerEvent) => {
    pointerDownPos = { x: e.clientX, y: e.clientY };
    isDragging = false;
  });

  canvas.addEventListener("pointermove", (e: PointerEvent) => {
    const dx = e.clientX - pointerDownPos.x;
    const dy = e.clientY - pointerDownPos.y;
    if (Math.sqrt(dx * dx + dy * dy) > 5) {
      isDragging = true;
    }
  });

  canvas.addEventListener("pointerup", (e: PointerEvent) => {
    if (isDragging || !unitManager) return;

    const rect = canvas.getBoundingClientRect();
    const screenX = e.clientX - rect.left;
    const screenY = e.clientY - rect.top;

    const worldX = (screenX - worldContainer.position.x) / worldContainer.scale.x;
    const worldY = (screenY - worldContainer.position.y) / worldContainer.scale.y;

    // Hit test units first
    const hit = unitManager.hitTest(worldX, worldY);
    if (hit) {
      selectUnit(hit.unitId);
    } else if (selectedUnitId.value !== null) {
      sendMoveCommand(selectedUnitId.value, worldX, worldY);
      clearSelection();
    }
  });

  watch(
    () => biomeIds.value,
    (data) => {
      if (!data || !width.value || !height.value) return;

      worldContainer.removeChildren();
      const tileMap = createTileMap(data, biomeDefinitions.value, width.value, height.value);
      worldContainer.addChild(tileMap);

      unitManager = new UnitManager();
      worldContainer.addChild(unitManager.container);

      camera?.destroy();
      camera = new MapCamera(worldContainer, canvas);
    }
  );

  watch(
    () => units.value,
    (data) => {
      if (unitManager && data) {
        unitManager.update(data, selectedUnitId.value);
      }
    },
    { deep: false }
  );

  watch(
    () => selectedUnitId.value,
    () => {
      if (unitManager && units.value) {
        unitManager.update(units.value, selectedUnitId.value);
      }
    }
  );
});
</script>

<template>
  <div ref="container" class="canvas-container">
    <div v-if="isLoading" class="loading-overlay">Loading...</div>
  </div>
</template>

<style scoped>
.canvas-container {
  width: 100%;
  height: 100%;
  position: relative;
}

.loading-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  font-size: 1.5rem;
  background: #1a1a2e;
}
</style>