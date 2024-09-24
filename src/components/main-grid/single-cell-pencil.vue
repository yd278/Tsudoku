<script setup lang="ts">
defineProps<{
    reverseY: boolean,
    candidates:boolean[]
}>()
</script>
<template>
    <div v-if="!reverseY" class="cell">
        <div class="row">
            <div :class="['num', { 'hidden': !candidates[0] }]">1</div>
            <div :class="['num', { 'hidden': !candidates[1] }]">2</div>
            <div :class="['num', { 'hidden': !candidates[2] }]">3</div>
        </div>
        <div class="row">
            <div :class="['num', { 'hidden': !candidates[3] }]">4</div>
            <div :class="['num', { 'hidden': !candidates[4] }]">5</div>
            <div :class="['num', { 'hidden': !candidates[5] }]">6</div>
        </div>
        <div class="row">
            <div :class="['num', { 'hidden': !candidates[6] }]">7</div>
            <div :class="['num', { 'hidden': !candidates[7] }]">8</div>
            <div :class="['num', { 'hidden': !candidates[8] }]">9</div>
        </div>
    </div>
    <div v-else class="cell">
        <div class="row">
            <div :class="['num', { 'hidden': !candidates[6] }]">7</div>
            <div :class="['num', { 'hidden': !candidates[7] }]">8</div>
            <div :class="['num', { 'hidden': !candidates[8] }]">9</div>
        </div>
        <div class="row">
            <div :class="['num', { 'hidden': !candidates[3] }]">4</div>
            <div :class="['num', { 'hidden': !candidates[4] }]">5</div>
            <div :class="['num', { 'hidden': !candidates[5] }]">6</div>
        </div>
        <div class="row">
            <div :class="['num', { 'hidden': !candidates[0] }]">1</div>
            <div :class="['num', { 'hidden': !candidates[1] }]">2</div>
            <div :class="['num', { 'hidden': !candidates[2] }]">3</div>
        </div>
    </div>
</template>
<style scoped>
.cell {
    display: flex;
    flex-direction: column;
    flex-basis: 0;
}

.row {
    display: flex;
    flex-grow: 1;
    flex-basis: 0;
}

.num {
    display: flex;
    font-size: small;
    justify-content: center;
    align-items: center;
    color: darkgrey;
    flex-grow: 1;
    flex-basis: 0;

}

.hidden {
    visibility: hidden;
}
</style>