import type { RouteRecordRaw } from "vue-router";
import { route as userRoute } from "./user/route";

const route: RouteRecordRaw[] = [
  ...userRoute,
];

export { route };
