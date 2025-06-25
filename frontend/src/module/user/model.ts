// 用户完整信息（内部使用）
interface User {
  userId: number;
  userName: string;
  userDesc: string;
  userPassword: string;
  userEmail: string;
  userAvatarUrl: string; // 头像 url
  userLevel: number; // 0
  userStatus: number; // 0: 正常, 1: 被封禁, 2: 删除
  userIdentity: number; // 0: 超级管理员, 1: 管理员, 2: 普通
  userCreateTime: string; // ISO日期字符串
  userUpdateTime: string; // ISO日期字符串
}

// 用户公开信息
interface UserPublic {
  userId: number;
  userName: string;
  userDesc: string;
  userEmail: string;
  userAvatarUrl: string;
  userLevel: number;
  userStatus: number;
  userIdentity: number;
  userCreateTime: string;
  userUpdateTime: string;
}

// 用户更新信息
interface UserUpdatePayload {
  userId: number;
  userName: string;
  userDesc: string;
  userEmail: string;
  userAvatarUrl: string;
  userLevel: number;
  userStatus: number;
  userIdentity: number;
  userCreateTime: string;
  userUpdateTime: string;
}

// 用户登录请求参数
interface UserLoginPayload {
  userName: string;
  userPassword: string;
  captchaImageKey: string;
  captchaImageValue: string;
}

// 用户注册请求参数
interface UserCreatePayload {
  userName: string;
  userPassword: string;
  userEmail: string;
  userAvatarUrl: string;
  captchaEmail: string;
  captchaImageKey: string;
  captchaImageValue: string;
}

// 用户搜索请求参数
interface UserSearchPayload {
  keyword: string;
}

// 用户Token声明
interface UserClaim {
  iat: number; // 颁发时间
  exp: number; // 过期时间
  data: UserPublic;
}

// 用户刷新Token声明
interface UserRefreshClaim {
  iat: number;
  exp: number;
  data: string;
}

// 用户认证信息
interface UserAuth {
  accessToken: string;
  refreshToken: string;
}

interface UserForgetPayload {
  userEmail: string,
  captchaEmail: string,
  userPassword: string,
  captchaImageKey: string,
  captchaImageValue: string,
}

export type {
  User,
  UserPublic,
  UserCreatePayload,
  UserUpdatePayload,
  UserLoginPayload,
  UserSearchPayload,
  UserClaim,
  UserRefreshClaim,
  UserAuth,
  UserForgetPayload,
};
