<script setup lang="ts">
import { ref, defineEmits, onBeforeUnmount } from 'vue'
import { NSelect, NInputGroup, useMessage, SelectOption } from 'naive-ui'
import { initialize } from '../vosk'
import { Record12Regular, RecordStop28Regular } from '@vicons/fluent'
import type { VoskRecognitionObject } from '../vosk'


const emits = defineEmits(['onPartial', 'onResult', 'update:modelValue']);
const message = useMessage();

const modelLoading = ref(false);
const voskObject = ref<VoskRecognitionObject | null>(null);
const isListening = ref(false);
const timer = ref<number | null>(null);
const recordingDuration = ref(0);

// initialize available models
const modelFiles = import.meta.glob("/public/models/*.zip"); // Adjust extension as needed
const availableModels = ref(
  Object.keys(modelFiles).map((path) => {
    const fileName = path.split("/").pop() || "";
    return {
      label: fileName.replaceAll("-", " ").replace(".zip", ""),
      value: "/models/" + fileName,
    };
  })
);

// When select a model, it loads it 
const loadModel = async (_: string, option: SelectOption) => {
    // Terminate old model if there was one to avoid aving lots of models
    // loaded at once
    if (voskObject.value !== null) {
        voskObject.value.model.terminate();
    }

    // if the recording was on we stop it
    if (isListening.value) {
        buttonListenClick();
    }

    // Ensure option.value is a valid string
    const modelValue = option.value as string | undefined;

    // Check if modelValue is valid
    if (typeof modelValue !== 'string') {
        throw new Error('Invalid model value');
    }

    try {
        // new instance of a model
        modelLoading.value = true;
        voskObject.value = await initialize(modelValue);
        modelLoading.value = false;
    } catch (error) {
        modelLoading.value = false;
        console.error('Failed to initialize model:', error);
        message.error("An error occured when loading the model :" + error)
    }

    // setup callbacks
    if (voskObject.value !== null) {
        voskObject.value.recognizer.on("result", (result) => {
            emits("onResult", result);
        });
        voskObject.value.recognizer.on("partialresult", (partial) => {
            emits("onPartial", partial);
        });
    }
}

// handle listen click
const buttonListenClick = () => {
    // switch listening
    isListening.value = !isListening.value;
    emits('update:modelValue', isListening.value);

    // start the timer if it's recording
    if (isListening.value) {
        recordingDuration.value = 0; // Reset time when starting
        timer.value = setInterval(() => {
        recordingDuration.value += 1; // Increase the timer every second
        }, 1000);
        message.success("Recording started");
    } else {
        if (timer.value !== null) {
            clearInterval(timer.value);
        }
        message.success("Recording stopped");
    }
}

// Format time to hh:mm:ss
const formatTime = (seconds: number) => {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;
  return `${String(hours).padStart(2, '0')}:${String(minutes).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
};

onBeforeUnmount(() => {
  // Clear the interval when the component is destroyed
  if (timer.value) {
    clearInterval(timer.value);
  }
});
</script>

<template>
    <n-flex class="recorder-container">
        <n-input-group class="w-full record-input-group">
            <n-button disabled size="large" primary dashed :type="isListening ? 'info' : 'default'">
                {{ isListening ? formatTime(recordingDuration) : '00:00:00' }}
            </n-button>
            <n-select class="select-model" 
                :options="availableModels" 
                @update:value="loadModel"
                :disabled="modelLoading"
                placeholder="Please select a recognition model"
                :loading="modelLoading"
                size="large"/>
            <n-button 
                size="large" 
                :type="isListening ? 'info' : 'error'"
                @click="buttonListenClick"
                :disabled="voskObject===null">
                <template #icon>
                    <RecordStop28Regular v-if="isListening"/>
                    <Record12Regular v-else-if="!isListening"/>
                </template>
            </n-button>
        </n-input-group>
    </n-flex>
</template>


<style scoped>

.recorder-container {
    padding: 2rem;
}
.select-model{
    min-width: 20rem;
    max-width: 512px;
}
.record-input-group{
    justify-content: center;
}
</style>