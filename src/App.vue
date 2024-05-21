<script setup lang="ts">

import editWrapper from './components/edit-options/edit-wrapper.vue';
import numSelectionWrapper from './components/num-selection/num-selection-wrapper.vue';
import toolboxWrapper from './components/toolbox/toolbox-wrapper.vue';
import { darkTheme, NConfigProvider } from 'naive-ui';

import sudokuGrid from './components/main-grid/sudoku-grid.vue';
import { reactive, ref} from 'vue';
import { cellInfo } from './interfaces/cellInfo';








type StatusType = 'default' | 'primary';

const parentStatuses = ref<StatusType[]>(Array(9).fill('default'));
const parentRemainingCounts = ref<number[]>(Array(9).fill(0));


function createDefaultCell(): cellInfo {
  return {
    isPencil:false,
    candidates:Array(9).fill(false),
    reverseY:false,
    isGiven:false,
    value:1
  };
}
const cellInfos = reactive(Array.from({length:81},createDefaultCell));

cellInfos[3].isGiven=true;
cellInfos[5].isPencil = true;
cellInfos[5].candidates=[true,false,false,false,true,false,true,false,true];
cellInfos[5].reverseY = false;
</script>

<template>


<n-config-provider :theme="darkTheme">
  <div id="root">
    <div id="edit-options">
      <editWrapper/>
    </div>
    <div id="play-ground">
      <div id="main-area">
        <div id="board" ref="gridContainer">
          <sudokuGrid :cell-infos="cellInfos"></sudokuGrid>
        </div>
        <div id="selection">
          <num-selection-wrapper v-model:remaining-counts="parentRemainingCounts" v-model:statuses="parentStatuses"/>
        </div>
      </div>
      <div id="extra-tools">
        <toolboxWrapper />
      </div>
    </div>
  </div>
</n-config-provider>
</template>

<style scoped>
.n-config-provider{
  display: flex;
  width: 100%;
}
#root {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  padding: 5px;
}

#edit-options {
  display: flex;
  height: 50px;
  text-align: center;
}

#play-ground {
  display: flex;
  flex-grow: 1;
}

#main-area {
  display: flex;
  flex-grow: 1;
  flex-direction: column;
}

#extra-tools {
  width: 300px;
  display: flex;
  flex-direction: column;
  gap:10px;

  /* Pink */
}

#board {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  min-height: 0;
  justify-content: center;
  align-items: center;
  flex-shrink: 1;
  padding: 20px;
}

#selection {
  display: flex;
  align-items: center;
  height: 75px;
}
</style>
