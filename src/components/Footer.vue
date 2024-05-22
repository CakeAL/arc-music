<script setup>
import { useNowPlayingStore } from "../stores/now_playing";
import { ref, onMounted, onUpdated } from "vue";
// import { useFps } from '@vueuse/core'

const now_playing = useNowPlayingStore();
let has_played = ref(0.0);
let duration = now_playing.duration;
const progressBarWidth = ref("0%");
let timer;
// const fps = useFps();

onMounted(() => {
  duration = now_playing.duration;
  has_played.value = 0;
  calculateProgressBarWidth();
});

watch(now_playing, () => {
  has_played.value = 0;
  duration = now_playing.duration;
  calculateProgressBarWidth();
  timer = setInterval(hasPlayedAddOne, 16.6666666667);
});

const calculateProgressBarWidth = () => {
  progressBarWidth.value = `${(has_played.value / duration) * 100}%`;
  //   console.log(duration);
  //   console.log(progressBarWidth.value);
};

// 鼠标按下事件处理
const startDragging = (event) => {
  const startX = event.clientX;
  const initialWidth = parseFloat(progressBarWidth.value);

  const handleMouseMove = (event) => {
    const diff = event.clientX - startX;
    let newWidth =
      initialWidth + (diff / event.target.parentNode.offsetWidth) * 100;

    // 限制拖动范围在 0% 到 100%
    if (newWidth < 0) {
      newWidth = 0;
    } else if (newWidth > 100) {
      newWidth = 100;
    }

    progressBarWidth.value = `${newWidth}%`;
    has_played.value = Math.round((newWidth / 100) * duration); // 更新时间
  };

  const handleMouseUp = () => {
    window.removeEventListener("mousemove", handleMouseMove);
    window.removeEventListener("mouseup", handleMouseUp);
  };

  window.addEventListener("mousemove", handleMouseMove);
  window.addEventListener("mouseup", handleMouseUp);
};

const hasPlayedAddOne = () => {
  //   console.log(has_played.value);
  if (has_played.value <= duration) {
    has_played.value += 0.0166666667;
  } else {
    has_played.value = duration;
    clearInterval(timer);
  }
  calculateProgressBarWidth();
};
</script>

<template>
  <div class="progress-container">
    <div class="progress-bar" :style="{ width: progressBarWidth }"></div>
    <div
      class="draggable-border"
      ref="draggableBorder"
      @mousedown="startDragging"
      :style="{ left: progressBarWidth }"
    ></div>
  </div>
</template>

<style scoped>
.progress-container {
  width: 100vw;
  height: 50px;
  position: relative;
}

.progress-bar {
  height: 100%;
  background-color: #d8efe5;
}

.draggable-border {
  width: 5px;
  height: 100%;
  background-color: #18a058;
  position: absolute;
  top: 0;
  cursor: ew-resize;
}
</style>
