<script setup lang="ts">
import Result from "./Result.vue";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Application, SearchPaylod } from "../type";

const searchval = ref("");

const list = ref([] as Application[])

async function getSearhResult(e) {
  if(e.target.value === '') {
    return list.value = [];
  }
  let result: SearchPaylod = await invoke("search", { name: e.target.value });
  if(result.status) {
    list.value = result.data;
  }
}

</script>

<template>
  <div class="search">
    <img src="/logo.png" class="search-logo logo" alt="logo" />
    <input @input="getSearhResult" class="search-input" v-model="searchval" placeholder="请叫我乌索普, 我支持应用程序搜索..." />
    <div class="search-more"><svg xmlns="http://www.w3.org/2000/svg" width="26" height="26" viewBox="0 0 24 24"><path fill="#000" d="M12 3c-1.1 0-2 .9-2 2s.9 2 2 2s2-.9 2-2s-.9-2-2-2m0 14c-1.1 0-2 .9-2 2s.9 2 2 2s2-.9 2-2s-.9-2-2-2m0-7c-1.1 0-2 .9-2 2s.9 2 2 2s2-.9 2-2s-.9-2-2-2"/></svg></div>
  </div>
  <hr />
  <Result :list="list" />

</template>
<style>

.search {
  display: flex;
  align-items: center;
  padding: 0px 15px;
  height: 65px;
}

.search-logo {
  width: 60px;
  height: 50px;
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
