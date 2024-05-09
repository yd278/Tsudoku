<script setup lang="ts">
import singleCellPen from './single-cell-pen.vue';
import singleCellPencil from './single-cell-pencil.vue';
import { cellInfo } from '../../interfaces/cellInfo';
defineProps<{
    cellI:cellInfo
}>()
</script>

<template>
<div v-if="cellI.isPencil">
<singleCellPencil :candidates="cellI.candidates" :reverse-y="cellI.reverseY"></singleCellPencil></div>
<div v-else>
<singleCellPen :is-given="cellI.isGiven" :value="cellI.value"></singleCellPen></div>
</template>
<style scoped>
div{
    display: flex;
    flex-grow: 1;
}
</style>