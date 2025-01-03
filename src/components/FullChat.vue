<script setup lang="ts">
import { ref, nextTick, defineProps, onMounted, watch } from 'vue'
import { NEmpty, NScrollbar, useMessage } from 'naive-ui';
import Recorder from './Recorder.vue'
import CurrentCardMessage from './CurrentCardMessage.vue';
import CardMessage from './CardMessage.vue';
import { VoskResult, VoskPartialResult } from '../vosk'
import { api } from '../api';
import { ChatMessageDto } from '../interfaces';

const isListening = ref(false);
const transcriptedMessages = ref<Array<ChatMessageDto>>([]);
const currentMessage = ref("");
const msgProvider = useMessage();

const props = defineProps<{
    chatId: number
}>();

const onPartial = (partial: VoskPartialResult) => {
    if (isListening.value) {
        currentMessage.value = partial.result.partial;
    }
}

const onResult = (result: VoskResult) => {
    // we get the result of the message
    const resultText = result.result.text;

    // ensure not empty message
    if (isListening.value && resultText !== "") {
        // Clear current
        currentMessage.value = "";

        // excecute scroll
        scrollToBottom();

        // call api to store the current message
        api.addMessage(resultText, props.chatId).then(id => {
            transcriptedMessages.value.push({
                content: resultText,
                id: id,
                chatId: props.chatId,
                date: ""
            });
        }).catch(err => {
            msgProvider.error("Unexpected error occured: " + err)
        });
    } 
}

const scrollToBottom = () => {
    nextTick(() => {
        // get container
        const container = document.getElementById("chat-message-container");
        if (!container) { 
            return;
        }

        // get child container
        const scrollContent = container.children[0];
        if (!scrollContent) {
            return;
        }

        // do scroll
        scrollContent.scrollTo({
            top: scrollContent.scrollHeight,
            behavior: "smooth",
        });
    });
};

onMounted(async () => {
    const oldMessages = await api.getMessageByChatId(props.chatId);
    transcriptedMessages.value.push(...oldMessages)
})

watch(
  () => props.chatId, // Source to watch
  async (newVal) => {
    console.log('New chat id to display', newVal);

    if (newVal !== null) {
        // reset message (new conversation)
        transcriptedMessages.value = [];

        // Stop record
        isListening.value = false;

        // Get the message of new conv on focus
        const oldMessages = await api.getMessageByChatId(props.chatId);
        transcriptedMessages.value.push(...oldMessages);
    }
  }
);
</script>

<template>
    <div class="messages-container">

        <!-- Old message part -->
        <n-scrollbar 
            ref="chatScrollbar" 
            id="chat-message-container">
            <n-empty 
                description="No messages, start a record !"
                v-if="transcriptedMessages.length === 0"/>
            <n-flex vertical v-else>
                <n-flex vertical v-for="(m, index) in transcriptedMessages" style="width: 100%;" align="center">
                    <card-message :key="index" :message="m" class="card-sizer" />
                </n-flex>               
            </n-flex> 
        </n-scrollbar>

        <n-divider/>

        <!-- Current message part -->
        <n-flex vertical align="center" style="width: 100%;">
            <current-card-message :current-message="currentMessage" class="card-sizer"/>
        </n-flex>

        <!-- Recorder part -->
        <recorder class="sticky-input"
            @on-partial="onPartial"
            @on-result="onResult"
            v-model="isListening"/>
    </div>
        
</template>

<style scoped>
.sticky-input {
    /* position: sticky; */
    bottom: 0;
    z-index: 10; 
}

.messages-container{
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.chat-message-container{
    width: 100%;
    height: 100%;
    flex-grow: 1; 
    display: flex;
    flex-direction: column;
    justify-content: center;
    scroll-behavior: smooth;
}

.separator {
    margin-bottom: 1rem;
    width: 100%;
    border: solid 1px #fefefe;
}

.card-sizer {
    max-width: var(--card-max-width);
}
</style>