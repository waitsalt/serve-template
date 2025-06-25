import type { AppResponse } from "@/module/model";
import type { CaptchaImageResponse } from "@/module/captcha/model";
import { axiosBase } from "@/util/axios";

async function getCaptchaEmail(userEmail: string): Promise<AppResponse<null>> {
  const response: AppResponse<null> = await axiosBase.get(
    `/captcha/email/${userEmail}`,
  );
  return response;
}

async function getCaptchaImage(): Promise<AppResponse<CaptchaImageResponse>> {
  const response: AppResponse<CaptchaImageResponse> =
    await axiosBase.get(`/captcha/image`);
  return response;
}

async function getCaptchaPhone(userPhone: string): Promise<AppResponse<null>> {
  const response: AppResponse<null> = await axiosBase.get(
    `/captcha/phone/${userPhone}`,
  );
  return response;
}

export { getCaptchaEmail, getCaptchaImage, getCaptchaPhone };
