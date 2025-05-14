<template>
  <v-sheet
    class="menu-bar d-flex align-center justify-end px-4 py-2"
    elevation="2"
  >
    <v-menu
      v-for="(item, index) in menuItems"
      :key="item.name"
      v-model="menuIndex"
      :model-value="menuIndex === index"
      :close-on-content-click="false"
      transition="scale-transition"
      offset-y
    >
      <template #activator="{ props }">
        <v-btn
          v-bind="props"
          :icon="item.icon"
          variant="text"
          size="small"
          color="white"
          class="mx-1"
          @click.stop="toggleMenu(index)"
        >
          <v-icon>{{ item.icon }}</v-icon>
        </v-btn>
      </template>

      <!-- Put each menu content in its own v-if block -->
      <v-card class="pa-4" width="250">
        <template v-if="item.name === 'Settings'">
          <h4 class="text-subtitle-1 mb-2">Settings</h4>
          <v-switch label="Dark Mode" />
          <v-slider label="Brightness" :max="100" :step="1" />
        </template>

        <template v-else-if="item.name === 'Camera'">
          <h4 class="text-subtitle-1 mb-2">Camera</h4>
          <v-switch label="Cooling" />
          <v-slider label="Gain" :max="100" :step="1" />
        </template>

        <template v-else-if="item.name === 'Guide'">
          <h4 class="text-subtitle-1 mb-2">Guide</h4>
          <v-switch label="Enable Guiding" />
          <v-select :items="['Aggressive', 'Moderate', 'Passive']" label="Aggression" />
        </template>

        <template v-else-if="item.name === 'Mount'">
          <h4 class="text-subtitle-1 mb-2">Mount</h4>
          <v-btn block>Park</v-btn>
          <v-btn block>Slew Home</v-btn>
        </template>

        <template v-else-if="item.name === 'EFW'">
          <h4 class="text-subtitle-1 mb-2">Filter Wheel</h4>
          <v-select :items="['L', 'R', 'G', 'B', 'Ha']" label="Filter" />
        </template>

        <template v-else-if="item.name === 'AutoFocus'">
          <h4 class="text-subtitle-1 mb-2">AutoFocus</h4>
          <v-btn block>Start Focus</v-btn>
          <v-switch label="Auto on Filter Change" />
        </template>

        <template v-else-if="item.name === 'Rotator'">
          <h4 class="text-subtitle-1 mb-2">Rotator</h4>
          <v-slider label="Angle" :max="360" :step="1" />
        </template>

        <template v-else-if="item.name === 'Storage'">
          <h4 class="text-subtitle-1 mb-2">Storage</h4>
          <v-progress-linear :value="65" color="blue" height="8" rounded />
          <div class="text-caption mt-2">65% used</div>
        </template>

        <template v-else-if="item.name === 'Info'">
          <h4 class="text-subtitle-1 mb-2">Information</h4>
          <p class="text-caption">ASIAIR v1.2.3</p>
          <p class="text-caption">Camera: ASI2600MC</p>
        </template>
      </v-card>
    </v-menu>
  </v-sheet>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const menuIndex = ref<number | null>(null)

function toggleMenu(index: number) {
  menuIndex.value = menuIndex.value === index ? null : index
}

const menuItems = [
  { name: 'Settings', icon: 'mdi-cog' },
  { name: 'Camera', icon: 'mdi-camera-outline' },
  { name: 'Guide', icon: 'mdi-crosshairs-gps' },
  { name: 'Mount', icon: 'mdi-telescope' },
  { name: 'EFW', icon: 'mdi-tune-variant' },
  { name: 'AutoFocus', icon: 'mdi-focus-auto' },
  { name: 'Rotator', icon: 'mdi-rotate-3d-variant' },
  { name: 'Storage', icon: 'mdi-sd' },
  { name: 'Info', icon: 'mdi-information-outline' }
]
</script>

<style scoped>
.menu-bar {
  background-color: rgba(33, 33, 33, 0.6); /* dark semi-transparent */
  color: white;
  border-radius: 8px;
  position: absolute;
  top: 0;
  right: 0;
  left: 0;
  z-index: 10;
  pointer-events: auto;
}
</style>
