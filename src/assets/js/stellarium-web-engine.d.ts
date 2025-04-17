declare module '@/assets/js/stellarium-web-engine.js' {
  interface StelWebEngineOptions {
    wasmFile: string;
    canvas: HTMLElement | null;
    translateFn?: (domain: string, str: string) => string;
    onReady?: (stel: any) => void;
  }

  const StelWebEngine: (options: StelWebEngineOptions) => void;
  export default StelWebEngine;
}