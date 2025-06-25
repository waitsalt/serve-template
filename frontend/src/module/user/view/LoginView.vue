<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { notify } from "@/util/notify";
import { useRouter } from "vue-router";
import { useUserStore } from "../store";
import type { CaptchaImageResponse } from "@/module/captcha/model";
import { getCaptchaImage } from "@/module/captcha/service";
import type { UserAuth, UserLoginPayload } from "../model";
import { postUserLogin } from "../service";

const router = useRouter();
const userStore = useUserStore();

const userName = ref<string>("");
const userPassword = ref<string>("");

const captchaImage = ref<string>("");
const captchaImageKey = ref<string>("");
const captchaImageValue = ref<string>("");

const loadCaptcha = async () => {
    try {
        const response: CaptchaImageResponse = (await getCaptchaImage()).data;
        captchaImage.value = response.captchaImage;
        captchaImageKey.value = response.captchaImageKey;
    } catch {
        notify.error("无法加载验证码图片");
    }
};

// 用户是否可以登陆
const userLoginCan = ref<boolean>(true);
const userLogin = async () => {
    userLoginCan.value = false;
    try {
        const userLoginPayload: UserLoginPayload = {
            userName: userName.value,
            userPassword: userPassword.value,
            captchaImageKey: captchaImageKey.value,
            captchaImageValue: captchaImageValue.value,
        };
        const response: UserAuth = (await postUserLogin(userLoginPayload)).data;
        userStore.userAuth = response;
        router.push("/");
    } catch (error) {
        notify.error(`登陆失败 ${error}`);
    }
    userLoginCan.value = true;
};

onMounted(() => {
    document.title = "用户登陆";
    loadCaptcha();
});
</script>

<template>
    <div class="loginContainer">
        <div class="loginShow">
            <div class="loginTitle">登录</div>
            <div class="loginFormItem">
                <label class="loginFromTip">用户名</label>
                <input v-model="userName" class="userInput" type="text" placeholder="请输入用户名" />
            </div>
            <div class="loginFormItem">
                <label class="loginFromTip">密码</label>
                <input v-model="userPassword" class="userInput" type="password" placeholder="请输入密码" />
            </div>
            <div class="loginFormItem">
                <label class="loginFromTip">验证码</label>
                <div class="captchaContainer">
                    <input v-model="captchaImageValue" class="userInput captchaInput" type="text"
                        placeholder="请输入验证码" />
                    <img class="captchaImage" :src="captchaImage" alt="验证码" @click="loadCaptcha()" title="点击刷新验证码" />
                </div>
            </div>
            <div class="loginTip">
                <a @click="router.push(`/user/forget`)" class="loginTipItem">忘记密码</a>
                <a @click="router.push(`/user/register`)" class="loginTipItem">没有账号</a>
            </div>
            <button class="loginButton" @click="userLogin()" :class="{ buttonDisable: !userLoginCan }"
                :disabled="!userLoginCan">
                登录
            </button>
        </div>
    </div>
</template>

<style scoped>
.loginContainer {
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
}

.loginShow {
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

.loginTitle {
    font-size: 2em;
    font-weight: 700;
}

.loginFormItem {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 5px;
}

.loginFromTip {
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

.captchaImage {
    height: 46px;
    border-radius: 8px;
    cursor: pointer;
    border: 1px solid #ddd;
    transition: transform 0.2s;
}

.loginTip {
    display: flex;
    justify-content: space-between;
    width: 100%;
}

.loginTipItem {
    color: #2980b9;
    cursor: pointer;
    font-size: 0.9em;
    text-decoration: none;
}

.loginTipItem:hover {
    color: #3498db;
}

.loginButton {
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

.loginButton:hover {
    background-color: #ceced8;
}

.buttonDisable {
    cursor: not-allowedo;
}
</style>
