<script setup lang="ts">
import editWrapper from './components/edit-options/edit-wrapper.vue';
</script>

<template>
  <div id = "root">
    <div id="edit-options">
      <editWrapper></editWrapper>
    </div>
    <div id = "play-ground">
      <div id = "main-area">
        <div id="board">Board</div>
        <div id="selection">Selection</div>
      </div>
      <div id = "extra-tools">Extra Tools</div>
    </div>
  </div>
</template>

<style scoped>
#root {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  padding: 2px;
}
#edit-options{
  height: 50px;
  text-align: center;
}
#play-ground{
  display: flex;
  flex-grow: 1;
}
#main-area{
  display: flex;
  flex-grow: 1;
  flex-direction: column;
}
#extra-tools {
  width: 300px;
  background-color: #e91e63; /* Pink */
}
#board {
  flex-grow: 1;
  background-color: #8bc34a; /* Light Green */
}
#selection{
  height: 75px;
  background-color: yellow;
}
</style>
