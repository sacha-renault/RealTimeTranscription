<script setup lang="ts">
import MHeader from "../components/MHeader.vue";
import FullChat from "../components/FullChat.vue";
import { defineEmits, onMounted, ref, onUnmounted } from "vue";

const chatHeight = ref<number>(0);
const emit = defineEmits(["updateTheme"]);

// Methods
const calculateRemainingHeight = () => {
  const viewportHeight = window.innerHeight;
  const header = document.querySelector('.header')?.getBoundingClientRect().height || 0;
  const footer = document.querySelector('.footer')?.getBoundingClientRect().height || 0;
  chatHeight.value = viewportHeight - header - footer;
};

const handleTheme = (value: boolean) => {
  emit("updateTheme", value);
};

// Vue compoenent 
onMounted(() => {
  calculateRemainingHeight();

  // Recalculate on resize
  window.addEventListener('resize', calculateRemainingHeight);
});

onUnmounted(() => {
  window.removeEventListener('resize', calculateRemainingHeight);
});
</script>

<template>
  <n-layout>
    <!-- Header -->
    <n-layout-header bordered class="header">
      <m-header @update-switch="handleTheme" />
    </n-layout-header>

    <!-- Main Content Area -->
    <n-layout :style="{ 'height': chatHeight + 'px' }">
        <!-- FullChat with Fixed Height -->
        <full-chat style="height: 100%"/>
      </n-layout>
  </n-layout>
</template>

<style scoped>

</style>