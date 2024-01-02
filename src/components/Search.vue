<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const name = ref("");

</script>

<template>
  <div class="search">
    <img src="/vite.svg" class="search-logo logo" alt="Vite logo" />
    <input class="search-input" v-model="name" placeholder="我是乌索普, 请输入你想要搜索的..." />
    <div class="search-more"><svg xmlns="http://www.w3.org/2000/svg" width="26" height="26" viewBox="0 0 24 24"><path fill="#000" d="M12 3c-1.1 0-2 .9-2 2s.9 2 2 2s2-.9 2-2s-.9-2-2-2m0 14c-1.1 0-2 .9-2 2s.9 2 2 2s2-.9 2-2s-.9-2-2-2m0-7c-1.1 0-2 .9-2 2s.9 2 2 2s2-.9 2-2s-.9-2-2-2"/></svg></div>
  </div>

</template>
<style>

.search {
  display: flex;
  align-items: center;
  padding: 0px 15px;
  height: 60px;
}

.search-logo {
  width: 40px;
}

.search-input {
  width: calc(100vw - 80px);
  font-size: 14px;
}

.search-more {
  color: #000;
  width: 40px;
}

input {
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
}

hr {
  margin-top: 0px;
  margin-bottom: 0px;
  border: 0;
  height: 1px;
  background-image: linear-gradient(to right, rgba(64, 52, 234, 0), rgba(0, 0, 0, 0.75), rgba(0,0,0,0));
}
</style>
