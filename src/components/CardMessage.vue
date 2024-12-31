<script setup lang="ts">
import { ChatMessageDto } from '../interfaces';
import { Edit16Regular as EditIcon } from '@vicons/fluent'
import { NCard, useMessage } from 'naive-ui';
import { ref } from 'vue';
import { api } from '../api';

const props = defineProps<{
    message: ChatMessageDto
}>()
const msgProvider = useMessage();

const msgCpy = ref(props.message);
const modificationBuffer = ref("");
const isPopOff = ref(false);

const startModification = () => {
    isPopOff.value = true;
    modificationBuffer.value = msgCpy.value.content;
}

const acceptModification = () => {
    api.modifyMessage(props.message.id, modificationBuffer.value).then(() => {
        msgCpy.value.content = modificationBuffer.value;
        modificationBuffer.value = "";
    }).catch(err => {
        msgProvider.error("Unexpected error occured duing message update: " + err)
    }).finally(() => {
        isPopOff.value = false;
    })
    
}
</script>

<template>
    <n-card class="card-message">
        <!-- Main Content Area -->
        <n-flex align="center">
            <span class="message-container" v-if="!isPopOff">
                {{ msgCpy.content }}
            </span>
            <input 
                v-else 
                class="message-container input-modification" 
                v-model="modificationBuffer"/>

            <n-divider vertical />
            
            <div class="floating-edit">
                <n-popconfirm 
                    trigger="manual"
                    :show-icon="false"
                    :show="isPopOff"
                    @positive-click="acceptModification"
                    @negative-click="() => isPopOff=false">
                    Confirm message modification ?
                    <template #trigger>
                        <n-button size="tiny" secondary type="info" @click="startModification">
                            <template #icon> <EditIcon/> </template>
                        </n-button>
                    </template>
                </n-popconfirm>
            </div>
        </n-flex>
    </n-card>
</template>

<style scoped>
.card-message{
    width:80%;
}

.message-container {
    width: 90%;
    box-sizing: border-box;
    padding: 0;
}

.input-modification {
    background-color: transparent;
    color: inherit;
    border-radius: 5px;
}
</style>