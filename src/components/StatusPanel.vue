<script setup lang="ts">
import { computed } from "vue";
import { useGridSnapshot } from "../composables/useGridSnapshot";

const { width, height, biomeDefinitions, biomeCounts, status, error, units, selectedUnitId, toggleDebug } = useGridSnapshot();

const selectedDebug = computed(() => {
  if (selectedUnitId.value === null) return false;
  const unit = units.value?.find((u) => u.id === selectedUnitId.value);
  return unit?.debug ?? false;
});

const rows = computed(() =>
  biomeDefinitions.value.map((def, id) => ({
    id,
    label: def.name,
    color: def.color,
    count: biomeCounts.value[id] ?? 0,
  }))
);

const colorToHex = (c: number) => `#${c.toString(16).padStart(6, "0")}`;
</script>

<template>
  <div class="status-panel">
    <h2>Status</h2>

    <div class="field">
      <label>Grid size</label>
      <div class="grid-size">
        {{ width }} × {{ height }}
        <span class="total" v-if="width > 0">({{ width * height }} cells)</span>
      </div>
    </div>

    <div class="field" v-if="width > 0">
      <label>Biomes</label>
      <ul class="biome-list">
        <li v-for="row in rows" :key="row.id">
          <span class="swatch" :style="{ background: colorToHex(row.color) }" />
          <span class="biome-name">{{ row.label }}</span>
          <span class="biome-count">{{ row.count }}</span>
        </li>
      </ul>
    </div>

    <div class="field">
      <label>Debug</label>
      <label class="debug-toggle">
        <input type="checkbox" :checked="selectedDebug" :disabled="selectedUnitId === null" @change="selectedUnitId !== null && toggleDebug(selectedUnitId)" />
        Show waypoints and target
      </label>
      <div v-if="selectedUnitId === null" class="hint">Select a unit first</div>
    </div>

    <div class="worker-status">
      <span v-if="status === 'loading'" class="badge loading">
        Connecting to game engine...
      </span>
      <span v-else-if="status === 'ready'" class="badge ready">
        Engine ready
      </span>
      <span v-else-if="status === 'error'" class="badge error">
        Error: {{ error }}
      </span>
    </div>
  </div>
</template>

<style scoped>
.status-panel {
  padding: 1.5rem;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

h2 {
  font-family: sans-serif;
  font-size: 1.25rem;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

label {
  font-family: sans-serif;
  font-size: 0.875rem;
  color: #555;
}

.grid-size {
  font-family: monospace;
  font-size: 0.95rem;
  padding: 0.5rem 0.75rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  background: #f9f9f9;
}

.grid-size .total {
  color: #777;
  margin-left: 0.5rem;
}

.biome-list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

.biome-list li {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  font-family: sans-serif;
  font-size: 0.875rem;
}

.swatch {
  display: inline-block;
  width: 0.9rem;
  height: 0.9rem;
  border-radius: 2px;
  border: 1px solid rgba(0, 0, 0, 0.15);
}

.biome-name {
  flex: 1;
}

.biome-count {
  font-family: monospace;
  font-weight: 600;
}

.badge {
  display: inline-block;
  padding: 0.25rem 0.75rem;
  border-radius: 4px;
  font-size: 0.8125rem;
  font-family: sans-serif;
}

.loading {
  background: #fff3cd;
  color: #856404;
  border: 1px solid #ffeeba;
}

.ready {
  background: #d4edda;
  color: #155724;
  border: 1px solid #c3e6cb;
}

.error {
  background: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}

.debug-toggle {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-family: sans-serif;
  font-size: 0.875rem;
  cursor: pointer;
}

.debug-toggle input {
  width: 1rem;
  height: 1rem;
  cursor: pointer;
}
</style>
