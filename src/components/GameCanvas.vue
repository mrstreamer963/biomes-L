<script setup lang="ts">
import { ref, watch, onMounted } from "vue";
import { useGridSnapshot } from "../composables/useGridSnapshot";
import { useViewMode } from "../composables/useViewMode";
import { createTileMap, TILE_SIZE } from "../game/MapRenderer";
import { MapCamera } from "../game/MapCamera";
import { UnitManager } from "../game/UnitManager";
import { TacticalRenderer, MARKER_SCREEN_PX } from "../game/TacticalRenderer";
import { approach } from "../game/viewBlend";

const container = ref<HTMLDivElement>();
const isLoading = ref(true);

const { width, height, biomeIds, biomeDefinitions, units, selectedUnitId, selectUnit, clearSelection, sendMoveCommand, hoveredCell } = useGridSnapshot();
const { blend, setZoomScale } = useViewMode();

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

  const detailedLayer = new Container();
  const tacticalLayer = new Container();
  tacticalLayer.alpha = 0;
  tacticalLayer.visible = false;
  worldContainer.addChild(detailedLayer);
  worldContainer.addChild(tacticalLayer);

  let camera: MapCamera | null = null;
  let unitManager: UnitManager | null = null;
  let tacticalRenderer: TacticalRenderer | null = null;
  let displayedBlend = 0;

  function handleClick(screenX: number, screenY: number) {
    if (!unitManager || !camera) return;

    const { x: worldX, y: worldY } = camera.screenToWorld(screenX, screenY);
    const minRadius = MARKER_SCREEN_PX / camera.getScale();

    // Hit test units first
    const hit = unitManager.hitTest(worldX, worldY, minRadius);
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

      detailedLayer.removeChildren();
      const tileMap = createTileMap(data, biomeDefinitions.value, width.value, height.value);
      detailedLayer.addChild(tileMap);

      unitManager = new UnitManager();
      detailedLayer.addChild(unitManager.container);

      tacticalRenderer?.destroy();
      tacticalLayer.removeChildren();
      tacticalRenderer = new TacticalRenderer();
      tacticalRenderer.buildMap(data, biomeDefinitions.value, width.value, height.value);
      tacticalLayer.addChild(tacticalRenderer.container);

      camera?.destroy();
      camera = new MapCamera(worldContainer, canvas, { minZoom: 0.1 });
      camera.onClick(handleClick);

      camera.onZoomChange((scale) => {
        setZoomScale(scale);
        tacticalRenderer?.setViewScale(scale);
      });

      camera.onPointerMove((sx, sy) => {
        if (!camera) return;
        const { x: worldX, y: worldY } = camera.screenToWorld(sx, sy);
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

      setZoomScale(camera.getScale());
      tacticalRenderer.setViewScale(camera.getScale());
      if (units.value) {
        tacticalRenderer.updateUnits(units.value, selectedUnitId.value);
      }
    }
  );

  watch(
    () => units.value,
    (data) => {
      if (unitManager && data) {
        unitManager.update(data, selectedUnitId.value);
      }
      if (tacticalRenderer && data) {
        tacticalRenderer.updateUnits(data, selectedUnitId.value);
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
      if (tacticalRenderer && units.value) {
        tacticalRenderer.updateUnits(units.value, selectedUnitId.value);
      }
    }
  );

  app.ticker.add((ticker) => {
    displayedBlend = approach(displayedBlend, blend.value, ticker.deltaMS);
    tacticalLayer.alpha = displayedBlend;
    tacticalLayer.visible = displayedBlend > 0.001;
  });
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