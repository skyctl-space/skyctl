<script setup lang="ts">
import { inject, ref, Ref, watch } from 'vue'
import { TelescopeConnection } from './types'
import { invoke } from '@tauri-apps/api/core';
import ImageViewer from './ImageViewer.vue'

const { telescopeIndex = 0 } = defineProps({
    telescopeIndex: Number
})

const telescopes = inject<Ref<TelescopeConnection[]>>('telescopes');

const disconnected = ref<boolean>(true);

const connecting = ref(false);
let connectTimeout: ReturnType<typeof setTimeout>;

function connect() {
    connecting.value = true;
    connectTimeout = setTimeout(() => {
        disconnected.value = false;
        connecting.value = false;
        if (telescopes && telescopes.value[telescopeIndex]) {
            telescopes.value[telescopeIndex].connected = true;
        }
    }, 1000);
}

function abort_connect() {
    connecting.value = false;
    disconnected.value = true;
    clearTimeout(connectTimeout);
}

// Watch for changes in the specific telescope's connected property
watch(
    () => telescopes?.value[telescopeIndex]?.connected,
    (newConnected) => {
        // If 'connected' changes, update 'disconnected' accordingly
        if (typeof newConnected === 'boolean') {
            disconnected.value = !newConnected;
        }
    },
    { immediate: true } // Ensure it runs immediately to reflect initial state
);

// Active panel index
const activePanel = ref(0);

// Icons for each panel
const panelData = [
    { icon: 'mdi-image', title: 'Preview' },
    { icon: 'mdi-image-filter-center-focus-weak', title: 'Focus' },
    { icon: 'mdi-auto-mode', title: 'Autorun' },
    { icon: 'mdi-list-status', title: 'Plan' },
    { icon: 'mdi-video', title: 'Video' }
];

// Function to jump to a specific panel
function goToPanel(index: number) {
    activePanel.value = index;
}

async function loadFits() {
    // Never render the preview in a resolution larger than the screen
    isBusy.value = true
    await invoke('load_fits_image', { telescopeIndex: telescopeIndex, displayWidth: window.innerWidth, displayHeight: window.innerHeight });
}
   

const isBusy = ref(false)

// const binModes = [
//     { title: 'Bin1', value: 0 },
//     { title: 'Bin2', value: 1 },
//     { title: 'Bin3', value: 2 },
//     { title: 'Bin4', value: 3 }
// ]

// const binMode = ref(0)

const showHistogram = ref(false)

</script>

<template>
    <v-container fluid class="fill-height pa-0 border-0 d-flex flex-column">

        <!-- v-window with 5 panels -->
        <v-window v-model="activePanel" direction="vertical" class="fill-height pa-0 border-0 window-container">
            <v-window-item v-for="(panel, index) in panelData" class="fill-height pa-0 border-0">
                <div class="window-item-container fill-height d-flex flex-column position-relative overflow-hidden">
                    <!-- Watermark inside each item -->
                    <div class="watermark-text">{{ panel.title }}</div>
                    <!-- <v-img v-if="index === 0" src="/gaia_milkyway.jpg" class="flex-grow-1 w-100 h-100 pa-0 ma-0" 
                    </v-img> -->
                   <ImageViewer :telescopeIndex="telescopeIndex" v-model:busy="isBusy" :show-histogram="showHistogram" v-if="index === 0"/>
                </div>
            </v-window-item>

            <v-dialog :attach=true contained v-model="disconnected" width="100%" persistent>
                <v-card prepend-icon="mdi-connection" title="Disconnected">
                    <template v-slot:default>
                        <v-card-text>
                            <p v-if="connecting">Trying to connect to the ASIAir at {{
                                telescopes?.[telescopeIndex]?.config?.host }}...</p>
                            <p v-else>You are not connected to the ASIAir at {{
                                telescopes?.[telescopeIndex]?.config?.host
                                }}</p>
                        </v-card-text>
                    </template>
                    <template v-slot:actions>
                        <v-progress-linear :active="connecting" height="4" indeterminate></v-progress-linear>
                        <v-spacer></v-spacer>
                        <v-btn v-if="connecting" icon="mdi-cancel" @click="abort_connect()">
                        </v-btn>
                        <v-btn @click="connect()" :disabled="connecting">
                            Connect
                        </v-btn>
                    </template>
                </v-card>
            </v-dialog>
        </v-window>

        <!-- Absolute Status Bar -->
        <v-sheet class="status-bar" elevation="6">
            <!-- <v-menu v-model="menu" :close-on-content-click="false" location="end">
                <template v-slot:activator="{ props }">
                    <v-btn v-bind="props">
                        ASI 2600MC Bin1 (1.0 °C)
                    </v-btn>
                </template>
            </v-menu> -->
            <v-spacer/>
            <v-spacer/>
            <v-progress-circular color="error" v-show="isBusy" indeterminate></v-progress-circular>
        </v-sheet>

        <v-btn class="floating-btn" icon @click="loadFits()">
            <v-icon>mdi-refresh</v-icon>
        </v-btn>

        <v-btn class="floating-btn-histogram" icon @click="showHistogram = !showHistogram">
            <v-icon>mdi-chart-histogram</v-icon>
        </v-btn>

        <v-speed-dial location="left center" transition="fade-transition">
            <template v-slot:activator="{ props: activatorProps }">
                <v-fab class="speed-dial" v-bind="activatorProps" size="large" icon>
                    <v-icon>{{ panelData[activePanel].icon }}</v-icon>
                </v-fab>
            </template>

            <v-tooltip v-for="(panel, index) in panelData" :text="panel.title" location="top">
                <template v-slot:activator="{ props }">
                    <v-btn v-bind="props" icon @click="goToPanel(index)">
                        <v-icon>{{ panel.icon }}</v-icon>
                    </v-btn>
                </template>
            </v-tooltip>
        </v-speed-dial>
    </v-container>
</template>

<style scoped>
.fits-canvas {
    width: 100%;
    height: 100%;
    height: auto;
    display: block;
    background: black;
}

.floating-btn {
    position: absolute;
    top: 70px;
    right: 16px;
    z-index: 999;
}

.floating-btn-histogram {
    position: absolute;
    top: 170px;
    right: 16px;
    z-index: 999;
}

.status-bar {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 40px;
    padding: 0 16px;
    background-color: rgba(50, 50, 50, 0.6);
    /* Semi-transparent dark background */
    color: white;
    display: flex;
    align-items: center;
    justify-content: space-between;
    /* pointer-events: none; */
    /* Makes it "pass through" mouse events unless needed */
    /* z-index: 9; */
    /* Below Vuetify dialogs (which are typically z-index 10+) */
}

.window-container {
    position: relative;
    /* Ensure positioning inside v-window */
    width: 100%;
    /* Make the window fill the width of the parent */
}

.speed-dial {
    position: absolute;
    right: 16px;
    top: 50%;
    transform: translateY(-50%);
    z-index: 1000;
    /* Make sure it's above other components */
}

.window-item-container {
    position: relative;
    min-height: 100%;
}

.watermark-text {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%) rotate(-45deg);
    font-size: 6rem;
    font-weight: bold;
    color: rgba(0, 0, 0, 0.05);
    pointer-events: none;
    z-index: -1;
    white-space: nowrap;
    text-align: center;
}
</style>