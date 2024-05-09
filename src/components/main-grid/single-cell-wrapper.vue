<script setup lang="ts">
import singleCellPen from './single-cell-pen.vue';
import singleCellPencil from './single-cell-pencil.vue';
defineProps<{
    isPencil: boolean,
    candidates: boolean[],
    reverseY:boolean,
    isGiven:boolean,
    value:number
}>()
</script>

<template>
<singleCellPencil v-if="isPencil", :candidates="candidates" :reverse-y="reverseY"></singleCellPencil>
<singleCellPen v-else :is-given="isGiven" :value="value"></singleCellPen>
<style scoped>
</style>