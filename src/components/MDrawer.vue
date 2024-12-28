<script setup lang="ts">
import { ref, defineModel, onMounted } from 'vue'
import { NList, NListItem, NInfiniteScroll, useMessage, NCard } from 'naive-ui'
import NewTranscriptModal from '../modals/NewTranscriptModal.vue';
import { 
    ClipboardTextLtr24Filled, 
    DismissCircle32Filled as DismissIcon, 
    AddCircle16Filled as AddIcon 
} from '@vicons/fluent'
import { ChatDto } from '../interfaces';
import { api } from '../api';
import { formatDate } from '../utils';

const page = ref(0);
const allChats = ref<ChatDto[]>([]);
const emit = defineEmits(['loadContent', 'chatClicked']);
const model = defineModel({ required: true, default: false});
const message = useMessage();
const showModal = ref(false);

const onLoad = async () => {
    page.value += 1;
    addChats(page.value);
    emit('loadContent', page.value);
}

const addChats = async (pageNum: number) => {
    // Get the new page 
    const newChats = await api.getChatsByPage(pageNum);

    // filter to remove duplicates
    const uniqueChats = newChats.filter(
        (chat) => !allChats.value.some((existingChat) => existingChat.id === chat.id)
    );

    // push them into existing array
    allChats.value.push(...uniqueChats);
}

const onChatClick = (chatId: number) => {
    emit('chatClicked', chatId);
    message.success("Clicked on " + chatId);
    model.value = false;
}

const onDimiss = () => {
    model.value = false;
}

const onNew = () => {
    model.value = false;
    showModal.value = true;
}

const onNewTranscript = (id: number) => {
    api.getChatById(id).then(chat => {
        allChats.value.unshift(chat);
    })
}

onMounted(async () => {
    addChats(page.value);
});
</script>

<template>
    <!-- Modal for new transcription -->
    <new-transcript-modal v-model="showModal" @new-transcript="onNewTranscript"/>

    <!-- Drawer normal content -->
    <n-drawer v-model:show="model" :width="502" placement="left">
        <n-drawer-content>
            <!-- Header -->
            <template #header>
                <n-flex align="center" justify="space-between">
                    <n-flex align="center">
                        <ClipboardTextLtr24Filled class="icon-regular"/>
                        Previous transcriptions
                    </n-flex>
                    <n-popover>
                        <template #trigger>
                            <n-button primary type="success" @click="onNew">
                                <template #icon>
                                    <AddIcon/>
                                </template>
                            </n-button>
                        </template>
                        <span> Add a new transcription </span>
                    </n-popover>
                    
                </n-flex>
            </template>

            <!-- content will be put here -->
            <n-infinite-scroll @load="onLoad">
                <n-list hoverable clickable>
                    <n-list-item 
                    class="list-item-drawer"
                        v-for="chat in allChats" :key="chat.id" @click="onChatClick(chat.id)">
                        <n-card :bordered="false" :title="chat.title" class="transparent-card">
                            {{ chat.description }}
                            <template #footer >
                                <span class="date-footer-small">
                                    {{ formatDate(chat.date) }}
                                </span>
                            </template>
                        </n-card>
                    </n-list-item>
                </n-list>
            </n-infinite-scroll>

            <!-- Footer -->
            <template #footer>
                <n-button 
                    primary
                    type="error"
                    @click="onDimiss">
                    <template #icon>
                        <DismissIcon/>
                    </template>
                </n-button>
            </template>
        </n-drawer-content>
    </n-drawer>
</template>

<style scoped>
.n-list-item.list-item-drawer {
    padding: 0px;
}

.transparent-card {
    background-color: transparent;
}

.date-footer-small{
    font-size: 10px;
    font-style: italic;
}
</style>