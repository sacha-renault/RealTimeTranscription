<template>
    <n-modal v-model:show="showModal" :mask-closable="false">
        <n-card class="modal-new-transcript" title="Create a new transcription">

            <!-- Name -->
            <n-thing>
                Name
            </n-thing>
            <n-input 
                :status="isValidName(newTranscript.title) ? '' : 'error'"
                placeholder="Type a Name (min 3 words)"
                v-model:value="newTranscript.title"
                :maxlength="50"
                clearable
                show-count/>

            <n-divider/>

            <!-- Description -->
            <n-thing>
                Description (optional)
            </n-thing>
            <n-input 
                placeholder="Type a description"
                type="textarea"
                v-model:value="newTranscript.description"
                :maxlength="250"
                clearable
                show-count
                :autosize="{
                    minRows: 2,
                    maxRows: 5,
                }"/>

            <n-divider/>

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
import { defineModel, ref, defineEmits } from 'vue';
import { useMessage } from 'naive-ui';
import { 
    DismissCircle32Filled as DismissIcon,
    Airplane20Filled as ValidateIcon
} from '@vicons/fluent'
import { api } from '../api';

const showModal = defineModel({ required: true, default: false});
const message = useMessage();
const newTranscript = ref({
    title: "",
    description: ""
})
const emits = defineEmits(['newTranscript']);

const isValidName = (name: string) => {
    return name.split("").length >= 3;
}

const onValidate = () => {
    // ensure valide name
    if (isValidName(newTranscript.value.title)) {
        api.createNewChat(newTranscript.value).then(v => {
            // emit a new transcript was created
            emits('newTranscript', v);
            message.success('Transcription named "' + newTranscript.value.title + '" was created!');

            // reset transcript
            newTranscript.value = {
                title: "",
                description: ""
            }

            // then close modal
            showModal.value = false;
        }).catch(err => {
            message.error("Unexpected error :" + err);
        })
        
    } else {
        message.error("The name should contain at least 3 characters");
    }
    
}
</script>

<style>
.modal-new-transcript {
    width: 80%;
    max-width: 512px;
    max-height: 80%;
}
</style>