<script setup lang="ts">
import { ref, watch, onMounted } from "vue";
import { useGridSnapshot } from "../composables/useGridSnapshot";
import { createTileMap } from "../game/MapRenderer";
import { MapCamera } from "../game/MapCamera";

const container = ref<HTMLDivElement>();
const isLoading = ref(true);

const { width, height, biomes } = useGridSnapshot();

watch(biomes, (data) => {
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

  const worldContainer = new Container();
  app.stage.addChild(worldContainer);

  let camera: MapCamera | null = null;

  watch(
    () => biomes.value,
    (data) => {
      if (!data || !width.value || !height.value) return;

      worldContainer.removeChildren();
      const tileMap = createTileMap(data, width.value, height.value);
      worldContainer.addChild(tileMap);

      worldContainer.position.set(
        (app.screen.width - width.value * 32) / 2,
        (app.screen.height - height.value * 32) / 2
      );

      camera?.destroy();
      camera = new MapCamera(worldContainer, app.canvas as HTMLCanvasElement);
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
