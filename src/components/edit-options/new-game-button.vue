<script setup lang="ts">
import { NButton } from 'naive-ui'
import { useEditStore } from '../../store/edit';
import { useGridDataStore } from '../../store/gridData';

type buttonClickedEvent =  () => void;

const editStore = useEditStore();
const gridDataStore = useGridDataStore();
const handleNewGameButtonClicked :buttonClickedEvent = async () => { 
  console.log(`new game button clicked with difficulty ${editStore.difficulty}`);
  var rawSudoku = "";
  try {
    rawSudoku = await window.electronAPI.generate(editStore.difficulty);
  } catch (error) {
    console.error('Error calling yourFunction:', error);
    rawSudoku = 'Error occurred';
  }

  console.log(`raw sudoku generated as:  ${rawSudoku}`) ;
  gridDataStore.setGrid(rawSudoku);
}
</script>

<template>
<n-button :focusable=false @click="handleNewGameButtonClicked"> New Game</n-button>  
</template>

<style scoped>
.n-button{
    width: 100px;
    padding:0;
}
</style>