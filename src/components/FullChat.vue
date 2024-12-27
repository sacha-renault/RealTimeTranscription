<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue'
import { NEmpty, NCard } from 'naive-ui';
import Recorder from './Recorder.vue'
import { VoskResult, VoskPartialResult } from '../vosk'

const isListening = ref(false);
const transcriptedMessages = ref<Array<string>>([""]);
const chatMessageContainer = ref<HTMLElement | null>(null);

const onPartial = (partial: VoskPartialResult) => {
    if (isListening.value) {
        transcriptedMessages.value[transcriptedMessages.value.length - 1] = partial.result.partial;
        scrollToBottom();
    }
}

const onResult = (result: VoskResult) => {
    const resultText = result.result.text;
    if (isListening.value && resultText !== "") {
        transcriptedMessages.value[transcriptedMessages.value.length - 1] = resultText;
        transcriptedMessages.value.push("");
        // scrollToBottom();
    } 
}

const scrollToBottom = () => {
  nextTick(() => {
    if (chatMessageContainer.value) {
        const container = chatMessageContainer.value;
        if (container.scrollHeight > container.clientHeight) {
            container.scrollTop = container.scrollHeight;
        }
    }
  });
};

onMounted(() => {
  chatMessageContainer.value = document.getElementById("chat-message-container") as HTMLElement;
});
</script>

<template>
    <div class="messages-container">
        <div class="chat-message-container" id="chat-message-container">
            <n-empty 
                description="No messages, start a record !"
                v-if="transcriptedMessages[0].trim() === ''"/>

            <n-flex vertical v-else>
                <n-flex vertical v-for="(m, index) in transcriptedMessages" style="width: 100%;" align="center">
                    <n-card :key="index" class="card-message" v-if="m.trim() !== ''">
                        {{ m }}
                    </n-card>
                </n-flex>               
            </n-flex>
            
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