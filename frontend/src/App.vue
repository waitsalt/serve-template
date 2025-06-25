<script setup lang="ts">
import NavComp from "./common/component/NavComp.vue";
</script>

<template>
    <div class="mainContainer">
        <NavComp />
        <div class="routerViewContainer">
            <RouterView />
        </div>
    </div>
</template>

<style scoped>
.mainContainer {
    width: 100%;
    height: 100%;
    margin: 0;
    padding: 0;
    background-color: #f2f2f8;
    display: flex;
    overflow: auto;
}

.routerViewContainer {
    padding: 0 20px;
    margin-top: 60px;
    width: 100%;
    overflow: auto;
}
</style>
