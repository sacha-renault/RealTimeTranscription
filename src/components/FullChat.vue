<script setup lang="ts">
import { ref } from 'vue'
import { NEmpty } from 'naive-ui';
import Recorder from './Recorder.vue'
import { VoskResult, VoskPartialResult } from '../vosk'

const isListening = ref(false);
const transcriptedMessages = ref<Array<string>>([]);
const currentMessage = ref("");

const onPartial = (partial: VoskPartialResult) => {
    if (isListening.value) {
        currentMessage.value = partial.result.partial;
    }
}

const onResult = (result: VoskResult) => {
    const resultText = result.result.text;
    if (isListening.value && resultText !== "") {
        transcriptedMessages.value.push(resultText);
    } 
}
</script>

<template>
    
    <div class="messages-container">
        <div class="chat-message-container">
            <n-empty v-if="transcriptedMessages.length === 0" description="No messages, start a record !"/>
            <div v-else v-for="m in transcriptedMessages">
                {{ m }}
            </div>
            <div>
                {{ currentMessage }}
            </div>
        </div>
        <recorder class="sticky-input"
            @on-partial="onPartial"
            @on-result="onResult"
            v-model="isListening"/>
    </div>
        
</template>

<style scoped>
.sticky-input {
    position: sticky;
    bottom: 0;
    z-index: 10; /* Make sure it stays on top of overlapping elements */
}

.chat-message-container{
    display: flex;
    flex-direction: column;
    gap: 2rem;
    /* background-color: blue; */
}

.messages-container{
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    overflow-y: scroll;
}

.chat-message-container{
    width: 100%;
    flex-grow: 1; /* Allow the message container to take all available space */
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 2rem;
}
</style>