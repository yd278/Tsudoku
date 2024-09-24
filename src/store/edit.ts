import { defineStore } from 'pinia'


export const useEditStore = defineStore('edit', {
    state:()=>({
        difficulty :0
    }),
});