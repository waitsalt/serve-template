<script lang="ts" setup>
import { ref } from 'vue';
import { onMounted } from 'vue';

const chooseAction = ref<"info">("info")

onMounted(() => {
    document.title = "用户设置";
});
</script>

<template>
    <div class="userSettingContainer">
        <div class="leftPart">
            <div class="userSettingActionItem">用户信息</div>
        </div>
        <div class="rightPart"></div>
    </div>
</template>

<style scoped>
.userSettingContainer {
    width: 100%;
    height: 100%;
    padding: 30px 50px 100px;
    display: flex;
}

.leftPart {
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 200px;
    font-size: 1.2em;
    background-color: #FFFFFF;
    border: 1px solid #E3E5E7;
    border-radius: 10px 0 0 10px;
    overflow: auto;

}

.rightPart {
    flex-grow: 1;
    background-color: #F6F7F8;
    border: 1px solid #E3E5E7;
    border-radius: 0px 10px 10px 0px;
}

.userSettingActionUserAvatar {
    margin: 20px 0;
    height: 150px;
    border-radius: 50%;
}

.userSettingActionItem {
    width: 100%;
    padding: 10px 20px;
    border-bottom: 1px solid #E3E5E7;
    text-align: center;
}

.userSettingActionItem:hover {
    background-color: #F1F2F3;
}
</style>
