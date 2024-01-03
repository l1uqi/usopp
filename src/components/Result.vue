<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps({
  list: Array,
});

const list = computed(() => {
  return props.list;
})

console.log(list);
</script>

<template>
  <div class="result">
    <div class="loading"></div>
    <div class="list">
      <div class="list-item" v-for="(item, index) in list" :key="index">
        <div class="icon">
          <img :src="item.icon_path" class="logo vite" alt="Vite logo" />
        </div>
        <div class="content">
          <div class="title">
            {{ item.display_name }}
          </div>
          <div class="description">
            {{ item.run_path }}
          </div>
        </div>
      </div>
    </div>
  </div>

</template>
<style>
.result {
  width: 100%;
  height: auto;
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
