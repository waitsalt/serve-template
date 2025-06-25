import type { AppResponse } from "@/module/model";
import type { Collect } from "@/module/collect/model";
import { axiosAuth } from "@/util/axios";

async function getCollectList(): Promise<AppResponse<Collect[]>> {
  const response: AppResponse<Collect[]> = await axiosAuth.get("/collect");
  return response;
}

async function getCollectAddition(bookId: number): Promise<AppResponse<void>> {
  const response: AppResponse<void> = await axiosAuth.get(
    `/collect/addition/${bookId}`
  );
  return response;
}

async function getCollectCancel(bookId: number): Promise<AppResponse<void>> {
  const response: AppResponse<void> = await axiosAuth.get(
    `/collect/cancel/${bookId}`
  );
  return response;
}

export { getCollectList, getCollectAddition, getCollectCancel };
