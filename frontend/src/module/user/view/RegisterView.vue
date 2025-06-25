<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { notify } from "@/util/notify";
import { useRouter } from "vue-router";
import { getCaptchaEmail, getCaptchaImage } from "@/module/captcha/service";
import type { CaptchaImageResponse } from "@/module/captcha/model";
import type { UserCreatePayload } from "../model";
import { postUserCreate } from "../service";

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
const userRegisterCan = ref<boolean>(true);
const userRegister = async () => {
    if (true) {
    }
    try {
        const userCreatePayload: UserCreatePayload = {
            userName: userName.value,
            userPassword: userPassword.value,
            userEmail: userEmail.value,
            userAvatarUrl: userAvatarUrl.value,
            captchaEmail: captchaEmail.value,
            captchaImageKey: captchaImageKey.value,
            captchaImageValue: captchaImageValue.value,
        };
        const response: null = (await postUserCreate(userCreatePayload)).data;
        router.push("/user/login");
    } catch (error) {
        notify.error(`注册失败 ${error}`);
    }
};

onMounted(() => {
    document.title = "用户注册";
    loadCaptcha();
});
</script>

<template>
    <div class="registerContainer">
        <div class="registerShow">
            <div class="registerTitle">注册</div>
            <div class="registerFormItem">
                <label class="registerFromTip">用户名</label>
                <input v-model="userName" class="userInput" type="text" placeholder="请输入用户名" />
            </div>
            <div class="registerFormItem">
                <label class="registerFromTip">密码</label>
                <input v-model="userPassword" class="userInput" type="password" placeholder="请输入密码" />
            </div>
            <div class="registerFormItem">
                <label class="registerFromTip">邮箱</label>
                <input v-model="userEmail" class="userInput" type="text" placeholder="请输入邮箱" />
            </div>
            <div class="registerFormItem">
                <label class="registerFromTip">邮箱验证码</label>
                <div class="captchaContainer">
                    <input v-model="captchaEmail" class="userInput" type="text" placeholder="请输入验证码" />
                    <button class="emailSendButton" @click="captchaEmailSend"
                        :class="{ buttonDisable: !captchaEmailSendCan }" :disabled="!captchaEmailSendCan">
                        {{ captchaEmailSendBtnText }}
                    </button>
                </div>
            </div>
            <div class="registerFormItem">
                <label class="registerFromTip">验证码</label>
                <div class="captchaContainer">
                    <input v-model="captchaImageValue" class="userInput captchaInput" type="text"
                        placeholder="请输入验证码" />
                    <img class="captchaImage" :src="captchaImage" alt="验证码" @click="loadCaptcha()" title="点击刷新验证码" />
                </div>
            </div>
            <div class="registerTip">
                <a @click="router.push(`/user/forget`)" class="registerTipItem">忘记密码</a>
                <a @click="router.push(`/user/login`)" class="registerTipItem">已有账号</a>
            </div>
            <button class="registerButton" @click="userRegister()" :class="{ buttonDisable: !userRegisterCan }"
                :disabled="!userRegisterCan">
                注册
            </button>
        </div>
    </div>
</template>

<style scoped>
.registerContainer {
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
}

.registerShow {
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

.registerTitle {
    font-size: 2em;
    font-weight: 700;
}

.registerFormItem {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 5px;
}

.registerFromTip {
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

.registerTip {
    display: flex;
    justify-content: space-between;
    width: 100%;
}

.registerTipItem {
    color: #2980b9;
    cursor: pointer;
    font-size: 0.9em;
    text-decoration: none;
}

.registerTipItem:hover {
    color: #3498db;
}

.registerButton {
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

.registerButton:hover {
    background-color: #ceced8;
}

.buttonDisable {
    cursor: not-allowed;
}
</style>
