<script setup lang="ts">
import { ref } from "vue";
import type { Record as LogRecord } from "../lib/record";

defineProps<{
  selectedFile: string | null;
  records: LogRecord[];
}>();

// Track expansions
const expanded = ref<number | null>(null);
function toggle(i: number) {
  expanded.value = expanded.value === i ? null : i;
}

// -------------------------------------------------------
// COLOR MAPS
// -------------------------------------------------------
const taskColors = new Map<string, string>();
const procColors = new Map<string, string>();
const targetColors = new Map<string, string>();

// Fixed colors for levels
const levelColors: Record<string, string> = {
  TRACE: "#7f8c8d",
  DEBUG: "#3498db",
  INFO:  "#2ecc71",
  WARN:  "#f1c40f",
  ERROR: "#e74c3c",
};

// deterministic pastel color generator
function pickColor(map: Map<string,string>, key: string) {
  if (map.has(key)) return map.get(key)!;

  const h = (Array.from(key).reduce((a, c) => a + c.charCodeAt(0), 0) * 37) % 360;
  const color = `hsl(${h}, 65%, 55%)`;
  map.set(key, color);
  return color;
}

// -------------------------------------------------------
// FORMAT LINE INTO COLORED SEGMENTS
// -------------------------------------------------------
function colored(text: string, color: string) {
  return `<span class="pill" style="color:${color}">[${text}]</span>`;
}

function formatHTML(r: LogRecord) {
  const task = r.extra.task_id ?? "None";
  const proc = r.extra.process_id ?? "None";

  const taskColor = pickColor(taskColors, task);
  const procColor = pickColor(procColors, proc);
  const targetColor = pickColor(targetColors, r.target);
  const levelColor = levelColors[r.level] ?? "#bbb";

  const t = colored(`T ${task}`, taskColor);
  const p = colored(`P ${proc}`, procColor);
  const tg = colored(r.target, targetColor);
  const lvl = colored(r.level, levelColor);

  // Final line — wrapped, monospace
  return `${t} ${p} ${tg} ${lvl} ${r.message}`;
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
      <div
        class="mono line-text"
        v-html="formatHTML(r)"
      ></div>

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

/* LOG ROW */
.log-line {
  padding: 6px 4px;
  border-bottom: 1px solid #3a3a3a;
  cursor: pointer;
  transition: background 0.15s;
}

.log-line:hover {
  background: #3a3a3a;
}

/* PILL STYLE */
.pill {
  display: inline-block;
  padding: 2px 6px;
  border-radius: 4px;
  margin-right: 4px;
  font-family: monospace;
  font-size: 12px;
  color: black;
  font-weight: 600;
  white-space: nowrap;
}

/* LOG TEXT */
.mono {
  font-family: monospace;
}

.line-text {
  white-space: pre-wrap; /* allow wrapping */
  word-break: break-word; /* ensures long messages wrap */
}

/* EXPANDED VIEW */
.expanded {
  margin-top: 6px;
  background: #1a1a1a;
  padding: 10px;
  border-radius: 6px;
  color: #e6e6e6;
  font-family: monospace;
}
</style>
