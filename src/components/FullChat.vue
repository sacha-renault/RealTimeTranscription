<script setup lang="ts">
import { ref, nextTick, onMounted, onBeforeUnmount } from 'vue'
import { NEmpty, NCard } from 'naive-ui';
import Recorder from './Recorder.vue'
import { VoskResult, VoskPartialResult } from '../vosk'

const isListening = ref(false);
const transcriptedMessages = ref<Array<string>>([]);
const currentMessage = ref("");
const chatMessageContainer = ref<HTMLElement | null>(null);
let resizeObserver: ResizeObserver | null = null;

const onPartial = (partial: VoskPartialResult) => {
    if (isListening.value) {
        currentMessage.value = partial.result.partial;
        scrollToBottom();
    }
}

const onResult = (result: VoskResult) => {
    const resultText = result.result.text;
    if (isListening.value && resultText !== "") {
        transcriptedMessages.value.push(resultText);
    } 
}

const scrollToBottom = () => {
  nextTick(() => {
    if (chatMessageContainer.value) {
        chatMessageContainer.value.scrollTop = chatMessageContainer.value.scrollHeight;
    } else {
        console.log("No element to scroll");
    }
  });
};

onMounted(() => {
  chatMessageContainer.value = document.getElementById("chat-message-container") as HTMLElement;
  if (chatMessageContainer.value) {
    resizeObserver = new ResizeObserver(() => {
      scrollToBottom();
    });
    resizeObserver.observe(chatMessageContainer.value);
  }
});

onBeforeUnmount(() => {
  if (resizeObserver && chatMessageContainer.value) {
    resizeObserver.unobserve(chatMessageContainer.value);
  }
});
</script>

<template>
    <div class="messages-container">
        <div class="chat-message-container" id="chat-message-container">
            <n-empty 
                description="No messages, start a record !"
                v-if="transcriptedMessages.length === 0"/>

            <n-flex vertical align="center"
                v-else>
                <n-card v-for="m in transcriptedMessages" class="card-message">
                    {{ m }}
                </n-card>
                <n-card v-if="currentMessage !== ''" class="card-message">
                    {{ currentMessage }}
                </n-card>
            </n-flex>
            
            <!-- <c-message v-else v-for="m in transcriptedMessages" :message="m"/>
            <c-message :message="currentMessage" v-if="currentMessage.length !== 0"/> -->
            
        </div>
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
    flex-grow: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    overflow-y: scroll;
    scroll-behavior: smooth;
}

.card-message{
    width:80%;
}
</style>