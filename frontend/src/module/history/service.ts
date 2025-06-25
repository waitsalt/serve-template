import type { AppResponse } from "@/module/model";
import type { History } from "@/module/history/model";
import { axiosAuth } from "@/util/axios";

async function getHistoryList(): Promise<AppResponse<History[]>> {
  const response: AppResponse<History[]> = await axiosAuth.get("/history");
  return response;
}

async function getHistoryAddition(bookId: number): Promise<AppResponse<void>> {
  const response: AppResponse<void> = await axiosAuth.get(
    `/history/addition/${bookId}`
  );
  return response;
}

async function getHistoryCancel(bookId: number): Promise<AppResponse<void>> {
  const response: AppResponse<void> = await axiosAuth.get(
    `/history/cancel/${bookId}`
  );
  return response;
}

export { getHistoryList, getHistoryAddition, getHistoryCancel };
