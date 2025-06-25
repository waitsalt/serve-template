<script lang="ts" setup>
import { getUserLogout } from "@/module/user/service";
import { useUserStore } from "@/module/user/store";
import { ref } from "vue";
import { onMounted } from "vue";
import { watch } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();
const userStore = useUserStore();

const userActionPanelShowCan = ref<boolean>(false);
const userActionPanelShowTimer = ref<number | undefined>(undefined);
const userActionPanelShow = () => {
    if (userActionPanelShowTimer.value) {
        clearTimeout(userActionPanelShowTimer.value);
    }
    userActionPanelShowCan.value = true;
};
const userActionPanelNotShow = () => {
    if (userActionPanelShowTimer.value) {
        clearTimeout(userActionPanelShowTimer.value);
    }
    userActionPanelShowTimer.value = setTimeout(() => {
        userActionPanelShowCan.value = false;
    }, 800);
};

const checkUserLogin = () => {
    const currentPath: string = router.currentRoute.value.path;
    const accessToken = userStore.userAuth.accessToken;
    console.log(currentPath);
    // 用户登陆后 不能访问的界面
    if (
        accessToken !== "" &&
        ["/user/login", "/user/register"].includes(currentPath)
    ) {
        router.push("/");
    }
    // 用户没登陆 不能访问的界面
    if (
        accessToken === "" &&
        ["/user/setting"].includes(currentPath)
    ) {
        router.push("/");
    }
};

watch(
    [
        () => router.currentRoute.value.path,
        () => userStore.userAuth.accessToken,
    ],
    async () => {
        checkUserLogin();
    },
);

onMounted(() => {
    checkUserLogin();
});
</script>

<template>
    <div class="navContainer">
        <div class="leftPart">
            <div class="logoSelection">
                <img @click="router.push('/')" class="logoImage" src="/src/asset/iamge/default.png" alt="logo" />
            </div>
        </div>
        <div class="centerPart"></div>
        <div class="rightPart">
            <div class="userSelection">
                <div class="userNoLoginShow" v-show="userStore.userAuth.accessToken === ''">
                    <div class="userNoLoginAction" @click="router.push('/user/login')">
                        登陆
                    </div>
                    <div class="userNoLoginAction" @click="router.push('/user/register')">
                        注册
                    </div>
                </div>
                <div class="userLoginShow" v-show="userStore.userAuth.accessToken !== ''">
                    <img class="userAvatar" @mouseenter="userActionPanelShow()" @mouseleave="userActionPanelNotShow()"
                        alt="头像" src="/src/asset/iamge/default.png" />
                    <div class="userActionPanel" v-show="userActionPanelShowCan" @mouseenter="userActionPanelShow()"
                        @mouseleave="userActionPanelNotShow()">
                        <button class="userActionItem" @click="router.push('/user/setting')">
                            用户设置
                        </button>
                        <hr class="userActionSeparation" />
                        <button class="userActionItem" @click="getUserLogout()">
                            退出登陆
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.navContainer {
    box-sizing: border-box;
    position: fixed;
    width: 100vw;
    top: 0;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 20px;
}

.leftPart,
.rightPart,
.centerPart,
.userNoLoginShow {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 5px;
}

.logoSelection {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.userNoLoginAction {
    background-color: white;
    border: 1px solid #dbdbe4;
    border-radius: 10px;
    padding: 5px 15px;
    cursor: default;
    font-size: 1em;
    transition: 0.3s ease all;
}

.userNoLoginAction:hover {
    background-color: #ceced8;
}

.logoImage,
.userAvatar {
    width: 40px;
    height: 40px;
    border: 1px solid #dbdbe4;
    border-radius: 50%;
    object-fit: cover;
}

.userActionPanel {
    width: 200px;
    top: 60px;
    right: 20px;
    padding: 10px;
    background-color: #eeeef6;
    border: 1px solid #dbdbe4;
    border-radius: 10px;
    position: fixed;
    transition: 0.3s ease all;
}

.userActionSeparation {
    color: #E3E5E7;
}

.userActionItem {
    width: 100%;
    border: 1px solid #dbdbe4;
    border-radius: 10px;
    padding: 5px;
    font-size: 1em;
    transition: 0.3s ease all;
}

.userActionItem:hover {
    background-color: #ceced8;
}
</style>
