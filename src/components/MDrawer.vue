<script setup lang="ts">
import { ref, defineModel } from 'vue'
import { NList, NListItem, NInfiniteScroll, useMessage, NCard } from 'naive-ui'
import { ClipboardTextLtr24Filled, Dismiss48Filled } from '@vicons/fluent'
import { ChatDto } from '../interfaces';

const page = ref(0);
const emit = defineEmits(['loadContent', 'chatClicked']);
const model = defineModel({ required: true, default: false});
const message = useMessage();

const onLoad = () => {
    page.value += 1;
    emit('loadContent', page.value);
}

const onChatClick = (chatId: number) => {
    emit('chatClicked', chatId);
    message.success("Clicked on " + chatId);
    model.value = false;
}

const onDimiss = () => {
    model.value = false;
}
</script>

<template>
    <n-drawer v-model:show="model" :width="502" placement="left">
        <n-drawer-content>
            <!-- Header -->
            <template #header>
                <n-flex align="center">
                    <ClipboardTextLtr24Filled style="height: 32px;"/>
                    Previous transcriptions
                </n-flex>
            </template>

            <!-- content will be put here -->
            <n-infinite-scroll @load="onLoad">
                <n-list hoverable clickable>
                    <n-list-item 
                    class="list-item-drawer"
                        v-for="i in page" :key="i" @click="onChatClick(i)">
                        <n-card :bordered="false" :title="'title card ' + i" class="transparent-card">
                            number : {{ i }}
                            <template #footer>
                                date card {{ i }}
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
                        <Dismiss48Filled/>
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
</style>