<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const add_list_res = ref("");

async function add_list_from_dir() {
  add_list_res.value = await invoke("add_list_from_dir");
  if (add_list_res.value !== "") {
    console.log(add_list_res.value);
  }
}

// 挂载时从后端获取列表
onMounted(() => {
  get_list_json();
});
const lists_info_res = ref("");
const lists_info = ref({});
async function get_list_json() {
  lists_info_res.value = await invoke("get_list_json");
  // console.log(lists_info_res.value);
  lists_info.value = JSON.parse(lists_info_res.value);
}

// 时间戳转换日期
function timestamp_to_date(timestamp) {
  var date = new Date(timestamp * 1000);
  var year = date.getFullYear();
  var month = date.getMonth() + 1;
  var day = date.getDate();
  var hour = date.getHours();
  var minute = date.getMinutes();
  var second = date.getSeconds();
  var formattedDate =
    year +
    "-" +
    addZero(month) +
    "-" +
    addZero(day) +
    " " +
    addZero(hour) +
    ":" +
    addZero(minute) +
    ":" +
    addZero(second);
  function addZero(num) {
    return num < 10 ? "0" + num : num;
  }
  return formattedDate;
}
</script>

<template>
  <n-scrollbar x-scrollable trigger="hover">
    <div class="folder-mgmt-div">
      <n-card
        title="添加文件夹"
        @click="add_list_from_dir"
        class="n-card"
        hoverable
      >
        <template #cover>
          <img src="../assets/plus.svg" />
        </template>
        <p>Click me to add a new folder as a music list</p>
      </n-card>
      <div v-for="(list_info, index) in lists_info" :key="index" class="n-card">
        <router-link :to="'/list-of-music/' + list_info.title" style="text-decoration:none;">
          <n-card :title="list_info.title" hoverable>
            <template #cover>
              <img :src="list_info.picture_base64" />
            </template>
            <p>创建时间：{{ timestamp_to_date(list_info.create_at) }}</p>
            <p>歌曲数量：{{ list_info.musics_num }}</p>
          </n-card>
        </router-link>
      </div>
    </div>
  </n-scrollbar>
</template>

<style scoped>
.folder-mgmt-div {
  width: 100%;
  height: calc(100vh - 100px);
  display: flex;
  overflow-x: auto;
  overflow-y: hidden;
  white-space: nowrap;
}
.n-card {
  width: 250px;
  min-width: 250px;
  min-height: 500px;
  margin: auto 5px;
  white-space: initial;
}
</style>
