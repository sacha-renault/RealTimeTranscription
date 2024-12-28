<script setup lang="ts">
import { NPopover } from 'naive-ui';
import { Info16Regular as InfoIcon } from '@vicons/fluent'

defineProps<{
    currentMessage: string
}>()

</script>

<template>
    <n-card class="card-message">
        <n-flex align="center">
            <!-- Hover Area -->
            <n-popover trigger="hover">
                <template #trigger>
                    <InfoIcon class="icon-regular"/>
                </template>
                <span class="current-message-hover"> This is the text that is currently being transcribed. </span>
            </n-popover>

            <n-divider vertical />

            <!-- Main Content Area -->
            <span class="message-container">
                {{ currentMessage }}
                Lorem ipsum dolor sit amet, consectetur adipisicing elit. Nam, temporibus consequatur! Excepturi harum doloremque obcaecati ea, quae ut sequi error laboriosam voluptatum accusamus debitis recusandae ab sit neque mollitia itaque?
            </span>
        </n-flex>
    </n-card>
</template>

<style scoped>
.message-container {
    width: 90%;
}
</style>