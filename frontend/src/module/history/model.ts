interface History {
  historyId: number; // 历史id
  userId: number; // 用户id
  bookIdList: string; // 历史书id集合
  historyCreateTime: string; // 历史发布时间 (ISO 8601)
  historyUpdateTime: string; // 历史更新时间 (ISO 8601)
}

export type { History };
