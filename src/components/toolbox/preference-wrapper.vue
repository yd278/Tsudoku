<script setup lang="ts">
import { NDivider } from 'naive-ui';
import { NList, NListItem } from 'naive-ui';
import { NSwitch, NIcon } from 'naive-ui';
import { ShapeUnion20Regular, ShapeIntersect20Regular } from '@vicons/fluent'
const isUnion = defineModel('isUnion', { default: false })
const numpad = defineModel('numpad', { default: false })
const mSelect = defineModel('mSelect', { default: false })
</script>

<template>


  <n-list hoverable>
    <n-list-item>
        <div class="option-item">
          Selection Option:
          <n-switch v-model:value="isUnion" size="large">
            <template #checked-icon>
              <n-icon :component="ShapeUnion20Regular" />
            </template>
            <template #unchecked-icon>
              <n-icon :component="ShapeIntersect20Regular" />
            </template>
          </n-switch>
        </div>

    </n-list-item>

    <n-list-item>
      <div class="option-item">
        Multi-select:

        <n-switch v-model:value="mSelect" size="large" />
      </div>
    </n-list-item>
    <n-list-item>

      <div class="option-item">
        Numpad Mode:
        <n-switch v-model:value="numpad" size="large" />

      </div>
    </n-list-item>
  </n-list>
  <!-- selection option -->


  <!-- numpad mod -->
</template>

<style scoped>
.option-item {
  display: flex;
  justify-content: space-between
}
</style>