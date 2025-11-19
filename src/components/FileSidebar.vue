<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount } from "vue";

const props = defineProps<{
  openFiles: string[];
  recentFiles: string[];
  selectedFile: string | null;
}>();

const emit = defineEmits(["select", "open-file", "close-file", "remove-recent"]);

// Sidebar width (resizable)
const width = ref(260);
let dragging = false;

function startDrag() {
  dragging = true;
}

function stopDrag() {
  dragging = false;
}

function drag(e: MouseEvent) {
  if (!dragging) return;
  width.value = Math.max(160, Math.min(e.clientX, 450));
}

onMounted(() => {
  window.addEventListener("mousemove", drag);
  window.addEventListener("mouseup", stopDrag);
});

onBeforeUnmount(() => {
  window.removeEventListener("mousemove", drag);
  window.removeEventListener("mouseup", stopDrag);
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
        <span class="path" :title="file">
          {{ file.split('/').slice(-3).join('/') }}
        </span>
        <button 
          class="icon-btn close" 
          @click.stop="$emit('close-file', file)"
        >
          ×
        </button>
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
        <span class="path" :title="file">
          {{ file.split('/').slice(-3).join('/') }}
        </span>

        <button 
          class="icon-btn close"
          @click.stop="$emit('remove-recent', file)"
        >
          ×
        </button>
      </li>
    </ul>

    <div class="drag-handle" @mousedown="startDrag"></div>
  </aside>
</template>

<style scoped>
.sidebar {
  background: #1f1f1f;
  border-right: 1px solid #333;
  padding: 12px;
  color: #e0e0e0;
  overflow-y: auto;
  position: relative;
  font-family: "Inter", sans-serif;
  user-select: none;
}

.section-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 6px;
  font-weight: 600;
  font-size: 14px;
}

.file-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.file-list li {
  padding: 8px;
  border-radius: 6px;
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  transition: background 0.2s;
  font-size: 14px;
}

.file-list li:hover {
  background: #2d2d2d;
}

.file-list li.active {
  background: #353556;
}

.path {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: block;
  max-width: 180px;
}

.icon-btn {
  border: none;
  background: none;
  color: #aaa;
  cursor: pointer;
  font-size: 16px;
  margin-left: 6px;
}

.icon-btn:hover {
  color: #36c3e8;
}

.close {
  font-size: 18px;
}

.drag-handle {
  width: 5px;
  height: 100%;
  background: transparent;
  cursor: col-resize;
  position: absolute;
  top: 0;
  right: 0;
}
</style>
