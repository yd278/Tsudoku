// src/electron.d.ts
export {};

declare global {
  interface Window {
    electronAPI: {
        generate: (difficulty: number ) => Promise<string>;
    }
  }
}
