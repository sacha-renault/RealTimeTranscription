<script setup lang="ts">
import { ref } from 'vue'
import { NFlex, NButton, NSwitch } from 'naive-ui'
import { SoundWaveCircle24Filled, Settings16Filled } from "@vicons/fluent"
import { defineEmits } from 'vue'
import { useThemeStore } from '../store/themeStore'
import MDrawer from './MDrawer.vue'

// For the store
const store = useThemeStore();

// Define emits for parent communication
const emit = defineEmits(['updateSwitch'])

// handle change of theme
const handleChange = (value: boolean) => {
    store.setDarkTheme(value);
    emit('updateSwitch', value);
}

// Define if the drawer is active
const drawerOut = ref(false);
</script>

<template>
    <n-flex class="m-header" justify="space-between">
        <!-- Left part of the header -->
        <n-flex align="center">
            <SoundWaveCircle24Filled class="header-logo icon-big" @click="() => drawerOut = !drawerOut"/>
            <h1>
                Real Time Transcription
            </h1>
        </n-flex>

        <!-- Right part of the header -->
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
    <m-drawer v-model="drawerOut"/>
</template>

<style scoped>
.m-header{
    padding: 24px;
}

.header-logo{
    cursor: pointer;
    transition: 0.25s ease-in-out;

    &:hover {
        transform: scale(1.2);
    }

    &:active {
        transform: scale(1.05);
    }
}
</style>