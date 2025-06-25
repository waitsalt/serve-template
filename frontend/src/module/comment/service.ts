import type { AppResponse } from "@/module/model";
import type { Comment } from "@/module/comment/model";
import { axiosBase } from "@/util/axios";

async function getCommentList(): Promise<AppResponse<Comment[]>> {
  const response: AppResponse<Comment[]> = await axiosBase.get("/comment");
  return response;
}

export { getCommentList };
