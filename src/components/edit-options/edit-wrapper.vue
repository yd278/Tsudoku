<script setup lang="ts">
import DifficultySelection from './difficulty-selection.vue';
import NewGameButton from './new-game-button.vue'
import { NDivider } from 'naive-ui';
import undo from './undo.vue';
import redo from './redo.vue'
</script>

<template>
  <div id="edit-wrapper">
    <NewGameButton/>
    <DifficultySelection />
    <n-divider vertical />
    <undo />
    <redo />

  </div>
</template>

<style scoped>
#edit-wrapper {
  display: flex;
  justify-content: flex-start;
  gap: 10px;
  align-items: center;
}
</style>