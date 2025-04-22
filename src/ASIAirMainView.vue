<script setup lang="ts">
import { inject, ref, Ref, watch, onMounted } from 'vue'
import { TelescopeConnection } from './types'
import { invoke } from '@tauri-apps/api/core';
import { listen, Event } from "@tauri-apps/api/event";

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
    await invoke('load_and_emit_fits_png', { telescopeIndex : telescopeIndex, displayWidth: window.innerWidth, displayHeight: window.innerHeight });
}

const canvasRefs: Record<number, HTMLCanvasElement | null> = {};

interface ImageUpdateEvent {
    index: number;
    data: string;
}

const updateImage = (event: Event<ImageUpdateEvent>) => {
    const { index, data } = event.payload;

    if (index !== telescopeIndex) {
        // Not for us, ignore
        return;
    }

    const canvas = canvasRefs[index];
    if (!canvas) return;

    if (canvas instanceof HTMLCanvasElement) {
        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        const img = new Image();

        // This is expected to be a base64 encoded string
        // data = 'data:image/png;base64,' + data;
        img.src = data;
        img.onload = () => {
            canvas.width = img.width;
            canvas.height = img.height;
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            ctx.drawImage(img, 0, 0);
        };
        img.onerror = (error) => {
            console.error('Error loading image:', error);
        };
    } else {
        console.error('Canvas ref is not an HTMLCanvasElement');
    }
};

onMounted(async () => {
  await listen<ImageUpdateEvent>('fits_image_updated', updateImage);
});
</script>

<template>
    <v-container fluid class="fill-height pa-0 border-0 d-flex flex-column">

        <!-- v-window with 5 panels -->
        <v-window v-model="activePanel" direction="vertical" class="fill-height pa-0 border-0 window-container">
            <v-window-item v-for="panel in panelData" class="fill-height pa-0 border-0">
                <div class="window-item-container fill-height d-flex flex-column position-relative overflow-hidden">
                    <!-- Watermark inside each item -->
                    <div class="watermark-text">{{ panel.title }}</div>
                    <!-- <v-img v-if="index === 0" src="/gaia_milkyway.jpg" class="flex-grow-1 w-100 h-100 pa-0 ma-0" 
                    </v-img> -->

                    <div class="canvas-wrapper fill-height fill-width">
                        <canvas :ref="el => canvasRefs[telescopeIndex] = el as HTMLCanvasElement" class="fits-canvas"></canvas>
                    </div>
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
            <span>Status: Connected</span>
            <span>ASIAIR @ 192.168.1.2</span>
        </v-sheet>

        <v-btn class="floating-btn" icon @click="loadFits()">
            <v-icon>mdi-refresh</v-icon>
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

.status-bar {
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 40px;
    padding: 0 16px;
    background-color: rgba(50, 50, 50, 0.2);
    /* Semi-transparent dark background */
    color: white;
    display: flex;
    align-items: center;
    justify-content: space-between;
    pointer-events: none;
    /* Makes it "pass through" mouse events unless needed */
    z-index: 9;
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
    z-index: 0;
    white-space: nowrap;
    text-align: center;
}
</style>