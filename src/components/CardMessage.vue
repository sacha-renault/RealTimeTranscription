<script setup lang="ts">
import { ChatMessageDto } from '../interfaces';
import { Edit16Regular as EditIcon } from '@vicons/fluent'
import { NCard } from 'naive-ui';
import { ref } from 'vue';

defineProps<{
    message: ChatMessageDto
}>()

const isPopOff = ref(false);

</script>

<template>
    <n-card class="card-message">
        <!-- Main Content Area -->
        <n-flex align="center">
            <span class="message-container">
                {{ message.content }}
            </span>
            <n-divider vertical />
            
            <div class="floating-edit">
                <n-popconfirm 
                    trigger="manual"
                    :show="isPopOff"
                    @positive-click="() => isPopOff=false"
                    @negative-click="() => isPopOff=false">
                    <template #trigger>
                        <n-button size="tiny" secondary type="info" @click="() => isPopOff=true">
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
}
</style>