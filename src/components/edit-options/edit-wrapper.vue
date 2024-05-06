<script setup lang="ts">
import difficultySelection from './difficulty-selection.vue';

  import { NButton } from 'naive-ui'
  components: {
      NButton
    }
</script>

<template>
<div id="edit-wrapper">
    <n-button> New Game</n-button>  
    <div>difficulty <difficultySelection></difficultySelection> </div>
</div>
</template>

<style scoped>
#edit-wrapper{
  display: flex;
  justify-content: flex-start;
}
</style>