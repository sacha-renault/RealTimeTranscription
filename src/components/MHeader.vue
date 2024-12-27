<script setup lang="ts">
import {NFlex, NButton, NSwitch} from 'naive-ui'
import { SoundWaveCircle24Filled, Settings16Filled } from "@vicons/fluent"
import { defineEmits } from 'vue'
import { useThemeStore } from '../store/themeStore'

const store = useThemeStore();

// Define emits for parent communication
const emit = defineEmits(['updateSwitch'])

const handleChange = (value: boolean) => {
    store.setDarkTheme(value);
    emit('updateSwitch', value);
}
</script>

<template>
    <n-flex class="m-header" justify="space-between">
        <!-- Left part -->
        <n-flex align="center">
            <SoundWaveCircle24Filled class="header-logo"/>
            <h1>
                Real Time Transcription
            </h1>
        </n-flex>

        <n-flex align="center">
            <n-switch size="large" @update:value="handleChange">
                <template #checked-icon>
                    🌙
                </template>
                <template #unchecked-icon>
                    ☀️
                </template>
            </n-switch>
            <n-button size="large">
                <template #icon>
                    <Settings16Filled />
                </template>
            </n-button>
        </n-flex>
    </n-flex>
</template>

<style scoped>
.m-header{
    padding: 24px;
}

.header-logo{
    height: 40px;
}
</style>