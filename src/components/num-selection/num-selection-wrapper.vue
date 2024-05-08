<script setup lang="ts">

import numSwitch from './num-switch.vue';
import togglePencil from './toggle-pencil.vue'

const statuses = defineModel<('default' | 'primary')[]>('statuses', {
  default: Array(9).fill('default')
});
const remainingCounts = defineModel<number[]>('remainingCounts', {
  default: Array(9).fill(0)
});
const pencilStatus = defineModel<'pencil' | 'marker'>('pencilStatus',{default:'marker'});
</script>
<template>
    <div>
        <numSwitch v-for="(_,index) in remainingCounts" :key="index" v-model:remaining="remainingCounts[index]" v-model:status="statuses[index]">{{ index + 1 }}</numSwitch>
        <togglePencil v-model:icon-type="pencilStatus"> </togglePencil>
    </div>
</template>
<style scoped>
div{
    display: flex;
    align-items: center;
    gap:10px;
    padding: 20px;
}
</style>