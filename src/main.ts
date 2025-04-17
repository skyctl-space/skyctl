import { createApp, reactive } from "vue";

// Vuetify
import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import { createMemoryHistory, createRouter } from 'vue-router'

import TelescopesView from "./TelescopesView.vue";
//import StellariumView from "./StellariumView.vue";
import Weather from "./Weather.vue";
import HomeView from "./HomeView.vue";
import SettingsView from "./SettingsView.vue";
import { attachConsole } from '@tauri-apps/plugin-log';

// Logs from Tauri will be shown in the console
await attachConsole();

import { loadSettings } from "./store";

loadSettings().then(() => {
    console.log('Settings loaded');
}).catch((error) => {
    console.error('Error loading settings:', error);
});

const routes = [
    { path: '/', component: HomeView, name: 'home' },
    { path: '/telescopes', component: TelescopesView, name: 'telescopes' },
    //    { path: '/stellarium', component: StellariumView, name: 'stellarium' },
    { path: '/weather', component: Weather, name: 'weather' },
    { path: '/settings', component: SettingsView, name: 'settings' },
]

const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

import App from "./App.vue";

const vuetify = createVuetify({
    components,
    directives,
    icons: {
        defaultSet: 'mdi',
    },
    theme: {
        defaultTheme: 'dark', // Use system theme by default
    },
})

import { GeoLocation } from "./types";

const geoLocation = reactive<GeoLocation>({
    valid: false,
    latitude: 0,
    longitude: 0,
    error: "GeoLocation not loaded yet"
})

createApp(App)
    .provide("geoLocation", geoLocation)
    .use(router)
    .use(vuetify)
    .mount("#app");
