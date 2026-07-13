<script setup lang="ts">
import StatusPanel from "./components/StatusPanel.vue";
import GameCanvas from "./components/GameCanvas.vue";
import { createGridSnapshot } from "./composables/useGridSnapshot";
import { createViewMode } from "./composables/useViewMode";

createGridSnapshot();
createViewMode();
</script>

<template>
  <div class="app-layout">
    <aside class="sidebar">
      <StatusPanel />
    </aside>
    <main class="canvas-area">
      <GameCanvas />
    </main>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  width: 100%;
  height: 100%;
  overflow: hidden;
}

.app-layout {
  display: flex;
  width: 100%;
  height: 100%;
}

.sidebar {
  width: 30%;
  min-width: 280px;
  max-width: 400px;
  height: 100%;
  border-right: 1px solid #ccc;
}

.canvas-area {
  flex: 1;
  height: 100%;
}
</style>
