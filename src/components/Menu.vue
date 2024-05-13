<template>
  <n-menu :options="menuOptions" />
</template>

<script>
import { defineComponent, h } from "vue";
import { RouterLink } from "vue-router";

const menuOptions = [
  {
    label: () =>
      h(
        RouterLink,
        {
          to: { path: "/" },
        },
        { default: () => "主页" }
      ),
    key: "home",
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: { path: "/about" },
        },
        { default: () => "关于" }
      ),
    key: "about",
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: { path: "/folder-mgmt" },
        },
        { default: () => "文件夹管理" }
      ),
    key: "folder-mgmt",
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: { path: "/music-i-like" },
        },
        { default: () => "我喜欢的音乐" }
      ),
    key: "music-i-like",
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: { path: "/my-collection" },
        },
        { default: () => "我的集合" }
      ),
    key: "my-collection",
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            path: "/recently-played",
          },
        },
        { default: () => "最近播放" }
      ),
    key: "recently-played",
  },
];

export default defineComponent({
  setup() {
    return {
      menuOptions,
    };
  },
});
</script>
