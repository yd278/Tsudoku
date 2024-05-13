<script setup lang="ts">
import {ref} from 'vue';
import preferenceWrapper from './preference-wrapper.vue';
import { NDivider } from 'naive-ui';
import HintWrapper from './hint-wrapper.vue';
import SelectionWrapper from './selection-wrapper.vue';
import auxiliaryWrapper from './auxiliary-wrapper.vue';
const isUnion = ref(false);
const mSelect = ref(false);
const numpad = ref(false)
</script>

<template>
<SelectionWrapper />
<n-divider />

<HintWrapper></HintWrapper>

<n-divider />

<auxiliaryWrapper />
<n-divider />
<preferenceWrapper v-model:isUnion="isUnion" v-model:numpad="numpad" v-model:mSelect="mSelect"/> 

</template>

<style scoped>
.n-divider{
    margin-top: 10px;
    margin-bottom: 10px;
}

</style>