<script setup lang="ts">
</script>

<template>
  <div id = "root">
    <div id="edit-options">Edit Options</div>
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
  height: 100vh;
}
#edit-options{
  padding: 10px;
  background-color: yellow;
  text-align: center;
}
#play-ground{
  display: flex;
  flex:1;
}
#main-area{
  display: flex;
  flex: 3;
  flex-direction: column;
}
#extra-tools {
  flex: 1;
  background-color: #e91e63; /* Pink */
  padding: 20px;
}
#board {
  flex:7;
  background-color: #8bc34a; /* Light Green */
  padding: 20px;
}
#selection{
  flex :1;
  background-color: yellow;
}
</style>
