<script setup>
const props = defineProps([
  "title",
  "artist",
  "duration",
  "file_path",
  "picture_base64",
  "album",
]);

</script>

<template>
  <div class="music-box">
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
