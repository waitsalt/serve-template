import type { Chapter } from "../chapter/model";

interface Roll {
  rollId: number; // 卷id
  rollName: string; // 卷名
  bookId: number; // 书id
  bookName: string; // 书名
  rollCreateTime: string; // 卷发布时间 (ISO 8601)
  rollUpdateTime: string; // 卷更新时间 (ISO 8601)
}

interface RollResponse {
  roll: Roll;
  chapterList: Chapter[];
}

export type { Roll, RollResponse };
