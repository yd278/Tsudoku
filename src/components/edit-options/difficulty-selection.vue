<script setup lang="ts">
import { DropdownOption, NButton, NDropdown } from 'naive-ui';
import { ref } from 'vue'
components: {
      NDropdown
    }
const difficulty = ref('Easy');
const options = [
    {
        label: 'Easy',
        key: 0
    },
    {
        label: 'Medium',
        key: 1
    },
    {
        label: 'Hard',
        key: 2
    },
    {
        label: 'Algorithmist',
        key: 3
    },
    {
        label: 'Cyborg',
        key: 4
    }
];
const handleSelect = (_: string | number, option: DropdownOption):void  =>  {
    difficulty.value = String(option.label);
};

</script>

<template>
    <n-dropdown trigger="hover" :options="options" @select="handleSelect">
        <n-button>{{ difficulty }}</n-button>
    </n-dropdown>
</template>
<style scoped>
.n-button{
    width: 120px;
}
</style>