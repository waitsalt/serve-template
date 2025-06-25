<script lang="ts" setup>
import { onMounted } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();

onMounted(() => {
    document.title = "网站首页";
});
</script>

<template>
    <div class="homeContainer">
        <div class="selection">
            <div class="selectionTitle">最近更新</div>
        </div>
        <div class="selection">
            <div class="selectionTitle">最近更新</div>
        </div>
    </div>
</template>

<style scoped></style>
