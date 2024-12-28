<script setup lang="ts">
import { ref, nextTick } from 'vue'
import { NEmpty, NCard, NPopover, NScrollbar } from 'naive-ui';
import Recorder from './Recorder.vue'
import CurrentCardMessage from './CurrentCardMessage.vue';
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
        currentMessage.value = "";
        scrollToBottom();
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

</script>

<template>
    <div class="messages-container">
        <n-scrollbar ref="chatScrollbar" id="chat-message-container">
            <div class="chat-message-container">
                <n-empty 
                    description="No messages, start a record !"
                    v-if="transcriptedMessages.length === 0"/>
                <n-flex vertical v-else>
                    <n-flex vertical v-for="(m, index) in transcriptedMessages" style="width: 100%;" align="center">
                        <n-card :key="index" class="card-message">
                            {{ m }}
                        </n-card>
                    </n-flex>               
                </n-flex> 
            </div>           
        </n-scrollbar>
        <n-divider/>
        <n-flex vertical align="center" style="width: 100%;">
            <current-card-message :current-message="currentMessage"/>
        </n-flex>
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

.card-message{
    width:80%;
}

.separator {
    margin-bottom: 1rem;
    width: 100%;
    border: solid 1px #fefefe;
}
</style>