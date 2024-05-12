<script setup>
// 单页面路由
import { ref, computed } from "vue";
import Home from "./pages/Home.vue";
import About from "./pages/About.vue";
import NotFound from "./pages/NotFound.vue";
import FolderMgmt from "./pages/FolderMgmt.vue";
import MusicILike from "./pages/MusicILike.vue";
import MyCollection from "./pages/MyCollection.vue";
import RecentlyPlayed from "./pages/RecentlyPlayed.vue";

const routes = {
  "/": Home,
  "/about": About,
  "/folder-mgmt": FolderMgmt,
  "/music-i-like": MusicILike,
  "/my-collection": MyCollection,
  "/recently-played": RecentlyPlayed,
};
const currentPath = ref(window.location.hash);
window.addEventListener("hashchange", () => {
  currentPath.value = window.location.hash;
});
const currentView = computed(() => {
  return routes[currentPath.value.slice(1) || "/"] || NotFound;
});

// import 组件
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
      <n-layout content-style="padding: 24px;" :native-scrollbar="false">
        <component :is="currentView" />
      </n-layout>
    </n-layout>
    <n-layout-footer position="absolute" style="height: 50px" bordered>
      footer
    </n-layout-footer>
  </n-layout>
</template>

<style scoped></style>
