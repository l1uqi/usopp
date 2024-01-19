<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { invoke } from "@tauri-apps/api/tauri";
import Search from "./components/Search.vue";
import { register } from '@tauri-apps/api/globalShortcut';
import { appWindow } from "@tauri-apps/api/window";
import { TauriEvent } from "@tauri-apps/api/event";
import { onMounted, ref } from "vue";

let isDragging = false;

let timeout: number | null | undefined = null;

const container = ref(null);


const mutationObserver = new MutationObserver(() => {
  if (!container.value) return;
  let height = parseFloat(window.getComputedStyle(container.value).getPropertyValue('height'));
  let width = parseFloat(window.getComputedStyle(container.value).getPropertyValue('width'));
  invoke("window_resize", { width, height, wType: 'window' });
});

onMounted(() => {
  if (container.value) {
    mutationObserver.observe(container.value, {
      childList: true, // 子节点的变动（新增、删除或者更改）
      attributes: true, // 属性的变动
      characterData: true, // 节点内容或节点文本的变动
      subtree: true // 是否将观察器应用于该节点的所有后代节点
    });
  }


})



appWindow.listen(TauriEvent.WINDOW_MOVED, () => {
  isDragging = true;
  if (timeout) {
    clearTimeout(timeout);
  }
  timeout = setTimeout(() => {
    isDragging = false;
  }, 500); // 设置延迟时间，单位为毫秒
});


appWindow.listen(TauriEvent.WINDOW_FOCUS, () => {
  if (!isDragging) {
    invoke("window_change", { event: 'focus' });
  }
});

appWindow.listen(TauriEvent.WINDOW_BLUR, () => {
  setTimeout(() => {
    if (!isDragging) {
      setTimeout(() => {
        // invoke("window_change", { event: 'blur' });
      }, 500);
    }
  }, 100);

});

register('alt+W', () => {
  invoke("window_change", { event: 'focus' });
  const input = document.querySelector("#search-input") as HTMLInputElement;
  if (input) {
    input.focus()
  }
});

window.onresize = () => {
  const element = document.getElementById('webview');
  if (element) {
    const width = element.offsetWidth;
    const height = element.offsetHeight;
    invoke("window_resize", { width: width, height: height, wType: 'webview' });
  }
}

</script>

<template>
  <div ref="container" class="container" data-tauri-drag-region>
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
