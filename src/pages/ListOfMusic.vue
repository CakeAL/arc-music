<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "vue-router";
import MusicBox from "../components/MusicBox.vue";
import ShowMusicInfo from "../components/ShowMusicInfo.vue";
// 挂载时从后端获取歌曲列表
const router = useRouter();
onMounted(() => {
  // console.log(router.currentRoute.value.params.list_title);
  get_list_of_music_json();
});
const musics_info_res = ref("");
const musics_info = ref({});
async function get_list_of_music_json() {
  musics_info_res.value = await invoke("get_list_of_music_json", {
    title: router.currentRoute.value.params.list_title + ".json",
  });
  //console.log(musics_info_res.value);
  musics_info.value = JSON.parse(musics_info_res.value);
}
</script>

<template>
  <!-- <h1>{{ $route.params.list_title }}</h1>
  {{ musics_info }} -->
  <div class="list-of-music">
    <ShowMusicInfo />
    <n-flex
      vertical
      class="n-flex"
      :native-scrollbar="false"
      id="music-box-list"
    >
      <div v-for="(music, index) in musics_info" :key="index">
        <MusicBox
          :title="music.title"
          :artist="music.artist"
          :picture_base64="music.picture_base64"
          :duration="music.duration"
          :file_path="music.file_path"
          :album="music.album"
        />
      </div>
    </n-flex>
  </div>
</template>

<style scoped>
.list-of-music {
  position: relative;
}

.n-flex {
  height: calc(100vh - 100px);
  scrollbar-color: green;
  overflow-y: auto;
}

#music-box-list {
  margin: 0 10px;
  position: absolute;
  right: 0px;
  height: calc(100vh - 100px);
}
</style>
