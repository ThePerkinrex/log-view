<script setup lang="ts">
import { ref } from "vue";
import type { Record } from "../lib/record";

const props = defineProps<{
  selectedFile: string | null;
  records: Record[];
}>();

const expanded = ref<number | null>(null);

function toggle(i: number) {
  expanded.value = expanded.value === i ? null : i;
}

function formatLine(r: Record) {
  const task = r.extra.task_id ?? "None";
  const proc = r.extra.process_id ?? "None";
  return `[T ${task}][P ${proc}][${r.target}][${r.level}] ${r.message}`;
}
</script>

<template>
  <div v-if="!selectedFile" class="empty">
    <h2>No file selected</h2>
    <p>Select a file from the left sidebar</p>
  </div>

  <div v-else class="log-container">
    <div class="title">{{ selectedFile }}</div>

    <div 
      class="log-line" 
      v-for="(r, i) in records" 
      :key="i"
      @click="toggle(i)"
    >
      <div class="mono line-text">
        {{ formatLine(r) }}
      </div>

      <div v-if="expanded === i" class="expanded">
        <pre>{{ r }}</pre>
      </div>
    </div>
  </div>
</template>

<style scoped>
.empty {
  padding: 20px;
  color: #aaaaaa;
}

.log-container {
  overflow-y: auto;
  height: 100%;
}

.title {
  position: sticky;
  top: 0;
  background: #2f2f2f;
  padding: 10px 4px;
  font-size: 18px;
  font-weight: 600;
  border-bottom: 1px solid #444;
  z-index: 5;
  font-family: "Inter", sans-serif;
}

.log-line {
  padding: 6px 4px;
  border-bottom: 1px solid #3a3a3a;
  cursor: pointer;
  transition: background 0.15s;
}

.log-line:hover {
  background: #3a3a3a;
}

.mono {
  font-family: monospace;
}

.line-text {
  white-space: pre;
  overflow: hidden;
  text-overflow: ellipsis;
}

.expanded {
  margin-top: 6px;
  background: #1a1a1a;
  padding: 10px;
  border-radius: 6px;
  color: #e6e6e6;
  font-family: monospace;
}
</style>
