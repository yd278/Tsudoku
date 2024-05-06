<script setup lang="ts">
import difficultySelection from './difficulty-selection.vue';
import newGameButton from './new-game-button.vue'
</script>

<template>
<div id="edit-wrapper">
    <newGameButton> </newGameButton>
    <difficultySelection></difficultySelection>
</div>
</template>

<style scoped>
#edit-wrapper{
  display: flex;
  justify-content: flex-start;
  gap:10px;
  align-item:center;
}
</style>