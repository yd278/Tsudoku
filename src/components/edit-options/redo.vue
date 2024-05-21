<script setup lang="ts">
import { NButton, NIcon } from 'naive-ui'
import { ArrowRedoSharp } from '@vicons/ionicons5'
import { useOperationStack } from '../../store/operationStack';
import { inject } from 'vue';
const operationStack = useOperationStack();

type buttonClickedEvent =  () => void;

const redoEvent = inject<buttonClickedEvent>('redoEvent');
const emitRedoEvent = ()=>{
    if(redoEvent){
        redoEvent();
    }
}
</script>

<template>
    <n-button :focusable=false :disabled="operationStack.noRedos()" @click="emitRedoEvent"> <n-icon>
            <ArrowRedoSharp />
        </n-icon> </n-button>
</template>

<style scoped></style>