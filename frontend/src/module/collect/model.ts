interface Collect {
  collectId: number; // 收藏id
  userId: number; // 用户id
  bookIdList: string; // 收藏书id集合
  collectCreateTime: string; // 收藏创建时间 (ISO 8601)
  collectUpdateTime: string; // 收藏更新时间 (ISO 8601)
}

export type { Collect };
