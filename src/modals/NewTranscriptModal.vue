<template>
    <n-modal v-model:show="showModal" :mask-closable="false">
        <n-card class="modal-new-transcript">
            sup BOYSSS

            <template #footer>
                <n-flex justify="space-between">
                    <n-button @click="showModal = false" primary type="error">
                        <template #icon>
                            <DismissIcon/>
                        </template>
                    </n-button>
                    <n-button @click="onValidate" primary type="success">
                        <template #icon>
                            <ValidateIcon/>
                        </template>
                    </n-button>
                </n-flex>
            </template>
        </n-card>
    </n-modal>
</template>

<script setup lang="ts">
import { defineModel } from 'vue';
import { useMessage } from 'naive-ui';
import { 
    DismissCircle32Filled as DismissIcon,
    Airplane20Filled as ValidateIcon
} from '@vicons/fluent'

const showModal = defineModel({ required: true, default: false});
const message = useMessage();

const onValidate = () => {
    showModal.value = false;
    message.success("New transcription created");
}
</script>

<style>
.modal-new-transcript {
    width: 80%;
}
</style>