import { reactive } from 'vue'
import { Settings } from './types'
import { Store } from '@tauri-apps/plugin-store'

export const settings = reactive<Settings>(new Settings());
export var store : Store | null = null; 

export const loadSettings = async () => {
    store = await Store.load('settings.json')
    await settings.loadSettings(store);
}

export const saveSettings = async () => {
    if (store) {
        console.log("Saving settings to store");
        await settings.saveSettings(store);
        console.log("Saved");
    } else {
        console.error("Store is not initialized");
    }
}