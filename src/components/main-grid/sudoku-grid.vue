<script lang="ts" setup>
import { cellInfo } from '../../interfaces/cellInfo';
import singleCellWrapper from './single-cell-wrapper.vue';

import { useThemeVars } from 'naive-ui'

const lightColor = useThemeVars().value.tagColor;
const strongColor= useThemeVars().value.warningColorSuppl;
defineProps<{cellInfos : cellInfo[]}>();
const rows =[...Array(9).keys()];
const cols =[...Array(9).keys()];

function getTopStyle(row :number,_ :number){
  if(row % 3==0) return `border-top: 1px solid ${strongColor};`;
  else return `border-top: 1px solid ${lightColor};`;
}
function getLeftStyle(_ :number,col :number){
  if(col % 3==0) return `border-left: 1px solid ${strongColor};`;
  else return `border-left: 1px solid ${lightColor};`;
}
function getRightStyle(_ :number,col :number){
  if(col % 3==2) return `border-right: 1px solid ${strongColor};`;
  else return `border-right: 1px solid ${lightColor};`;
}
function getBottomStyle(row :number,_ :number){
  if(row % 3==2) return `border-bottom: 1px solid ${strongColor};`;
  else return `border-bottom: 1px solid ${lightColor};`;
}
function getCompleteBorderStyle(row: number, col: number): string {
  let topStyle = getTopStyle(row, col);
  let leftStyle = getLeftStyle(row, col);
  let rightStyle = getRightStyle(row, col);
  let bottomStyle = getBottomStyle(row, col);

  return `${topStyle} ${leftStyle} ${rightStyle} ${bottomStyle}`;
}
const containerBoard = `1px solid ${strongColor}`;
</script>

<template>
  <div class="container">
<div class="row" v-for="row in rows" :key = row>
  <div class = "square" v-for="col in cols" :key = col :style="getCompleteBorderStyle(row,col)">
    <singleCellWrapper :cell-i="cellInfos[row * 9 + col]"></singleCellWrapper>
  </div>
</div></div>
</template>



<style scoped>
.row{
  display: flex;
  flex-grow: 1;
  flex-basis: 0;
  justify-content: center;
  overflow: hidden;
}
.square{
  display: flex;
  flex: 1 0 0%;
  overflow: hidden;

}
.container{
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  aspect-ratio: 1 / 1;
  border: v-bind(containerBoard)
}
</style>
