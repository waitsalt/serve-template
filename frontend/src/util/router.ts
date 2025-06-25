import { route as moduleRoute } from "@/module/route";
import { route as commonRoute } from "@/common/route";
import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [...commonRoute, ...moduleRoute],
});

export default router;
