<script setup lang="ts">
import { NButton, NIcon } from 'naive-ui'
import { ArrowRedoSharp } from '@vicons/ionicons5'
import { useOperationStack } from '../../store/operationStack';
import { handleRedo } from '../../util/editFunctions';
import { useGridDataStore } from '../../store/gridData';
const operationStackStore = useOperationStack();
const gridDataStore = useGridDataStore();
const handleRedoButtonClicked = () =>{
  const operation = operationStackStore.undo();
  if(operation){
    handleRedo(operation,gridDataStore);
  }
}

</script>

<template>
    <n-button :focusable=false :disabled="operationStackStore.noRedos()" @click="handleRedoButtonClicked"> <n-icon>
            <ArrowRedoSharp />
        </n-icon> </n-button>
</template>

<style scoped></style>