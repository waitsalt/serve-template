<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { notify } from "@/util/notify";
import { useRouter } from "vue-router";
import { getCaptchaEmail, getCaptchaImage } from "@/module/captcha/service";
import type { CaptchaImageResponse } from "@/module/captcha/model";
import type { UserCreatePayload, UserForgetPayload } from "../model";
import { postUserCreate, postUserForget } from "../service";

const router = useRouter();

const userName = ref<string>("");
const userPassword = ref<string>("");
const userEmail = ref<string>("");
const userAvatarUrl = ref<string>("");

const captchaEmail = ref<string>("");
const captchaImage = ref<string>("");
const captchaImageKey = ref<string>("");
const captchaImageValue = ref<string>("");

const captchaEmailSendCan = ref<boolean>(true);
const captchaEmailSendBtnText = ref<string>("发送");
const captchaEmailSend = async () => {
    // 如果按钮不可用，则不执行后续操作
    if (!captchaEmailSendCan.value) {
        return;
    }

    // 校验邮箱是否为空
    if (!userEmail.value) {
        notify.error("请输入邮箱地址");
        return;
    }
    // 邮箱格式校验
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    if (!emailRegex.test(userEmail.value)) {
        notify.error("请输入有效的邮箱地址");
        return;
    }

    captchaEmailSendCan.value = false;
    captchaEmailSendBtnText.value = "发送中...";
    try {
        const response: null = (await getCaptchaEmail(userEmail.value)).data;
        // 设置允许发送邮件的时间间隔
        let time = 60;
        captchaEmailSendBtnText.value = `${time}`;
        const timer = setInterval(() => {
            time--;
            captchaEmailSendBtnText.value = `${time}`;
            if (time <= 0) {
                clearInterval(timer);
                captchaEmailSendBtnText.value = "发送";
                captchaEmailSendCan.value = true;
            }
        }, 1000);
    } catch {
        notify.error("邮件发送失败");
        captchaEmailSendBtnText.value = "发送";
        captchaEmailSendCan.value = true;
    }
};

const loadCaptcha = async () => {
    try {
        const response: CaptchaImageResponse = (await getCaptchaImage()).data;
        captchaImage.value = response.captchaImage;
        captchaImageKey.value = response.captchaImageKey;
    } catch {
        notify.error("无法加载验证码图片");
    }
};
const userForgetCan = ref<boolean>(true);
const userForget = async () => {
    if (!(userEmail.value && userPassword.value && captchaEmail.value && captchaImageValue.value)) {
        notify.error("请填写完整");
        return
    }
    try {
        const userForgetPayload: UserForgetPayload = {
            userPassword: userPassword.value,
            userEmail: userEmail.value,
            captchaEmail: captchaEmail.value,
            captchaImageKey: captchaImageKey.value,
            captchaImageValue: captchaImageValue.value,
        };
        const response: null = (await postUserForget(userForgetPayload)).data;
        router.push("/user/login");
    } catch (error) {
        notify.error(`密码重设失败 ${error}`);
    }
};

onMounted(() => {
    document.title = "忘记密码";
    loadCaptcha();
});
</script>

<template>
    <div class="forgetContainer">
        <div class="forgetShow">
            <div class="forgetTitle">忘记密码</div>
            <div class="forgetFormItem">
                <div class="forgetFormItem">
                    <label class="forgetFromTip">邮箱</label>
                    <input v-model="userEmail" class="userInput" type="text" placeholder="请输入邮箱" />
                </div>
                <div class="forgetFormItem">
                    <label class="forgetFromTip">邮箱验证码</label>
                    <div class="captchaContainer">
                        <input v-model="captchaEmail" class="userInput" type="text" placeholder="请输入验证码" />
                        <button class="emailSendButton" @click="captchaEmailSend"
                            :class="{ buttonDisable: !captchaEmailSendCan }" :disabled="!captchaEmailSendCan">
                            {{ captchaEmailSendBtnText }}
                        </button>
                    </div>
                </div>
                <label class="forgetFromTip">新密码</label>
                <input v-model="userPassword" class="userInput" type="password" placeholder="请输入密码" />
            </div>
            <div class="forgetFormItem">
                <label class="forgetFromTip">验证码</label>
                <div class="captchaContainer">
                    <input v-model="captchaImageValue" class="userInput captchaInput" type="text"
                        placeholder="请输入验证码" />
                    <img class="captchaImage" :src="captchaImage" alt="验证码" @click="loadCaptcha()" title="点击刷新验证码" />
                </div>
            </div>
            <div class="forgetTip">
                <a @click="router.push(`/user/login`)" class="forgetTipItem">已有账号</a>
                <a @click="router.push(`/user/forget`)" class="forgetTipItem">没有账号</a>
            </div>
            <button class="forgetButton" @click="userForget()" :class="{ buttonDisable: !userForgetCan }"
                :disabled="!userForgetCan">
                重设密码
            </button>
        </div>
    </div>
</template>

<style scoped>
.forgetContainer {
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
}

.forgetShow {
    width: 100%;
    max-width: 500px;
    border: 1px solid #dbdbe4;
    border-radius: 10px;
    padding: 40px;
    background-color: #eeeef6;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
}

.forgetTitle {
    font-size: 2em;
    font-weight: 700;
}

.forgetFormItem {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 5px;
}

.forgetFromTip {
    font-size: 1em;
    color: #555;
    font-weight: 500;
}

.userInput {
    width: 100%;
    padding: 10px 10px;
    border: 1px solid #dbdbe4;
    box-sizing: border-box;
    border-radius: 10px;
    font-size: 1em;
}

.userInput:focus {
    outline: none;
    border-color: #3498db;
    background-color: white;
}

.userInput::placeholder {
    color: #aaa;
}

.captchaContainer {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 100%;
}

.captchaInput {
    flex: 1;
}

.emailSendButton {
    width: 40%;
    padding: 10px;
    border: none;
    border-radius: 10px;
    border: 1px solid #dbdbe4;
    background-color: white;
    font-size: 1em;
    cursor: pointer;
    transition: all 0.3s ease;
}

.emailSendButton:hover {
    background-color: #ceced8;
}

.captchaImage {
    height: 46px;
    border-radius: 8px;
    cursor: pointer;
    border: 1px solid #ddd;
    transition: transform 0.2s;
}

.forgetTip {
    display: flex;
    justify-content: space-between;
    width: 100%;
}

.forgetTipItem {
    color: #2980b9;
    cursor: pointer;
    font-size: 0.9em;
    text-decoration: none;
}

.forgetTipItem:hover {
    color: #3498db;
}

.forgetButton {
    width: 100%;
    padding: 10px;
    border: none;
    border-radius: 10px;
    border: 1px solid #dbdbe4;
    background-color: white;
    font-size: 1em;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.3s ease;
}

.forgetButton:hover {
    background-color: #ceced8;
}

.buttonDisable {
    cursor: not-allowed;
}
</style>
