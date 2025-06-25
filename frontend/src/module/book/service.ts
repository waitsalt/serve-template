import type { AppResponse, SqlQueryResultListWithCount } from "@/module/model";
import type {
  Book,
  BookCatalogResponse,
  BookListQuery,
  BookSearchQuery,
} from "@/module/book/model";
import { axiosBase } from "@/util/axios";

async function getBookList(
  bookListQuery: BookListQuery,
): Promise<AppResponse<SqlQueryResultListWithCount<Book>>> {
  const response: AppResponse<SqlQueryResultListWithCount<Book>> =
    await axiosBase.get("/book", {
      params: bookListQuery,
    });
  return response;
}

async function getBookSearch(
  bookSearchQuery: BookSearchQuery,
): Promise<AppResponse<SqlQueryResultListWithCount<Book>>> {
  const response: AppResponse<SqlQueryResultListWithCount<Book>> =
    await axiosBase.get("/book/search", {
      params: bookSearchQuery,
    });
  return response;
}

async function getBookInfo(bookId: number): Promise<AppResponse<Book>> {
  const response: AppResponse<Book> = await axiosBase.get(`/book/${bookId}`);
  return response;
}

async function getBookCatalog(
  bookId: number,
): Promise<AppResponse<BookCatalogResponse>> {
  const res: AppResponse<BookCatalogResponse> = await axiosBase.get(
    `/book/${bookId}/catalog`,
  );
  return res;
}

async function getBookComment(
  bookId: number,
): Promise<AppResponse<Comment[]>> {
  const res: AppResponse<Comment[]> = await axiosBase.get(
    `/book/${bookId}/comment`,
  );
  return res;
}

export {
  getBookList,
  getBookSearch,
  getBookInfo,
  getBookCatalog,
  getBookComment,
};
