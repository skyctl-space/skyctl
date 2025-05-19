<template>
    <v-container fluid class="fill-height pa-0 border-0 d-flex flex-column" style="position:relative;">
        <v-window v-model="activePanel" direction="vertical" class="fill-height pa-0 border-0 window-container">
            <v-window-item v-for="(panel, index) in panels" class="fill-height pa-0 border-0">
                <div class="window-item-container fill-height d-flex flex-column position-relative overflow-hidden">
                    <!-- Watermark inside each item -->
                    <div class="watermark-text">{{ panel.title }}</div>

                    <ImageViewer :maximized :guid="telescope_guid" :telescopeIndex="telescopeIndex" v-model:busy="isBusy" :show-histogram="showHistogram"
                        :show-crosshair="showCrosshair" v-if="index === 0" />
                    <ImageViewer :maximized :guid="telescope_guid" :telescopeIndex="telescopeIndex" v-model:busy="isBusy" :show-histogram="showHistogram"
                        :show-crosshair="showCrosshair" v-if="index === 1" />
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
                        <v-progress-linear v-if="!connectionErrorMessage" :active="connecting" height="4" indeterminate></v-progress-linear>
                        <v-messages color="red" :messages="connectionErrorMessage"
                            :active="!!connectionErrorMessage"></v-messages>
                        <v-spacer></v-spacer>
                        <!-- <v-btn v-if="connecting" icon="mdi-cancel" @click="abort_connect()"/> -->
                        <v-btn @click="connect()" :disabled="connecting">
                            Connect
                        </v-btn>
                    </template>
                </v-card>
            </v-dialog>
            <v-dialog :attach=true contained v-model="reconnectDialog" width="auto" persistent>
                <v-card max-width="400" prepend-icon="mdi-update"
                    text="Connection with the ASIAir is lost, attempting to reconnect..."
                    title="Attempting to reconnect">
                    <template v-slot:actions>
                        <v-progress-circular indeterminate></v-progress-circular>
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
            <v-spacer />
            <v-spacer />
            <v-progress-circular color="error" v-show="isBusy" indeterminate></v-progress-circular>
        </v-sheet>

        <LeftPanel v-if="maximized" v-model:show-histogram="showHistogram" v-model:show-crosshair="showCrosshair"
            :autoHide="false" />
        <RigthPanel v-if="maximized" v-model:active-panel="activePanel" :guid="telescope_guid" />
        <StatusBar v-if="maximized" :guid="telescope_guid"/>
        <MenuBar v-if="maximized" />
    </v-container>
</template>

<script setup lang="ts">
import { computed, inject, ref, Ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { listen } from "@tauri-apps/api/event";
import { TelescopeConnection } from '../types'
import ImageViewer from './ImageViewer.vue'
import RigthPanel from './RigthPanel.vue'
import LeftPanel from './LeftPanel.vue'
import StatusBar from './StatusBar.vue'
import MenuBar from './MenuBar.vue'

const { telescopeIndex = 0, maximized = false } = defineProps({
    telescopeIndex: {
        type: Number,
        required: true,
    },
    maximized: Boolean,
    shouldBeConnected: Boolean
})

const telescope_guid = computed(() => {
    return telescopes?.value?.[telescopeIndex]?.config?.guid ?? ''
})

interface ConnectionChange {
  guid: string;
  connected: boolean;
}

// Listen for the "asiair_connection_state" event
listen<ConnectionChange>("asiair_connection_state", (event) => {
  const { guid, connected } = event.payload;

  if (guid !== telescope_guid.value) {
    // Ignore events for other ASIAir devices
    return;
  }
  if (!connected) {
    reconnecting.value = true;
  } else {
    reconnecting.value = false;
  }
});


const reconnectDialog = computed(() => {
    return reconnecting.value && !disconnected.value
})

const telescopes = inject<Ref<TelescopeConnection[]>>('telescopes');

// Shows a modal when already connected and tryin got reconnect
const reconnecting = ref(false);
// Defines if we are supposed to be disconnected or connected
const disconnected = ref<boolean>(true);
// Defines if we are in the process of connecting for the first time
const connecting = ref<boolean>(false);

const connectionErrorMessage = ref('');

async function connect() {
    if (!disconnected.value) {
        // Already connected, no need to connect again
        return;
    }

    connecting.value = true;
    connectionErrorMessage.value = '';

    await invoke("asiair_attach", {
        guid: `${telescopes?.value[telescopeIndex].config.guid}`,
        connection: `${telescopes?.value[telescopeIndex].config.host}`,
    }).then(() => {
        // We are now connected
        disconnected.value = false;
        reconnecting.value = false;
        connecting.value = false;
        if (telescopes?.value[telescopeIndex]) {
            telescopes.value[telescopeIndex].connected = true;
        }
    }).catch((error: any) => {
        // We failed to connect
        connectionErrorMessage.value = error;
        connecting.value = false;
        disconnected.value = true;
        reconnecting.value = false;
    });
}

async function disconnect() {
    if (disconnected.value) {
        return;
    }

    await invoke("asiair_deattach", {
        'guid': `${telescopes?.value[telescopeIndex].config.guid}`,
    }).then(() => {
    }).catch((error: any) => {
        // We failed to disconnect
        console.log(error);
    });
    // We are now disconnected
    connecting.value = false;
    disconnected.value = true;
    reconnecting.value = false;
}

// Watch for changes in the specific telescope's connected property
watch(
    () => telescopes?.value[telescopeIndex]?.connected,
    (newConnected) => {
        // If 'connected' changes, update 'disconnected' accordingly
        if (typeof newConnected === 'boolean') {
            if (!newConnected) {
                disconnect();
            }
        }
    },
    { immediate: true } // Ensure it runs immediately to reflect initial state
);

// Active panel index
const activePanel = ref(0);

const panels = [
    { title: 'Preview' },
    { title: 'Focus' },
    { title: 'Autorun' },
    { title: 'Plan' },
    { title: 'Live' },
    { title: 'Video' }
];

const isBusy = ref(false)
const showHistogram = ref(false)
const showCrosshair = ref(false)

</script>



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