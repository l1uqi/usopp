<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { invoke } from "@tauri-apps/api/tauri";
import Search from "./components/Search.vue";
import { register } from '@tauri-apps/api/globalShortcut';
import { appWindow } from "@tauri-apps/api/window";
import { TauriEvent } from "@tauri-apps/api/event";

let isDragging = false;
let timeout: number | null | undefined = null;

appWindow.listen(TauriEvent.WINDOW_MOVED , () => {
  isDragging = true;
  if (timeout) {
    clearTimeout(timeout);
  }
  timeout = setTimeout(() => {
    isDragging = false;
  }, 500); // 设置延迟时间，单位为毫秒
});

appWindow.listen(TauriEvent.WINDOW_FOCUS , () => {
  if (!isDragging) {
    invoke("window_change", { event: 'focus' });
  }
});

appWindow.listen(TauriEvent.WINDOW_BLUR , () => {
  setTimeout(() => {
    if (!isDragging) {
      invoke("window_change", { event: 'blur' });
    }
  }, 100);
 
});

register('alt+W', () => {
  invoke("window_change", { event: 'focus' });
  const input = document.querySelector("#search-input") as HTMLInputElement;
  if(input) {
    input.focus()
  }
});

</script>

<template>
  <div class="container" data-tauri-drag-region>
    <Search />
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
