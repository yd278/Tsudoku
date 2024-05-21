import { defineStore } from 'pinia';
import { operation } from '../interfaces/operation';
export const useOperationStack = defineStore('operationStack', {
    state: () => ({
        operations: [] as operation[],
        redoStack: [] as operation[]
    }),
    actions: {
        performOperation(newOperation: operation): void {
            this.operations.push(newOperation);
            // Clear the redo stack when a new operation is performed
            this.redoStack = [];
        },
        undo(): operation | undefined {
            if (this.operations.length === 0) return undefined;
            const operation = this.operations.pop();
            if (operation) this.redoStack.push(operation);
            return operation;
        },
        redo(): operation | undefined {
            if (this.redoStack.length === 0) return undefined;
            const operation = this.redoStack.pop();
            if (operation) this.operations.push(operation);
            return operation;
        },
        isEmpty(): boolean {
            return this.operations.length === 0;
        },
        noRedos(): boolean {
            return this.operations.length === 0;
        },
        clear(): void {
            this.operations = [];
            this.redoStack = [];
        }
    }
});