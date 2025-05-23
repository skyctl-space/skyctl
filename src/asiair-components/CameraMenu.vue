<template>
  <v-card class="pa-2">
    <v-row dense align="center" class="mb-2" no-gutters>
      <v-col cols="auto" class="pr-1">
        <v-select
          block
          :disabled ="isCameraOpened"
          v-model="selectedCamera"
          :items="cameraList"
          item-title="name"
          item-value="name"
          :item-disabled="isCameraDisabled"
          placeholder="No camera selected"
          density="compact"
          hide-details
          style="min-width: 250px;"
        />
      </v-col>
      <v-col cols="auto">
        <v-switch
          :model-value="isCameraOpened"
          @update:model-value="val => handleMainCameraToggle(val)"
          inset
          color="indigo"
          density="compact"
          hide-details
        />
      </v-col>
    </v-row>
    <!-- <v-row dense class="mb-2" no-gutters>
      <v-col>
        <v-slider v-model="gain" label="Gain" :min="-25" :max="300" :step="1" density="compact" thumb-label thicks hide-details/>
      </v-col>
    </v-row>
    <v-row dense align="center" class="mb-2" no-gutters>
      <v-col cols="auto" class="pr-1">
        <v-switch v-model="coolerOn" label="Cooling" density="compact" color="indigo" hide-details />
        <v-slider v-model="targetTemp" label="Target (°C)" :min="-10" :max="20" :step="1" density="compact" thumb-label thicks hide-details  />
      </v-col>
    </v-row>
    <v-row dense no-gutters>
      <v-col>
        <v-switch v-model="antiDewHeater" label="Anti-dew heater" color="indigo" density="compact" hide-details />
      </v-col>
    </v-row> -->
  </v-card>
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

const { availableCameras, mainCameraName, mainCameraState, guideCameraState, main_camera_open, main_camera_close } = useASIAirController(props.guid, undefined);

const cameraList = ref<{ name: string; selectable: boolean }[]>([]);
const selectedCamera = ref<string | null>(mainCameraName.value);

function isCameraDisabled(item: { name: string; selectable: boolean }): boolean {
  return !item.selectable;
}

function handleMainCameraToggle(val: boolean | null) {
  if ((val == null) || selectedCamera.value == null) {
    return;
  }

  if (val) {
    main_camera_open(selectedCamera.value);
  } else {
    main_camera_close();
  }
}

const isCameraOpened = ref(false);

watch(
  mainCameraState,
  (state) => {
    console.log('Main camera state changed:', state);
    if (state && state.state !== 'close') {
      isCameraOpened.value = true;
    } else {
      isCameraOpened.value = false;
    }
  },
  { immediate: true }
);

watch(
  [availableCameras, guideCameraState],
  ([cameras, guideState]) => {
    if (Array.isArray(cameras) && cameras.length > 0) {
      let guideName = null;
      let guideActive = false;
      if (guideState && guideState.state !== 'close') {
        guideName = guideState.name;
        guideActive = true;
      }
      cameraList.value = cameras.map(cam => ({
        name: cam.name,
        selectable: !(guideActive && cam.name === guideName)
      }));
    }
  },
  { immediate: true }
);

</script>