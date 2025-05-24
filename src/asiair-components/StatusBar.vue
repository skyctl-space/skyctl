<template>
    <v-sheet class="status-bar d-flex align-center justify-space-between px-4 py-2" elevation="2">
        <!-- Left content -->
        <div v-if="mainCamera.isConnected" class="d-flex align-center flex-wrap text-caption statusbar-main-group">
            <v-tooltip location="top">
                <template v-slot:activator="{ props }">
                    <div v-show="mainCamera.isConnected" v-bind="props"
                        class="d-flex align-center flex-wrap gap-2 text-caption">
                        <v-icon icon="mdi-camera" class="text-blue" />
                        <span>{{ mainCameraName }}</span>
                    </div>
                </template>
                <div class="d-flex flex-column text-body2 gap-2">
                    <div class="d-flex align-center gap-2">
                        <v-icon icon="mdi-ruler" size="small" /> Pixel Size: {{ mainCamera.info.pixel_size_um }} µm
                    </div>
                    <div class="d-flex align-center gap-2">
                        <v-icon icon="mdi-image-size-select-actual" size="small" /> Sensor Size: {{ mainCamera.info.chip_size }}
                    </div>
                    <div class="d-flex align-center gap-2">
                        <v-icon :icon="mainCamera.info.is_color ? 'mdi-palette' : 'mdi-palette-outline'" size="small" />
                        <span>
                            {{ mainCamera.info.is_color ? `Color Camera (${mainCamera.bayer_pattern || 'Unknown'})` :
                                'Mono Camera' }}
                        </span>
                    </div>
                    <div class="d-flex align-center gap-2">
                        <v-icon :icon="mainCamera.info.has_cooler ? 'mdi-snowflake' : 'mdi-weather-sunny'"
                            size="small" />
                        {{ mainCamera.info.has_cooler ? 'Cooled Camera' : 'Uncooled Camera' }}
                    </div>
                    <div class="d-flex align-center gap-2">
                        <v-icon :icon="mainCamera.info.is_usb3_host ? 'mdi-usb-port' : 'mdi-usb'" size="small" />
                        {{ mainCamera.info.is_usb3_host ? 'USB 3.0' : 'USB 2.0' }}
                    </div>
                </div>
            </v-tooltip>

            <div v-if="mainCamera.isConnected" class="d-flex align-center gap-1 ml-2">
                <v-icon icon="mdi-thermometer" class="text-blue" />
                <span>{{ mainCamera.temperature !== null ? `${mainCamera.temperature}°C` : 'N/A' }}</span>
            </div>

            <div class="d-flex align-center gap-2 statusbar-control-group">
                <v-menu v-model="gainMenuOpen" :close-on-content-click="false" location="bottom">
                    <template #activator="{ props }">
                        <v-btn v-bind="props" icon="mdi-circle" size="x-small" color="blue" variant="outlined">
                            {{ gain }}
                        </v-btn>
                    </template>
                    <v-card min-width="200" class="pa-2">
                        <div class="d-flex align-center justify-space-between mb-2">
                            <span class="text-caption">Gain (Unity Gain is {{ mainCamera.info.unity_gain }})</span>
                            <span>{{ gain }}</span>
                        </div>
                        <v-slider v-model="gain" :min="0" :max="100"
                            :step="1" hide-details thumb-label @change="applyGain" />
                    </v-card>
                </v-menu>

                <v-menu v-if="mainCamera.info.has_cooler" v-model="coolingMenuOpen" :close-on-content-click="false"
                    location="bottom">
                    <template #activator="{ props }">
                        <v-tooltip location="top" text="Cooler">
                            <template #activator="{ props: tooltipProps }">
                                <v-btn v-bind="Object.assign({}, props, tooltipProps)" icon="mdi-snowflake"
                                    variant="outlined" size="x-small" :color="mainCamera.coolerOn ? 'blue' : 'grey'">
                                </v-btn>
                            </template>
                        </v-tooltip>
                    </template>

                    <v-card min-width="200" class="pa-2">
                        <v-switch v-model="mainCamera.coolerOn" label="Cooler On" color="blue" density="compact"
                            hide-details class="my-2" />
                        <div class="d-flex align-center justify-space-between mb-2">
                            <span class="text-caption">Target Temperature</span>
                            <span>{{ targetTemperature }}°C</span>
                        </div>
                        <v-slider v-model="targetTemperature" :min="-20" :max="20" step="0.5" hide-details thumb-label
                            @change="applyTargetTemperature" />
                    </v-card>
                </v-menu>

                <div v-if="mainCamera.isConnected">
                    <v-tooltip location="top" text="AntiDew Heater">
                        <template #activator="{ props }">
                            <v-btn v-if="mainCamera.hasAntiDew" v-bind="props" variant="outlined"
                                :color="antiDewHeaterOn ? 'blue' : 'grey'" @click="toggleAntiDewHeater()" size="x-small"
                                icon="mdi-water">
                            </v-btn>
                        </template>
                    </v-tooltip>
                </div>
            </div>
        </div>

        <!-- Optional Right content -->
        <div class="text-caption text-end">
            <v-chip v-if="mainCamera.errorMessage || mainCamera.infoMessage" variant="outlined"
                class="gap-2 margin-right-4">
                <span v-if="mainCamera.errorMessage" class="text-red">{{ mainCamera?.errorMessage }}</span>
                <span v-if="mainCamera.infoMessage" class="text-blue">{{ mainCamera?.infoMessage }}</span>
            </v-chip>
            <v-progress-circular v-if="mainCamera?.isBusy" color="dark-blue" indeterminate></v-progress-circular>
        </div>
    </v-sheet>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useASIAirController } from '@/asiair-components/useASIAirController';

const props = defineProps({
    guid: {
        type: String,
        required: true,
    },
});

const { mainCamera, mainCameraName } = useASIAirController(props.guid, undefined);

const coolingMenuOpen = ref(false);
const antiDewHeaterOn = ref(false);
const targetTemperature = ref(0);
const gainMenuOpen = ref(false);
const gain = ref(0);

// watch(() => mainCamera.value.gain, (val) => {
//     gain.value = val ?? mainCamera.value.info.gain_min ?? 0;
// });

function applyGain() {
    // mainCamera.value.gain = gain.value;
    gainMenuOpen.value = false;
    // Here you would call a backend command to apply the gain
}

watch(() => mainCamera.value.targetTemperature, (val) => {
    targetTemperature.value = val;
});

function applyTargetTemperature() {
    mainCamera.value.targetTemperature = targetTemperature.value;
    coolingMenuOpen.value = false;
    // Here you would call a backend command to apply the temperature
}


function toggleAntiDewHeater() {
    antiDewHeaterOn.value = !antiDewHeaterOn.value;
    // Here you would call a backend command to toggle the anti-dew heater
}



</script>

<style scoped>
.status-bar {
    background-color: rgba(33, 33, 33, 0.6);
    /* semi-transparent dark */
    color: white;
    border-radius: 8px;
}

.statusbar-main-group > *:not(:first-child) {
    margin-left: 2.2rem;
}
.statusbar-main-group > *:nth-child(2) {
    margin-left: 0.7rem;
}
.statusbar-control-group > *:not(:first-child) {
    margin-left: 1.1rem;
}
/* Reduce spacing between the last three controls */
.statusbar-control-group > *:nth-last-child(-n+3):not(:first-child) {
    margin-left: 0.4rem;
}
</style>
