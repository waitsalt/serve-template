import type { AppResponse } from "@/module/model";
import type {
  UserAuth,
  UserCreatePayload,
  UserForgetPayload,
  UserLoginPayload,
  UserPublic,
  UserSearchPayload,
  UserUpdatePayload,
} from "@/module/user/model";
import { useUserStore } from "@/module/user/store";
import { axiosAuth, axiosBase } from "@/util/axios";

async function postUserCreate(
  userCreatePayload: UserCreatePayload,
): Promise<AppResponse<null>> {
  const response: AppResponse<null> = await axiosBase.post(
    "/user",
    userCreatePayload,
  );
  return response;
}

async function userDelete(userId: string): Promise<AppResponse<null>> {
  const response: AppResponse<null> = await axiosAuth.delete(`/user/${userId}`);
  return response;
}

async function userUpdate(
  userUpdatePayload: UserUpdatePayload,
): Promise<AppResponse<UserAuth>> {
  const response: AppResponse<UserAuth> = await axiosAuth.patch(
    `/user/${userUpdatePayload.userId}`,
    userUpdatePayload,
  );
  return response;
}

async function userSearch(
  userSearchPayload: UserSearchPayload,
): Promise<AppResponse<UserPublic[]>> {
  const response: AppResponse<UserPublic[]> = await axiosBase.post(
    `/user/search`,
    userSearchPayload,
  );
  return response;
}

async function postUserLogin(
  userLoginPayload: UserLoginPayload,
): Promise<AppResponse<UserAuth>> {
  const response: AppResponse<UserAuth> = await axiosBase.post(
    `/user/login`,
    userLoginPayload,
  );
  return response;
}

async function getUserLogout(): Promise<AppResponse<null>> {
  const userStore = useUserStore();
  userStore.userAuth = {
    accessToken: "",
    refreshToken: "",
  };
  const response: AppResponse<null> = await axiosAuth.get(`/user/logout`);
  return response;
}

async function userInfo(userId: string): Promise<AppResponse<UserPublic>> {
  const response: AppResponse<UserPublic> = await axiosBase.get(
    `/user/${userId}`,
  );
  return response;
}

async function userRefreshToken(): Promise<AppResponse<UserAuth>> {
  const response: AppResponse<UserAuth> = await axiosAuth.post(`/user/refresh`);
  return response;
}

async function postUserForget(
  userForgetPayload: UserForgetPayload,
): Promise<AppResponse<null>> {
  const response: AppResponse<null> = await axiosBase.post(
    "/user/forget",
    userForgetPayload,
  );
  return response;
}

export {
  postUserCreate,
  userDelete,
  userUpdate,
  userSearch,
  postUserLogin,
  getUserLogout,
  userInfo,
  userRefreshToken,
  postUserForget
};
