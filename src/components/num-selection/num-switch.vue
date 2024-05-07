<script setup lang="ts">
import { NButton } from 'naive-ui'
import { defineModel } from 'vue';
import { NBadge } from 'naive-ui';

const buttonType = defineModel<'default' | 'primary'>('state', {default: 'default'});
function toggleButtonType() {
  buttonType.value = buttonType.value === 'default' ? 'primary' : 'default';
}
const remaining = defineModel<number>('remaining', {default:9});
</script>

<template>
<n-button :focusable=false :type="buttonType" @click="toggleButtonType"> 
    <n-badge :value=remaining type="info" :offset="[20, -20]" >
<slot></slot></n-badge>
</n-button>  
</template>

<style scoped>
.n-button{
    width: 60px;
    height:60px;
    padding:0;
}
</style>