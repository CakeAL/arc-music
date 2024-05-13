import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createRouter, createWebHashHistory } from "vue-router";

import Home from "./pages/Home.vue";
import About from "./pages/About.vue";
import NotFound from "./pages/NotFound.vue";
import FolderMgmt from "./pages/FolderMgmt.vue";
import MusicILike from "./pages/MusicILike.vue";
import MyCollection from "./pages/MyCollection.vue";
import RecentlyPlayed from "./pages/RecentlyPlayed.vue";
import ListOfMusic from "./pages/ListOfMusic.vue";

const routes = [
  { path: "/", component: Home },
  { path: "/about", component: About },
  { path: "/folder-mgmt", component: FolderMgmt },
  { path: "/music-i-like", component: MusicILike },
  { path: "/my-collection", component: MyCollection },
  { path: "/recently-played", component: RecentlyPlayed },
  { path: "/list-of-music/:list_title", component: ListOfMusic},
  { path: "/:catchAll(.*)", component: NotFound },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

const app = createApp(App);
app.use(router);
app.mount("#app");
