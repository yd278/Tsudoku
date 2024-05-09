<script setup lang="ts">
import { defineModel, computed } from 'vue';
import { NButton, NIcon } from 'naive-ui';
import { Filter9Round, FilterNoneRound, GridGoldenratioRound } from '@vicons/material';

// Define a specific type for `iconType`
type IconType = 'number' | 'territory' | 'pencil';

// Use `defineModel` with the new specific type
const iconType = defineModel<IconType>('iconType', { default: 'number' });

// Function to toggle between the three states
const toggleButtonType = () => {
  if (iconType.value === 'number') {
    iconType.value = 'territory';
  } else if (iconType.value === 'territory') {
    iconType.value = 'pencil';
  } else {
    iconType.value = 'number';
  }
};

// Computed property to determine the current icon based on the state
const currentIcon = computed(() => {
  switch (iconType.value) {
    case 'number':
      return Filter9Round;
    case 'territory':
      return FilterNoneRound;
    case 'pencil':
      return GridGoldenratioRound;
    default:
      return Filter9Round;
  }
});
</script>

<template>
  <n-button :focusable="false"  @click="toggleButtonType">
    <n-icon :component="currentIcon" />
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
