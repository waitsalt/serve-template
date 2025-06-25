interface ChapterContent {
  chapterContentId: number; // 章内容id
  chapterId: number; // 章id
  chapterName: string; // 章名
  chapterContent: string; // 章内容
  rollId: number; // 卷id
  rollName: string; // 卷名
  bookId: number; // 书id
  bookName: string; // 书名
  chapterContentCreateTime: string; // 章发布时间 (ISO 8601)
  chapterContentUpdateTime: string; // 章更新时间 (ISO 8601)
}

export type { ChapterContent };
