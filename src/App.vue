<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { invoke } from "@tauri-apps/api/tauri";
import Search from "./components/Search.vue";
import { register } from '@tauri-apps/api/globalShortcut';

window.addEventListener('blur', () => {
  invoke("window_change", { event: 'blur' });
});

register('alt+W', () => {
  invoke("window_change", { event: 'focus' });
});

</script>

<template>
  <div class="container">
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
