<script setup lang="ts">
import { NButton,NIcon } from 'naive-ui';

</script>

<template>
<div class="buttons">
    <n-button :focusable="false" size="large">
        S
  </n-button>

  <n-button :focusable="false" size="large">
    R
  </n-button>

  <n-button :focusable="false" size="large">
    C
  </n-button>

  <n-button :focusable="false" size="large">
    B
  </n-button>
</div>
</template>

<style scoped>
.buttons{
    display: flex;
    justify-content: space-around;
}
.n-button{
    width: 50px;
    height:50px;
    padding:0;
    font-size: x-large;
}
</style>