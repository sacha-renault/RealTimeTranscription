<template>
  <n-config-provider :theme="themeUsed ? darkTheme : lightTheme">
    <n-message-provider>
      <main-vue @update-theme="handleTheme"/>
    </n-message-provider>
  </n-config-provider>
</template>

<script setup lang="ts">
import { darkTheme, lightTheme, NMessageProvider, NConfigProvider } from 'naive-ui'
import MainVue from './views/MainVue.vue'
import { ref } from 'vue'

const themeUsed = ref(false);

const handleTheme = (value: boolean) => {
  themeUsed.value = value;
}

console.log(localStorage);
</script>