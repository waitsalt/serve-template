interface Comment {
  commentId: number; // 评论id
  commentReplyIs: boolean; // 是否为回复评论
  commentReplyId: number; // 父评论
  userId: number; // 用户id
  bookId: number; // 图书id
  commentContent: string; // 评论内容
  commentPraise: number; // 评论点赞
  commentCreateTime: string; // 评论发布时间 (ISO 8601)
  commentUpdateTime: string; // 评论更新时间 (ISO 8601)
}

export type { Comment };
