<script setup lang="ts">
import { ref } from 'vue'
import { NFlex, NSwitch, useMessage } from 'naive-ui'
import { SoundWaveCircle24Filled, Options16Regular as OptionIcon } from "@vicons/fluent"
import { defineEmits } from 'vue'
import { useThemeStore } from '../store/themeStore'
import MDrawer from './MDrawer.vue'
import { formatDate } from '../utils'
import { ChatDto } from '../interfaces'
import { api } from '../api'

// For the store
const store = useThemeStore();
const msgProvider = useMessage();
// Define emits for parent communication
const emit = defineEmits(['updateSwitch', 'chatClicked'])

// handle change of theme
const handleChange = (value: boolean) => {
    store.setDarkTheme(value);
    emit('updateSwitch', value);
}

// When we receive the event of new click on chat
const onChatClicked = (id: number) => {
    emit('chatClicked', id);
    api.getChatById(id).then(d => {
        chat.value = d;
    }).catch(err => {
        msgProvider.error("Unexpected error: " + err);
    })
}

// open option modal?
const onOptionClick = () => {
    msgProvider.warning('Options clicked');
}

// Define if the drawer is active
const drawerOut = ref(false);
const chat = ref<ChatDto | null>(null)
</script>

<template>
    <n-page-header class="m-header">
        <n-flex>
            <n-breadcrumb>
                <n-breadcrumb-item :clickable="false">
                    <h4> {{ chat ? chat.title : "--"}} </h4>
                </n-breadcrumb-item>
                <n-breadcrumb-item>
                    {{ chat ? formatDate(new Date(chat.date)) : '--'}}
                </n-breadcrumb-item>
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
            <n-flex align="center">
                <n-switch size="large" @update:value="handleChange">
                    <template #checked-icon>
                        ☀️
                    </template>
                    <template #unchecked-icon>
                        🌙
                    </template>
                </n-switch>
                <n-button round @click="onOptionClick">
                    <template #icon>
                        <OptionIcon/>
                    </template>
                </n-button>
            </n-flex>
        </template>
    </n-page-header>

    <m-drawer v-model="drawerOut" @chat-clicked="onChatClicked"/>
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