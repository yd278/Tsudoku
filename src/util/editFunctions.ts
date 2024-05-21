import { operation } from "../interfaces/operation";
import { useGridDataStore } from "../store/gridData";
//TODO: implement two functions
export function handleRedo(operation : operation, gridDataStore : ReturnType<typeof useGridDataStore>) :void {
    console.log("handleRedo called")
}
export function handleUndo(operation : operation, gridDataStore : ReturnType<typeof useGridDataStore>) :void {
    console.log("handleUndo called")

}