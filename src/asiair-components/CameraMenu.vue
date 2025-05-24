<template>
  <v-card class="pa-2">
    <v-row dense align="center" class="mb-2" no-gutters>
      <v-col cols="auto" class="pr-1">
        <v-select
          block
          :disabled="isCameraOpened"
          v-model="selectedCamera"
          :items="cameraList"
          item-title="name"
          item-value="id"
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

const cameraList = ref<{ name: string; selectable: boolean; id: number }[]>([]);
const selectedCamera = ref<number | null>(null);
const selectedCameraId = ref<number>(0);

function isCameraDisabled(item: { name: string; selectable: boolean }): boolean {
  return !item.selectable;
}

function handleMainCameraToggle(val: boolean | null) {
  if ((val == null) || selectedCamera.value == null) {
    return;
  }

  console.log('Main camera toggle:', val, selectedCamera.value);
  if (val) {
    main_camera_open(selectedCameraId.value);
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
        selectable: !(guideActive && cam.name === guideName),
        id: cam.id
      }));
    }
  },
  { immediate: true }
);

watch(
  mainCameraName,
  (name) => {
    // Set selectedCamera to the id of the camera with the matching name
    const found = cameraList.value.find(cam => cam.name === name);
    selectedCamera.value = found ? found.id : null;
    selectedCameraId.value = selectedCamera.value ?? 0;
  },
  { immediate: true }
);

watch(
  selectedCamera,
  (id) => {
    selectedCameraId.value = id ?? 0;
  }
);

</script>