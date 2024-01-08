<script setup lang="ts">
import Result from "./Result.vue";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Application, SearchPaylod } from "../type";

const searchval = ref("");

const loading = ref(false);

const list = ref([] as Application[])

let timeout = null;

async function getSearhResult(e) {
  loading.value = true;
  if (e.target.value === '') {
    loading.value = false;
    return list.value = [];
  }

  clearTimeout(timeout);
  timeout = setTimeout(async () => {
    invoke("search", { name: e.target.value }).then(result => {
      if (result.status) {
        loading.value = false;
        list.value = result.data;
      }
    });

  }, 500);
}

</script>

<template>
  <div class="search" data-tauri-drag-region>
    <img src="/logo.png" class="search-logo logo" alt="logo" />
    <svg v-show="loading" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24">
      <g fill="none" stroke="#87CEFA" stroke-linecap="round" stroke-width="2">
        <path stroke-dasharray="60" stroke-dashoffset="60" stroke-opacity=".3"
          d="M12 3C16.9706 3 21 7.02944 21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3Z">
          <animate fill="freeze" attributeName="stroke-dashoffset" dur="1.3s" values="60;0" />
        </path>
        <path stroke-dasharray="15" stroke-dashoffset="15" d="M12 3C16.9706 3 21 7.02944 21 12">
          <animate fill="freeze" attributeName="stroke-dashoffset" dur="0.3s" values="15;0" />
          <animateTransform attributeName="transform" dur="1.5s" repeatCount="indefinite" type="rotate"
            values="0 12 12;360 12 12" />
        </path>
      </g>
    </svg>
    <input id="search-input" @input="getSearhResult" class="search-input" v-model="searchval"
      placeholder="I'm usopp, I support application search...." />
    <div class="search-more"><svg xmlns="http://www.w3.org/2000/svg" width="26" height="26" viewBox="0 0 24 24">
        <path fill="#000"
          d="M12 3c-1.1 0-2 .9-2 2s.9 2 2 2s2-.9 2-2s-.9-2-2-2m0 14c-1.1 0-2 .9-2 2s.9 2 2 2s2-.9 2-2s-.9-2-2-2m0-7c-1.1 0-2 .9-2 2s.9 2 2 2s2-.9 2-2s-.9-2-2-2" />
      </svg></div>
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
  font-size: 15px;
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
  background-image: linear-gradient(to right, rgba(64, 52, 234, 0), rgba(0, 0, 0, 0.75), rgba(0, 0, 0, 0));
}
</style>
