<script setup>
import { useNowPlayingStore } from "../stores/now_playing";
const now_playing = useNowPlayingStore();
</script>

<template>
  <div class="show-music-info">
    <p class="show-title">{{ now_playing.title }}</p>
    <p>艺术家: {{ now_playing.artist }}</p>
    <p>时长: {{ now_playing.duration }} s</p>
    <n-image width="350" :src="now_playing.picture_base64" class="show-img" />
  </div>
</template>

<style scoped>
.show-music-info {
  position: absolute;
  top: 20px;
  left: 20px;
  padding: 10px;
  /* height: 100px; */
  width: 45vw;
  border: 1px solid #ddd;
}
.show-title {
  font-size: large;
}
.show-img {
  margin-left: 20px;
  /* transform: rotate(-5deg); */
  animation: wobble 5s infinite;
}

@keyframes wobble {
  0% {
    transform: rotate(-5deg);
  }
  50% {
    transform: rotate(-2deg);
  }
  100% {
    transform: rotate(-5deg);
  }
}
</style>
