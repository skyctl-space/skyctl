<template>
    <div class="floating-right-panel d-flex justify-center align-center">
        <v-speed-dial location="left center" transition="fade-transition">
            <template v-slot:activator="{ props: activatorProps }">
                <v-fab class="speed-dial" v-bind="activatorProps" size="small" variant="outlined"
                    :text="panelData[activePanel].title">
                </v-fab>
            </template>

            <v-tooltip v-for="(panel, index) in panelData" :text="panel.title" location="top">
                <template v-slot:activator="{ props }">
                    <v-btn :disabled="!panel.supported" v-bind="props" icon @click="goToPanel(index)">
                        <v-icon>{{ panel.icon }}</v-icon>
                    </v-btn>
                </template>
            </v-tooltip>
        </v-speed-dial>
        <v-spacer/>
        <v-select v-model="selectedBin" :items="bins" density="compact" variant="outlined"></v-select>
  
        <Shutter :exposureTime="exposureTime"/>
        <ExposureSelector v-model="exposureTime"/>
        <v-spacer/>
        <v-btn icon="mdi-download" disabled></v-btn>
    </div>

</template>

<script setup lang="ts">
import Shutter from './Shutter.vue';
import ExposureSelector from './ExposureSelector.vue';
import { ref } from 'vue';

const activePanel = defineModel<number>('activePanel', { required: true });

const exposureTime = ref(1.0); // seconds

const selectedBin = ref('Bin1'); // Default selected bin

const bins = [
    'Bin1',
    'Bin2',
    'Bin3',
    'Bin4',
  ]

// Icons for each panel
const panelData = [
    { icon: 'mdi-image', title: 'Preview', supported: true },
    { icon: 'mdi-image-filter-center-focus-weak', title: 'Focus', supported: true },
    { icon: 'mdi-auto-mode', title: 'Autorun', supported: false },
    { icon: 'mdi-list-status', title: 'Plan', supported: false },
    { icon: 'mdi-image-multiple-outline', title: 'Live', supported: false },
    { icon: 'mdi-video', title: 'Video', supported: false }
];

// Function to jump to a specific panel
function goToPanel(index: number) {
    activePanel.value = index;
}

</script>

<style scoped>
.floating-right-panel {
    position: absolute;
    top: 50%;
    right: 5px;
    transform: translateY(-50%);
    z-index: 999;
    background-color: rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.floating-right-panel v-btn {
    color: white;
}
</style>