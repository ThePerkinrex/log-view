<script setup lang="ts">
import { ref } from "vue";
import { 
  get_open_files,
  get_recent_files,
  get_file,
  select_file as selectFileCommand,
  open_file as openFileCommand,
  close_file,
  remove_recent_file
} from "./lib/commands";

import FileSidebar from "./components/FileSidebar.vue";
import LogView from "./components/LogView.vue";
import type { Record } from "./lib/record";

const openFiles = ref<string[]>([]);
const recentFiles = ref<string[]>([]);
const selectedFile = ref<string | null>(null);
const records = ref<Record[]>([]);

async function reloadFiles() {
  openFiles.value = await get_open_files();
  recentFiles.value = await get_recent_files();
}

async function selectFile(path: string) {
  await openFileCommand(path);
  await reloadFiles();
  selectedFile.value = path;
  records.value = await get_file(path) ?? [];
  
  await reloadFiles();
}

async function openFile() {
  const file = await selectFileCommand();
  if (file) {
    selectFile(file);
  }
}

async function closeSelectedFile() {
  if (!selectedFile.value) return;
  await close_file(selectedFile.value);
  selectedFile.value = null;
  records.value = [];
  await reloadFiles();
}

async function deleteRecent(path: string) {
  await remove_recent_file(path);
  await reloadFiles();
}

reloadFiles();
</script>

<template>
  <div class="layout">
    <FileSidebar 
      :openFiles="openFiles"
      :recentFiles="recentFiles"
      :selectedFile="selectedFile"
      @select="selectFile"
      @open-file="openFile"
      @close-file="closeSelectedFile"
      @remove-recent="deleteRecent"
    />

    <main class="content">
      <LogView 
        :selectedFile="selectedFile"
        :records="records"
      />
    </main>
  </div>
</template>

<style scoped>
.layout {
  display: flex;
  height: 100vh;
  background: #2b2b2b;
  color: #f0f0f0;
  font-family: "Inter", sans-serif;
  overflow: hidden;
}

.content {
  flex: 1;
  padding: 0;
  overflow-y: auto;
}
</style>
<style>
body, html {
  margin: 0;
  padding: 0;
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  background: #1e1e1e;
  color: #f6f6f6;
}

::-webkit-scrollbar {
  width: 8px;
}
::-webkit-scrollbar-thumb {
  background: #444;
  border-radius: 4px;
}
::-webkit-scrollbar-thumb:hover {
  background: #555;
}
</style>