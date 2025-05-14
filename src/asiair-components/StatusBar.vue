<template>
    <v-sheet class="status-bar d-flex align-center justify-space-between px-4 py-2" elevation="2">
        <!-- Left content -->
        <div class="d-flex align-center flex-wrap gap-4 text-caption">
            <span>📷 {{ cameraName }}</span>
            <span>🖼 {{ resolution }}</span>
            <span>🌡 {{ temperature }}°C</span>
            <span>💧 Anti-dew: {{ antiDew ? 'On' : 'Off' }}</span>
            <span>❄ Cooler: {{ coolerPercent }}%</span>
        </div>

        <!-- Optional Right content -->
        <div class="text-caption text-end">
            <!-- Put other info here if needed -->
        </div>
    </v-sheet>
</template>

<script setup lang="ts">
const cameraName = 'ASI294MC Pro'
const resolution = '4144×2822'
const temperature = '-10.5'
const antiDew = true
const coolerPercent = 72
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
