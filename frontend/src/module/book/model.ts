import type { RollResponse } from "../roll/model";

interface Book {
  bookId: number; // 书id
  bookName: string; // 书名
  bookDesc: string; // 简介
  bookClass: string; // 书分类
  bookTag: string; // 书标签
  bookHeat: number; // 书浏览量
  bookCollect: number; // 书收藏量
  bookRecommend: number; // 书推荐量
  bookCoverUrl: string; // 书封面url
  bookWordCount: number; // 书字数
  bookLatestChapterId: number; // 书最新章节id
  bookLatestChapterName: string; // 书最新章节名
  bookCreateTime: string; // 书发布时间 (ISO 8601 string)
  bookUpdateTime: string; // 书更新时间 (ISO 8601 string)
}

interface BookListQuery {
  bookClass?: string;
  bookTag?: string;
  limit?: number;
  page?: number;
}

interface BookSearchQuery {
  limit?: number;
  page?: number;
  bookName?: string;
}

interface BookCatalogResponse {
  rollList: RollResponse[];
}

export type { Book, BookListQuery, BookSearchQuery, BookCatalogResponse };
