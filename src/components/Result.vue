<script setup lang="ts">

</script>

<template>
  <div class="result">
    <div class="loading"></div>
    <div class="list">
      <div class="list-item">
        <div class="icon">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </div>
        <div class="content">
          <div class="title">
            ECS电子合同平台
          </div>
          <div class="description">
            C:\Program Files\ECS-CS\ECS-CS.exe
          </div>
        </div>
      </div>
      <div class="list-item">
        <div class="icon">
          <img src="/tauri.svg" class="logo vite" alt="Vite logo" />
        </div>
        <div class="content">
          <div class="title">
            ECS电子合同平台
          </div>
          <div class="description">
            C:\Program Files\ECS-CS\ECS-CS.exe
          </div>
        </div>
      </div>
      <div class="list-item">
        <div class="icon">
          <img src="../assets/vue.svg" class="logo vite" alt="Vite logo" />
        </div>
        <div class="content">
          <div class="title">
            DBI电子报表
          </div>
          <div class="description">
            C:\Program Files\ECS-CS\ECS-CS.exe
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
