<script setup lang="ts">
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import Result from "./Result.vue";
import Webview from "./Webview.vue";
import { onBeforeUnmount, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Application, SearchPaylod } from "../type";

// 支持的指令
const directives = ["vscode", "idea"];

// 搜索指令
const matchDirective = ref("");

const searchval = ref("");

const loading = ref(false);

const list = ref([] as Application[])

let timeout: number | null | undefined = null;

const webViewRef = ref<HTMLDivElement | null>(null);

const isWebViewVisible = ref(false);

let unlisten: Promise<UnlistenFn> | null = null;

// 获取 WebView 组件的尺寸
const getWebViewDimensions = () => {
  if (webViewRef.value) {
    // ignore
    const webViewElement = webViewRef.value as HTMLDivElement; // 获取 WebView 组件的 DOM 元素
    const webViewWidth = webViewElement.offsetWidth; // 获取 WebView 组件的宽度
    const webViewHeight = webViewElement.offsetHeight; // 获取 WebView 组件的高度
    const webViewTop = webViewElement.offsetTop; // 获取 WebView 组件距离顶部的距离

    invoke("set_parent_window_info", { width: webViewWidth, height: webViewHeight, top: webViewTop })
  }

};

onMounted(() => {
  getWebViewDimensions();
  unlisten = search_listen();
})

onBeforeUnmount(() => {
  if(unlisten != null) {
    unlisten = null;
  }
})

function checkDirective(str: string) {
  const regex = /^(\w+):$/; // 正则表达式匹配带有冒号的输入
  const match = regex.exec(str);
  if (match) {
    const directive = match[1];
    if (directives.includes(directive)) {
      return directive;
    }
  }
  return false;
}

const handleFocus = () => {
  // 隐藏子窗口
  invoke("window_change", { event: 'close_child_win' });
}

const handleKeyDown = (e: { key: string; }) => {
  if (e.key === 'Backspace') {
    if (searchval.value === '' && matchDirective.value) {
      matchDirective.value = '';
    }
  }
}

// 搜索
async function getSearhResult(e: Event) {
  const inputValue = (e.target as HTMLInputElement).value;
  if (inputValue.includes(':')) {
    const val = checkDirective(inputValue)
    if (val) {
      searchval.value = ""
      return matchDirective.value = val;
    }
  }

  loading.value = true;
  if (inputValue === '') {
    loading.value = false;
    return list.value = [];
  }

  list.value = [];

  if (timeout) {
    clearTimeout(timeout);
  }

  timeout = setTimeout(async () => {
    invoke("async_search", { name: inputValue, directive: matchDirective.value });
  }, 500);
}

const search_listen = async () => {
  return await listen<SearchPaylod>('search-result', search_callback);
};


const search_callback = async (event: { payload: { status: any; data: any; }; }) => {
  const { status, data } = event.payload;
  if (status === 'InProgress') {
    list.value = list.value.concat(data);
  }

  if (status === 'Completed') {
    loading.value = false;
    list.value = data;
    console.log(data);
  }

  if (status === 'Error') {
    list.value = [];
  }

}

</script>

<template>
  <div class="search" data-tauri-drag-region>
    <img src="/logo.png" class="search-logo logo" alt="logo" />
    <span v-if="matchDirective" class="directive fade-in">{{ matchDirective }}</span>
    <input id="search-input" @focus="handleFocus" @keydown="handleKeyDown" @input="getSearhResult" class="search-input"
      v-model="searchval" placeholder="I support application search...." />
    <svg v-show="loading" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 25 25">
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
    <!-- <div class="search-more" @click="more"><svg xmlns="http://www.w3.org/2000/svg" width="26" height="26"
        viewBox="0 0 24 24">
        <path fill="#000"
          d="M12 3c-1.1 0-2 .9-2 2s.9 2 2 2s2-.9 2-2s-.9-2-2-2m0 14c-1.1 0-2 .9-2 2s.9 2 2 2s2-.9 2-2s-.9-2-2-2m0-7c-1.1 0-2 .9-2 2s.9 2 2 2s2-.9 2-2s-.9-2-2-2" />
      </svg></div> -->
  </div>
  <hr />
  <Result :searchval="searchval" :list="list" :directive="matchDirective" />
  <Webview v-if="isWebViewVisible" ref="webViewRef" />
</template>
<style>
.search {
  display: flex;
  align-items: center;
  padding: 0px 15px;
  height: 65px;
  background-color: white;
}

.search-logo {
  width: 60px;
  height: 50px;
}

.search-input {
  width: calc(100vw - 150px);
  font-size: 15px;
}

.search-more {
  color: #000;
  width: 40px;
}

input {
  border: 1px solid transparent;
  padding: 10px 12px;
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

.directive {
  font-size: 12px;
  background-color: rgb(214, 221, 154);
  color: white;
  height: 25px;
  padding: 0px 10px;
  border-radius: 8px;
}

.fade-in {
  animation: fade-in 0.8s;
}

@keyframes fade-in {
  from {
    opacity: 0;
  }

  to {
    opacity: 1;
  }
}
</style>
