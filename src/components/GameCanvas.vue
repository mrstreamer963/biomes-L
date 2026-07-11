<script setup lang="ts">
import { ref, watch, onMounted } from "vue";
import { useGridSnapshot } from "../composables/useGridSnapshot";
import { createTileMap, TILE_SIZE } from "../game/MapRenderer";
import { MapCamera } from "../game/MapCamera";
import { UnitManager } from "../game/UnitManager";

const container = ref<HTMLDivElement>();
const isLoading = ref(true);

const { width, height, biomeIds, biomeDefinitions, units, selectedUnitId, selectUnit, clearSelection, sendMoveCommand, hoveredCell } = useGridSnapshot();

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

  function handleClick(screenX: number, screenY: number) {
    if (!unitManager) return;

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
  }

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
      camera.onClick(handleClick);

      camera.onPointerMove((sx, sy) => {
        const worldX = (sx - worldContainer.position.x) / worldContainer.scale.x;
        const worldY = (sy - worldContainer.position.y) / worldContainer.scale.y;
        const col = Math.floor(worldX / TILE_SIZE);
        const row = Math.floor(worldY / TILE_SIZE);
        if (col < 0 || col >= width.value || row < 0 || row >= height.value) {
          hoveredCell.value = null;
        } else {
          hoveredCell.value = { col, row };
        }
      });
      camera.onPointerLeave(() => {
        hoveredCell.value = null;
      });
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