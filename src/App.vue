<script setup lang="ts">

import editWrapper from './components/edit-options/edit-wrapper.vue';
import { darkTheme, NConfigProvider } from 'naive-ui';

const handleNewGameButtonClicked = (): void => { console.log('new game button clicked') }

const handleDifficultySelected = (n: number): void => {

  const message = "selected difficulty is";

  console.log(`${message} ${n}`);
}
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
        <div id="selection">Selection</div>
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
  height: 75px;
  background-color: yellow;
}
</style>
