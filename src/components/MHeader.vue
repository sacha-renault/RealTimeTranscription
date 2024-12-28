<script setup lang="ts">
import { ref } from 'vue'
import { NFlex, NSwitch } from 'naive-ui'
import { SoundWaveCircle24Filled } from "@vicons/fluent"
import { defineEmits } from 'vue'
import { useThemeStore } from '../store/themeStore'
import MDrawer from './MDrawer.vue'

// For the store
const store = useThemeStore();

// Define emits for parent communication
const emit = defineEmits(['updateSwitch', 'chatClicked'])

// handle change of theme
const handleChange = (value: boolean) => {
    store.setDarkTheme(value);
    emit('updateSwitch', value);
}

// Define if the drawer is active
const drawerOut = ref(false);
</script>

<template>
    <n-page-header subtitle="A podcast to improve designs" class="m-header">
        <n-flex>
            <n-breadcrumb>
                <n-breadcrumb-item :clickable="false">
                    <h4> Title here </h4>
                </n-breadcrumb-item>
                <n-breadcrumb-item> date here </n-breadcrumb-item>
            </n-breadcrumb>
        </n-flex>

        <template #title>
            <span style="text-decoration: none; color: inherit">
                Real Time Transcription
            </span>
        </template>

        <template #header>
            <!-- Put something in header ? -->
        </template>

        <template #avatar>
            <SoundWaveCircle24Filled class="header-logo icon-big" @click="() => drawerOut = !drawerOut"/>
        </template>

        <template #extra>
            <n-switch size="large" @update:value="handleChange">
                <template #checked-icon>
                    🌙
                </template>
                <template #unchecked-icon>
                    ☀️
                </template>
            </n-switch>
        </template>

        <template #back>
            salut :)
        </template>
    </n-page-header>

    <m-drawer v-model="drawerOut" @chat-clicked="(id) => emit('chatClicked', id)"/>
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