declare module '@/assets/js/stellarium-web-engine.js' {
  const StelWebEngine: any;
  export default StelWebEngine;
}

declare interface Window {
  StelWebEngine?: any;
}