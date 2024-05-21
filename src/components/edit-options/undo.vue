<script setup lang="ts">
import { NButton, NIcon } from 'naive-ui'
import {ArrowUndoSharp} from '@vicons/ionicons5'
import { useOperationStack } from '../../store/operationStack';
import { handleUndo } from '../../util/editFunctions';
import { useGridDataStore } from '../../store/gridData';
const operationStackStore = useOperationStack();
const gridDataStore = useGridDataStore();
const handleUndoButtonClicked = () =>{
  const operation = operationStackStore.undo();
  if(operation){
    handleUndo(operation,gridDataStore);
  }
}

</script>

<template>
<n-button :focusable=false :disabled="operationStackStore.isEmpty()" @click="handleUndoButtonClicked"> <n-icon> <ArrowUndoSharp /> </n-icon> </n-button>  
</template>

<style scoped>
</style>