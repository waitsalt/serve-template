import { useUserStore } from "@/module/user/store";
import axios from "axios";

// 普通资源
const axiosBase = axios.create({
  baseURL: "http://localhost:8000/api/v0",
  timeout: 0,
});

axiosBase.interceptors.request.use((config) => {
  return config;
});

axiosBase.interceptors.response.use(
  (response) => {
    return response.data;
  },
  (error) => {
    return Promise.reject(error);
  },
);

// 认证资源
const axiosAuth = axios.create({
  baseURL: "http://localhost:8000/api/v0",
  timeout: 0,
});

axiosAuth.interceptors.request.use((config) => {
  const userStore = useUserStore();
  const token = userStore.userAuth.accessToken;
  if (token !== null) {
    config.headers.Authorization = `Basic ${token}`;
  } else {
    alert("请登录");
  }
  return config;
});

axiosAuth.interceptors.response.use(
  (response) => {
    return response.data;
  },
  (error) => {
    return Promise.reject(error);
  },
);

// 刷新认证资源
const axiosAuthRefresh = axios.create({
  baseURL: "http://localhost:8000/api/v0",
  timeout: 0,
});

axiosAuthRefresh.interceptors.request.use((config) => {
  const userStore = useUserStore();
  const token = userStore.userAuth.refreshToken;
  if (token !== null) {
    config.headers.Authorization = `Basic ${token}`;
  } else {
    alert("请登录");
  }
  return config;
});

axiosAuthRefresh.interceptors.response.use(
  (response) => {
    return response.data;
  },
  (error) => {
    return Promise.reject(error);
  },
);

export { axiosAuth, axiosBase };
