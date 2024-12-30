<script setup lang="ts">

defineProps<{
    message: string
}>()
</script>

<template>
    <n-card class="card-message">
        {{ message }}
    </n-card>
</template>

<style scoped>
.card-message{
    width:80%;
}
</style>