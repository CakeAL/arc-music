<template>
  <n-menu :options="menuOptions" />
</template>

<script>
import { defineComponent, h } from "vue";

const menuOptions = [
  {
    label: () =>
      h(
        "a",
        {
          href: "#/",
        },
        { default: () => "主页" }
      ),
    key: "home",
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/about",
        },
        { default: () => "关于" }
      ),
    key: "about",
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/folder-mgmt",
        },
        { default: () => "文件夹管理" }
      ),
    key: "folder-mgmt",
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/music-i-like",
        },
        { default: () => "我喜欢的音乐" }
      ),
    key: "music-i-like",
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/my-collection",
        },
        { default: () => "我的集合" }
      ),
    key: "my-collection",
  },
  {
    label: () =>
      h(
        "a",
        {
          href: "#/recently-played",
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
