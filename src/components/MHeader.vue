<script setup lang="ts">
import { ref } from 'vue'
import { NFlex, NButton, NSwitch } from 'naive-ui'
import { SoundWaveCircle24Filled, Settings16Filled, ClipboardTextLtr24Filled, Dismiss48Filled } from "@vicons/fluent"
import { defineEmits } from 'vue'
import { useThemeStore } from '../store/themeStore'

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
            <SoundWaveCircle24Filled class="header-logo" @click="() => drawerOut = !drawerOut"/>
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
    <n-drawer v-model:show="drawerOut" :width="502" placement="left">
        <n-drawer-content>
            <!-- Header -->
            <template #header>
                <n-flex align="center">
                    <ClipboardTextLtr24Filled style="height: 32px;"/>
                    Previous transcriptions
                </n-flex>
            </template>

            <!-- content will be put here -->
            salut moi c'est sacha

            <!-- Footer -->
            <template #footer>
                <n-button 
                    primary
                    type="error"
                    @click="() => drawerOut=false">
                    <template #icon>
                        <Dismiss48Filled/>
                    </template>
                </n-button>
            </template>
        </n-drawer-content>
    </n-drawer>
</template>

<style scoped>
.m-header{
    padding: 24px;
}

.header-logo{
    height: 40px;
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