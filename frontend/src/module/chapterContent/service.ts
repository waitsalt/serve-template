import type { AppResponse } from "@/module/model";
import type { ChapterContent } from "@/module/chapterContent/model";
import { axiosBase } from "@/util/axios";

async function getChapterContentInfo(
  chapterId: number,
): Promise<AppResponse<ChapterContent>> {
  const res: AppResponse<ChapterContent> = await axiosBase.get(
    `/chapter_content/${chapterId}`,
  );
  return res;
}

export { getChapterContentInfo };
