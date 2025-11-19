<script setup lang="ts">
import { ref, onMounted } from "vue";

defineProps<{
  openFiles: string[];
  recentFiles: string[];
  selectedFile: string | null;
}>();

const emit = defineEmits(["select", "open-file", "close-file", "remove-recent"]);

// Sidebar resizing
const width = ref(260);
const resizing = ref(false);

function startResize(e: MouseEvent) {
  resizing.value = true;
  e.preventDefault();
}

function stopResize() {
  resizing.value = false;
}

function doResize(e: MouseEvent) {
  if (!resizing.value) return;
  width.value = Math.max(180, e.clientX);
}

onMounted(() => {
  window.addEventListener("mousemove", doResize);
  window.addEventListener("mouseup", stopResize);
});
</script>

<template>
  <aside class="sidebar" :style="{ width: width + 'px' }">
    <div class="section-header">
      <span>Open Files</span>
      <button class="icon-btn" @click="$emit('open-file')">＋</button>
    </div>

    <ul class="file-list">
      <li 
        v-for="file in openFiles" 
        :key="file"
        :class="{ active: file === selectedFile }"
        @click="$emit('select', file)"
      >
        <span class="file-path" :title="file">{{ file }}</span>
        <button 
          class="icon-btn close" 
          @click.stop="$emit('close-file', file)"
        >×</button>
      </li>
    </ul>

    <hr />

    <div class="section-header">
      <span>Recent</span>
    </div>

    <ul class="file-list">
      <li 
        v-for="file in recentFiles" 
        :key="file"
        @click="$emit('select', file)"
      >
        <span class="file-path" :title="file">{{ file }}</span>
        <button 
          class="icon-btn close" 
          @click.stop="$emit('remove-recent', file); $emit('close-file', file)"
        >×</button>
      </li>
    </ul>
  </aside>

  <div class="resize-handle" @mousedown="startResize"></div>
</template>

<style scoped>
/* === Resizable sidebar === */
.sidebar {
  background: #272727;
  border-right: 1px solid #444;
  padding: 12px;
  color: #f6f6f6;
  overflow-y: auto;
  font-family: Inter, sans-serif;
  display: flex;
  flex-direction: column;
}

/* Resize bar */
.resize-handle {
  width: 4px;
  cursor: ew-resize;
  background: transparent;
}
.resize-handle:hover {
  background: #555;
}

/* === Layout === */
.section-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 6px;
  font-weight: 600;
}

.file-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.file-list li {
  padding: 8px 6px;
  border-radius: 6px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  gap: 6px;
  transition: background 0.2s;
}

.file-list li:hover {
  background: #353535;
}

.file-list li.active {
  background: #3e3e60;
}

/* === Path shortening from the START === */
/* 
   Magic trick:
   direction: rtl reverses text rendering,
   text-overflow: clip allows start clipping,
   Unicode-bidi restores normal shaping.
*/
.file-path {
  display: inline-block;
  max-width: 100%;
  overflow: hidden;
  white-space: nowrap;
  direction: rtl;
  text-overflow: clip;
  unicode-bidi: plaintext;
  text-align: left;
}

/* === Buttons === */
.icon-btn {
  border: none;
  background: none;
  color: #aaa;
  cursor: pointer;
  font-size: 16px;
}
.icon-btn:hover {
  color: #24c8db;
}
.close {
  font-size: 18px;
}
</style>
