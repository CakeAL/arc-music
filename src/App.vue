<script setup>
import Menu from "./components/Menu.vue";
</script>

<template>
  <n-layout style="height: 100vh">
    <n-layout-header style="height: 50px" bordered data-tauri-drag-region>
      Title
    </n-layout-header>
    <n-layout position="absolute" style="top: 50px; bottom: 50px" has-sider>
      <n-layout-sider
        content-style="padding: 0px;"
        :native-scrollbar="false"
        bordered
      >
        <Menu />
      </n-layout-sider>
      <n-layout content-style="" :native-scrollbar="false">
        <router-view></router-view>
      </n-layout>
    </n-layout>
    <n-layout-footer position="absolute" style="height: 50px" bordered>
      footer
    </n-layout-footer>
  </n-layout>
</template>

<style scoped></style>
