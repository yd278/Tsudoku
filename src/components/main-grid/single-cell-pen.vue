<script setup lang="ts">
defineProps<{
  isGiven: boolean
  value: number
}>()
import { useThemeVars } from 'naive-ui'
const givenColor = useThemeVars().value.primaryColorSuppl;
</script>
<template>
<div :class="{nGiven:!isGiven}">{{value}}</div>
</template>
<style scoped>
div{
    display: flex;
    font-size:xx-large;
    font-weight: bolder;
}
.nGiven{
    color: v-bind(givenColor)
}
</style>