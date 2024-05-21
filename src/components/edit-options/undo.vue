<script setup lang="ts">
import { NButton, NIcon } from 'naive-ui'
import {ArrowUndoSharp} from '@vicons/ionicons5'
import { useOperationStack } from '../../store/operationStack';
const operationStack = useOperationStack();

</script>

<template>
<n-button :focusable=false :disabled="operationStack.isEmpty()"> <n-icon> <ArrowUndoSharp /> </n-icon> </n-button>  
</template>

<style scoped>
</style>