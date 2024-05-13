<script setup>
import { useRouter } from "vue-router";
const router = useRouter();

function goback() {
  router.go(-1);
}

function goforward() {
  router.go(1);
}
</script>

<template>
  <n-button-group size="small" class="n-button-group">
    <n-button type="default" round @click="goback"> < </n-button>
    <n-button type="default" round @click="goforward"> > </n-button>
  </n-button-group>
</template>

<style scoped>
.n-button-group {
  margin-left: 175px;
  position: absolute;
  bottom: 5px;
}
</style>
