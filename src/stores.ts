import { defineStore } from 'pinia'

interface StellariumStoreState {
    stel: any | null
    core: any | null
    tree: any | null
    currentLocation: string | null
    currentLongitude: string | null
    currentLatitude: string | null
}

export const useStellariumStore = defineStore('stellarium', {
  state: (): StellariumStoreState => {
    return {
        stel: null,
        core: null,
        tree: null,
        currentLocation: null,
        currentLongitude: null,
        currentLatitude: null,
    }
  },
})
