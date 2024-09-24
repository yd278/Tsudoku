<script setup lang="ts">
import { NButton } from 'naive-ui'
import { useEditStore } from '../../store/edit';
import { useGridDataStore } from '../../store/gridData';
import { invoke } from "@tauri-apps/api/tauri";

type buttonClickedEvent =  () => void;

const editStore = useEditStore();
const gridDataStore = useGridDataStore();
const handleNewGameButtonClicked :buttonClickedEvent = async () => { 
  console.log(`new game button clicked with difficulty ${editStore.difficulty}`);
  var rawSudoku = "";
  rawSudoku = await invoke("generate", { name: editStore.difficulty });

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