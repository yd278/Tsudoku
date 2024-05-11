<script setup>
import {ref} from 'vue';
import preferenceWrapper from './preference-wrapper.vue';
const isUnion = ref(false);
const mSelect = ref(false);
const numpad = ref(false)
</script>

<template>

<preferenceWrapper v-model:isUnion="isUnion" v-model:numpad="numpad" v-model:mSelect="mSelect"/> 

</template>

<style scoped>

</style>