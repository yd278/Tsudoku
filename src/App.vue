<script setup lang="ts">

import { ref } from 'vue';
import editWrapper from './components/edit-options/edit-wrapper.vue';
import numSelectionWrapper from './components/num-selection/num-selection-wrapper.vue';
import { darkTheme, NConfigProvider } from 'naive-ui';

const handleNewGameButtonClicked = (): void => { console.log('new game button clicked') }

const handleDifficultySelected = (n: number): void => {

  const message = "selected difficulty is";

  console.log(`${message} ${n}`);
}


type StatusType = 'default' | 'primary';

const parentStatuses = ref<StatusType[]>(Array(9).fill('default'));
const parentRemainingCounts = ref<number[]>(Array(9).fill(0));
</script>

<template>


<n-config-provider :theme="darkTheme">
  <div id="root">
    <div id="edit-options">
      <editWrapper @new-game-button-clicked="handleNewGameButtonClicked" @selectDifficulty="handleDifficultySelected" />
    </div>
    <div id="play-ground">
      <div id="main-area">
        <div id="board">Board</div>
        <div id="selection">
          <num-selection-wrapper v-model:remaining-counts="parentRemainingCounts" v-model:statuses="parentStatuses"/>
        </div>
      </div>
      <div id="extra-tools">Extra Tools</div>
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
  padding: 2px;
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
  background-color: #e91e63;
  /* Pink */
}

#board {
  flex-grow: 1;
  background-color: #8bc34a;
  /* Light Green */
}

#selection {
  display: flex;
  align-items: center;
  height: 75px;
}
</style>
