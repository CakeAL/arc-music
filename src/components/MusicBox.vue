<script setup>
import { invoke } from "@tauri-apps/api/tauri";
import { useNowPlayingStore } from "../stores/now_playing";
// props
const props = defineProps([
  "title",
  "artist",
  "duration",
  "file_path",
  "picture_base64",
  "album",
]);
// play a music
async function playMusic() {
  const event = {
    action: "play",
    file_path: props.file_path,
  };
  await invoke("handle_music_event", { event: JSON.stringify(event) }).catch(
    (error) => console.log(error)
  );
  const now_playing = useNowPlayingStore()
  now_playing.title = props.title;
  now_playing.artist = props.artist;
  now_playing.duration = props.duration;
  now_playing.file_path = props.file_path;
  now_playing.picture_base64 = props.picture_base64;
  now_playing.album = props.album;
  console.log(now_playing);
}
</script>

<template>
  <div class="music-box" @click="playMusic">
    <n-image width="60" :src="props.picture_base64" class="music-box-image" />
    <div>
      <p class="music-box-p">{{ props.title }}</p>
      <p class="music-box-p">{{ props.artist }}</p>
    </div>
  </div>
</template>

<style scoped>
.music-box {
  padding: 2px;
  width: 250px;
  display: flex;
  height: 60px;
  margin: 2px 5px;
  border: 1px solid #888;
}

.music-box:hover {
  box-shadow: 0 4px 8px rgba(82, 60, 60, 0.3);
}

.music-box-image {
  margin-right: 5px;
}
.music-box-p {
  width: 190px;
  margin: 5px 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
