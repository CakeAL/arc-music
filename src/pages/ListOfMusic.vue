<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "vue-router";
// 挂载时从后端获取歌曲列表
const router = useRouter();
onMounted(() => {
  // console.log(router.currentRoute.value.params.list_title);
  get_list_of_music_json();
});
const musics_info_res = ref("");
const musics_info = ref({});
async function get_list_of_music_json() {
  musics_info_res.value = await invoke("get_list_of_music_json", { title: router.currentRoute.value.params.list_title + ".json" });
  //console.log(musics_info_res.value);
  musics_info.value = JSON.parse(musics_info_res.value);
}
</script>

<template>
  <h1>{{ $route.params.list_title }}</h1>
  {{ musics_info }}
</template>

<style scoped></style>
