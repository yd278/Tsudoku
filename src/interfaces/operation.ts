export interface operation{
    x:number,
    y:number,
    type : "setV" | "clearV" | "setC" | "clearC"
    value : number
}