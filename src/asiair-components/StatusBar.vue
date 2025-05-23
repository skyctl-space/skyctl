<template>
    <v-sheet class="status-bar d-flex align-center justify-space-between px-4 py-2" elevation="2">
        <!-- Left content -->
        <div :v-show="mainCamera.isConnected" class="d-flex align-center flex-wrap gap-4 text-caption">
            <v-icon icon="mdi-camera" class="text-blue" /> <span> {{ mainCameraName }} </span>
            <!-- <span>🖼 {{ resolution }}</span> -->
            <!-- <span>🌡 {{ temperature }}°C</span>
            <span>💧 Anti-dew: {{ antiDew ? 'On' : 'Off' }}</span>
            <span>❄ Cooler: {{ coolerPercent }}%</span> -->
        </div>

        <!-- Optional Right content -->
        <div class="text-caption text-end">
            <v-chip v-if="mainCamera.errorMessage || mainCamera.infoMessage" variant="outlined" class="gap-2 margin-right-4">
            <span v-if="mainCamera.errorMessage" class="text-red">{{ mainCamera?.errorMessage }}</span>
            <span v-if="mainCamera.infoMessage" class="text-blue">{{ mainCamera?.infoMessage }}</span>
            </v-chip>
            <v-progress-circular v-if="mainCamera?.isBusy" color="dark-blue" indeterminate></v-progress-circular>
        </div>
    </v-sheet>
</template>

<script setup lang="ts">
import { useASIAirController } from '@/asiair-components/useASIAirController';

const props = defineProps({
  guid: {
    type: String,
    required: true,
  },
});

const { mainCamera, mainCameraName } = useASIAirController(props.guid, undefined);


// const resolution = '4144×2822'
// const temperature = '-10.5'
// const antiDew = true
// const coolerPercent = 72
</script>

<style scoped>
.status-bar {
    background-color: rgba(33, 33, 33, 0.6);
    /* semi-transparent dark */
    color: white;
    border-radius: 8px;
}

.gap-4>*:not(:last-child) {
    margin-right: 1rem;
}
</style>
