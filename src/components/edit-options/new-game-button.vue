<script setup lang="ts">
import { NButton } from 'naive-ui'
import { inject } from 'vue';

type buttonClickedEvent =  () => void;

const newGameEvent = inject<buttonClickedEvent>('newGameEvent');
const emitNewGameEvent = ()=>{
    if(newGameEvent){
        newGameEvent();
    }
}
</script>

<template>
<n-button :focusable=false @click="emitNewGameEvent"> New Game</n-button>  
</template>

<style scoped>
.n-button{
    width: 100px;
    padding:0;
}
</style>