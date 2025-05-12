<script setup lang="ts">
import { inject, ref, Ref, watch } from 'vue'
import { TelescopeConnection } from '../types'
import ImageViewer from './ImageViewer.vue'
import RigthPanel from './RigthPanel.vue'
import LeftPanel from './LeftPanel.vue'

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

const panels = [
    { title: 'Preview'},
    { title: 'Focus'},
    { title: 'Autorun'},
    { title: 'Plan'},
    { title: 'Live'},
    { title: 'Video'}
];

const isBusy = ref(false)
const showHistogram = ref(false)
const showCrosshair = ref(false)

</script>

<template>
    <v-container fluid class="fill-height pa-0 border-0 d-flex flex-column" style="position:relative;">

        <v-window v-model="activePanel" direction="vertical" class="fill-height pa-0 border-0 window-container">
            <v-window-item v-for="(panel, index) in panels" class="fill-height pa-0 border-0">
                <div class="window-item-container fill-height d-flex flex-column position-relative overflow-hidden">
                    <!-- Watermark inside each item -->
                    <div class="watermark-text">{{ panel.title }}</div>

                   <ImageViewer :telescopeIndex="telescopeIndex" v-model:busy="isBusy" :show-histogram="showHistogram" :show-crosshair="showCrosshair" v-if="index === 0"/>
                   <ImageViewer :telescopeIndex="telescopeIndex" v-model:busy="isBusy" :show-histogram="showHistogram" :show-crosshair="showCrosshair" v-if="index === 1"/>
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

        <LeftPanel v-model:show-histogram="showHistogram" v-model:show-crosshair="showCrosshair" :autoHide="false"/>
        <RigthPanel v-model:active-panel="activePanel"/>
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