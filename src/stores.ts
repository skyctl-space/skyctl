import { defineStore } from 'pinia'

interface StellariumStoreState {
    stel: any | null
    core: any | null
    tree: any | null
    currentLocation: string | null
    currentLongitude: string | null
    currentLatitude: string | null
    showCreditsDialog: boolean
    selectedObject: any | null
    selectionLayer: any | null
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
        showCreditsDialog: false,
        selectedObject: null,
        selectionLayer: null,
    }
  },
})
