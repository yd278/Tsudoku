<script setup lang="ts">
import { ref } from 'vue';
import { defineModel } from 'vue';
import { NButton, NIcon } from 'naive-ui';
import { PencilAlt, Marker } from '@vicons/fa';

// Define the v-model using `defineModel`
const iconType = defineModel<'pencil' | 'marker'>('iconType', { default: 'marker' });

// Determine whether the current icon is the pencil
const isPencilIcon = ref(iconType.value === 'pencil');

// Toggle function to switch and update the model value
function toggleButtonType() {
  isPencilIcon.value = !isPencilIcon.value;
  iconType.value = isPencilIcon.value ? 'pencil' : 'marker';
}
</script>

<template>
  <n-button :focusable="false"  @click="toggleButtonType">
    <n-icon>
      <component :is="isPencilIcon ? PencilAlt : Marker" />
    </n-icon>
  </n-button>
</template>

<style scoped>
.n-button {
  width: 60px;
  height: 60px;
  padding: 0;
  font-size: xx-large;
}
</style>
