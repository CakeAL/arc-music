<script setup>
import Menu from "./components/Menu.vue";
import TitleBar from "./components/TitleBar.vue";
</script>

<template>
  <n-layout style="height: 100vh">
    <n-layout-header
      style="height: 50px; position: relative"
      bordered
      data-tauri-drag-region
    >
      <TitleBar />
    </n-layout-header>
    <n-layout
      position="absolute"
      style="top: 50px; bottom: 50px"
      :native-scrollbar="false"
    >
        <n-split
          direction="horizontal"
          style="height: calc(100vh - 100px)"
          max="500px"
          min="200px"
          default-size="200px"
        >
          <template #1>
            <Menu />
          </template>
          <template #2>
            <router-view></router-view>
          </template>
        </n-split>
    </n-layout>
    <n-layout-footer position="absolute" style="height: 50px" bordered>
      footer
    </n-layout-footer>
  </n-layout>
</template>

<style scoped></style>
