<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { Application } from '../type';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { invoke } from '@tauri-apps/api';

const selectIndex = ref(0);

const props = defineProps({
  list: Array<Application>,
  directive: String,
});

const list = computed(() => {
  return props.list;
})

const directive = computed(() => {
  return props.directive;
});

onMounted(() => {
  document.addEventListener('keydown', handleKeyDown);
})

const fileToUrl = (filePath: string) => {
  if(filePath)return convertFileSrc(filePath)
  return "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAADcklEQVR4nLWWPUgjQRiGv4B2YnsgcsjVdruFxMJGLI4rRLnGExJBkkJSWYiNEhF/CrnCQgwW11iqzXHNCWJjZZBgCoOF5DgWrvEgbQLxfWeZye4m2SRr8kBmZmc3+z5+O7MxJhHZ2Nioo4vM4eGhylZNr+zt7dU3Nzcx6p3b21u5u7uTra0tqdVqsXcJxGJZHHVPvb7df4HU/TZmXPKWyCn6ND7sbXxSeTQgZ2cHKxAMZ2/p8DwG6Z+DE+gYTgYlYOGmOlT3TeFkUAL3uClDQ8NJVAGGofPBL1erVSXgrUC7cAvPKJ+LsAgZzr/Uy/DwsOzs7AjnKaArEBZOehZgeKlUwqjB+fk5WvEJCG5qtlqbcNKTQLvws7Mfsrqa9Av0QEsBhuFcS7SEDidegSg0CWBO/cWtbphMJn3hRAvE43EcRaNJgCQSiaZKBMMJBfrB5eWlLC0tNX4NWQV0PmjZSoAVeA+Tk5NSLBbV/X0CwcfALddKgO8AcnJygrZBuVxGK3JwcIA2nP39/egC6+vrGIn8//pJTi1sMc/2c5wvks1mMRJZXFxE60evHb0OYhgruhW4ufktxXhG8G7DRscrGJ+gAHl9rcjx8XcpFAo4anB9fS2VSqW3R+A4f2Rs7KMbnskgEScBJU5TGAAtQQGGk4mJcVWto6MjHPmZnZ3174J2AlxwDKcE3wtKgORdATSmCgwnFGD4zMyM2LbdVAXCcHSiGtJOQFeA4bu7u/Jr/B9euThJ0CsJkMKPAQW84U9PTzgjsry8TEGT5cVMagFcKJaF26KfmpoyFWD5KUAoQbRIAdcR7m0KJBLfVPjz8zNmRS1K3M9keTGTFJibmzPh7IMV0CvYy9raminn0NBQ/fHxESORq6srtC4XFxe8p7omiJmkQKcKZPTzB47joBVZWFjwCfB6DV84pG8VoIAO1oyOjqoVTV5eXtC6PDw8oHXpWqBTBYIvFoaPjJQwwr/geeyGlCWf/37AkZ/5+Xnez2R5MZMUCKsAXxrc09PT07jaDScUwKUSy6HBjqAE0SJh4cScoEC7CjCcz5m/mKyCDidawMZ/P/gKaEiQnJ3GfJcCrSpAGI5OQYmVlRWM3HCCSxFrSRoC+CpwJTqFE3OSKxhdE95wDSXQYVG62zIoYOXSmOU4PJx0vCAMLUL446LpJljzBpfZ4I7wyXu9AAAAAElFTkSuQmCC"
}

const handleKeyDown = (event) => {
  if(list.value?.length === 0) return; 
  if (event.key === 'ArrowUp') {
    selectIndex.value = selectIndex.value -1;
    if(selectIndex.value < 0) {
      selectIndex.value = list.value.length - 1;
    }
  } else if (event.key === 'ArrowDown') {
    selectIndex.value = selectIndex.value + 1;
    if(selectIndex.value >= list.value.length) {
      selectIndex.value = 0;
    }
  }

  if(event.key === 'Enter') {
    handleOpen(list.value[selectIndex.value]);
  }
}

const handleMouseOver = (index: number) => {
      // Your logic for mouseover event
  selectIndex.value = index;
}

const handleOpen = (app: Application) => {
  invoke("open", { rType: app.r_type, path: app.r_exe_path, directive: directive.value });
}
</script>

<template>
  <div class="result">
    <div class="loading"></div>
    <div class="list">
      <div class="list-item" :class="{ active: selectIndex === index }" :style="{ background: selectIndex === index? '#f7f7f7' : '' }" :index="index" v-for="(item, index) in list" :key="index" @click="handleOpen(item)" @mouseover="handleMouseOver(index)">
        <div class="icon">
          <img :src="fileToUrl(item.r_icon_path)" />
        </div>
        <div class="content">
          <div class="title">
            {{ item.text_name }}
          </div>
          <div class="description">
            {{ item.r_exe_path }}
          </div>
        </div>
      </div>
    </div>
  </div>

</template>
<style>
.result {
  width: 100%;
  max-height: 315px;
  overflow-y: auto;
}
.loading {
  background: #fff;
  position: fixed;
  top: 0;
  left: 0;
  z-index: 999
}
.list-item {
  display: flex;
  align-items: center;
  padding: 15px;
  border-bottom: 1px solid rgb(243, 241, 241);
  overflow: hidden;
}

.icon {
  width: 40px;
  margin-right: 10px;
}

.content {
  display: flex;
  flex-direction: column;
}

.title {
  font-size: 14px;
}

.description {
  font-size: 14px;
  margin-top: 5px;
  color: gray;
}
</style>
