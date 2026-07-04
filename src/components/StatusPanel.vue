<script setup lang="ts">
import { useWasmGreeting } from "../composables/useWasmGreeting";

const { greeting, status, error } = useWasmGreeting();
</script>

<template>
  <div class="status-panel">
    <h2>Status</h2>

    <div class="field">
      <label for="greeting-input">Greeting from WASM</label>
      <input
        id="greeting-input"
        type="text"
        :value="greeting"
        readonly
        placeholder="Waiting..."
      />
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

input {
  padding: 0.5rem 0.75rem;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 0.875rem;
  font-family: monospace;
  background: #f9f9f9;
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
</style>
