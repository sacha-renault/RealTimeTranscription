<template>
  <n-config-provider :theme="themeUsed ? darkTheme : lightTheme">
    <n-modal-provider>
      <n-message-provider>
        <main-vue @update-theme="handleTheme"/>
        <n-global-style />
      </n-message-provider>
    </n-modal-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { darkTheme, lightTheme, NMessageProvider, NConfigProvider, NGlobalStyle, NModalProvider } from 'naive-ui'
import MainVue from './views/MainVue.vue'
import { ref } from 'vue'

const themeUsed = ref(false);

const handleTheme = (value: boolean) => {
  themeUsed.value = value;
}

console.log(localStorage);
</script>