<script setup lang="ts">
import { NButton, NColorPicker } from 'naive-ui';

</script>
<template>
    <div class="buttons">
        <n-button :focusable="false" size="large">
            solid_line
        </n-button>

        <n-button :focusable="false" size="large">
            dashed_line
        </n-button>

        <n-button :focusable="false" size="large">
            color_cell
        </n-button>

        <n-button :focusable="false" size="large">
            color_candidate
        </n-button>
    </div>
    <div class="buttons">
        <n-button :focusable="false" size="large">
            erase
        </n-button>

        <n-button :focusable="false" size="large">
            clear
        </n-button>

        <n-button :focusable="false" size="large">
            undo
        </n-button>

        <n-button :focusable="false" size="large">
            redo
        </n-button>
    </div>
    <n-color-picker :show-alpha="false" :swatches="[

        'rgb(255, 255, 255)',
        '#18A058',
        '#2080F0',
        '#F0A020',
        'rgb(208, 48, 80)'
    ]" :placement="'top'" />
</template>
<style scoped>
.buttons {
    display: flex;
    justify-content: space-around;
}

.n-button {
    width: 50px;
    height: 50px;
    padding: 0;
}
</style>