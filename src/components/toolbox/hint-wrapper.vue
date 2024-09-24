<script setup lang="ts">
import {AiResultsVeryHigh, AiStatusComplete, AiResultsUrgent, AiStatusFailed} from '@vicons/carbon';
import { NButton,NIcon,NCard } from 'naive-ui';
import { ref } from 'vue';

const hint = ref("");

</script>

<template>

<n-card>{{ hint }}</n-card>
<div class="buttons">
    <n-button :focusable="false" size="large">
    <n-icon><AiResultsVeryHigh/> </n-icon>
  </n-button>

  <n-button :focusable="false" size="large">
    <n-icon><AiResultsUrgent /> </n-icon>
  </n-button>

  <n-button :focusable="false" size="large">
    <n-icon><AiStatusComplete /> </n-icon>
  </n-button>

  <n-button :focusable="false" size="large">
    <n-icon><AiStatusFailed/> </n-icon>
  </n-button>
</div>
</template>

<style scoped>
.n-card{
    height: 120px;
}
.buttons{
    display: flex;
    justify-content: space-around;
}
.n-button{
    width: 50px;
    height:50px;
    padding:0;
    font-size: x-large;
}
</style>