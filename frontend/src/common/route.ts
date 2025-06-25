import type { RouteRecordRaw } from "vue-router";
import HomeView from "./view/HomeView.vue";

const route: RouteRecordRaw[] = [
  {
    path: "/",
    name: "网站首页",
    component: HomeView,
  },
];

export { route };
