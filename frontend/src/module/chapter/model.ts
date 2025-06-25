interface Chapter {
  chapterId: number; // 章id
  chapterName: string; // 章名
  rollId: number; // 卷id
  rollName: string; // 卷名
  bookId: number; // 书id
  bookName: string; // 书名
  chapterCreateTime: string; // 章发布时间 (ISO 8601 string)
  chapterUpdateTime: string; // 章更新时间 (ISO 8601 string)
}

export type { Chapter };
