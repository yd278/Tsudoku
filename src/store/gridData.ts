import { defineStore } from 'pinia'
import { operation } from '../interfaces/operation';

//TODO: implement it
export const useGridDataStore = defineStore('gridData', {
    state:()=>({
        
    }),
    actions:{
        performOperation(operation : operation) : void{

        },
        reverseOperation(operation:operation) : void{

        },
        setGrid(rawSudoku : string) : void{

        }
    }
});