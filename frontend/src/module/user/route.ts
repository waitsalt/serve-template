import type { RouteRecordRaw } from "vue-router";
import LoginView from "./view/LoginView.vue";
import RegisterView from "./view/RegisterView.vue";
import SettingView from "./view/SettingView.vue";
import ForgetView from "./view/ForgetView.vue";

const route: RouteRecordRaw[] = [
  {
    path: "/user/login",
    name: "用户登陆",
    component: LoginView,
  },
  {
    path: "/user/register",
    name: "用户注册",
    component: RegisterView,
  },
  {
    path: "/user/setting",
    name: "用户设置",
    component: SettingView,
  },
  {
    path: "/user/forget",
    name: "忘记密码",
    component: ForgetView,
  },
];

export { route };
