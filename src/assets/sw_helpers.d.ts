declare module '@/assets/sw_helpers.js' {
  const swh: {
    initStelWebEngine: (
      store: any,
      engine: any,
      canvas: HTMLCanvasElement,
      callback: () => void
    ) => void;
    getGeolocation: () => Promise<GeolocationPosition>;
    geoCodePosition: (position: GeolocationPosition, context: any) => Promise<any>;
  };
  export default swh;
}