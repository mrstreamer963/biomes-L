<script setup lang="ts">
import { ref, onMounted } from "vue";

const container = ref<HTMLDivElement>();

onMounted(async () => {
  const { Application } = await import("pixi.js");
  const app = new Application();
  await app.init({
    resizeTo: container.value!,
    background: 0x1a1a2e,
  });
  container.value!.appendChild(app.canvas as HTMLCanvasElement);

  const { Graphics } = await import("pixi.js");
  const circle = new Graphics();
  circle.circle(0, 0, 50);
  circle.fill({ color: 0xe94560 });
  circle.position.set(app.screen.width / 2, app.screen.height / 2);
  app.stage.addChild(circle);
});
</script>

<template>
  <div ref="container" class="canvas-container" />
</template>

<style scoped>
.canvas-container {
  width: 100%;
  height: 100%;
}
</style>
