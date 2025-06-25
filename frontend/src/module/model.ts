interface AppResponse<T> {
  code: number;
  message: string;
  data: T;
}

interface SqlQueryResultListWithCount<T> {
  count: number,
  list: T[]
}

export type { AppResponse, SqlQueryResultListWithCount };
