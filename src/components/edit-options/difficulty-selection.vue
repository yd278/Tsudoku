<script setup lang="ts">
import { DropdownOption, NButton, NDropdown } from 'naive-ui';
import { ref } from 'vue'
import { useEditStore } from '../../store/edit';

const difficulty = ref('Difficulty');
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
const editStore = useEditStore();
const handleSelect = (key: number, option: DropdownOption):void  =>  {
    difficulty.value = String(option.label);
    editStore.difficulty = key;
};

</script>

<template>
    <n-dropdown trigger="hover" :options="options" @select="handleSelect">
        <n-button :focusable=false>{{ difficulty }}</n-button>
    </n-dropdown>
</template>
<style scoped>
.n-button{
    width: 100px;
    padding:0;
}
</style>