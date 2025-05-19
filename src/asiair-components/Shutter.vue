<template>
  <div class="shutter-wrapper">
    <svg viewBox="0 0 50 50" class="shutter-svg" @click="handleClick">
      <!-- Outer idle ring -->
      <circle class="idle-ring" cx="25" cy="25" r="22" stroke-width="2" fill="none" />

      <!-- Rotated progress ring -->
      <g transform="rotate(-90 25 25)">
        <circle class="progress-ring" cx="25" cy="25" r="22" stroke-width="3" fill="none" :style="progressStyle" />
      </g>

      <!-- Inner button: circle when idle, square when active -->
      <circle v-if="!isActive" class="shutter-button" cx="25" cy="25" r="17" fill="currentColor" />
      <rect v-else class="stop-button" x="17" y="17" width="16" height="16" rx="2" ry="2" fill="white" />
    </svg>

    <!-- Countdown text below the SVG -->
    <div class="countdown-text" v-if="isActive">
      {{ remainingSeconds }}s
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useASIAirController } from '@/asiair-components/useASIAirController';

const emit = defineEmits(['shutter-start', 'shutter-end']);

const props = defineProps({
  guid: {
    type: String,
    required: true,
  },
  exposureTime: {
    type: Number,
    required: true,
  },
});

const { imageHeight, imageWidth, imageStats, imageData, cameraInfo, cameraError, cameraBusy } = useASIAirController(props.guid);

const duration = computed(() => {
  return props.exposureTime * 1000; // Convert seconds to milliseconds
});

const isActive = ref(false);
const progress = ref(0);
const remainingSeconds = ref(0);

let timeoutId: ReturnType<typeof setTimeout> | null = null;
let intervalId: ReturnType<typeof setInterval> | null = null;

const progressStyle = computed(() => {
  const radius = 22; // Match the 'r' attribute of the circle
  const circumference = 2 * Math.PI * radius;
  return {
    strokeDasharray: `${circumference}px`,
    strokeDashoffset: `${circumference * (1 - progress.value)}px`,
    transition: isActive.value ? `stroke-dashoffset ${duration.value}ms linear` : '',
  };
});

function startShutter() {
  if (isActive.value) return;

  isActive.value = true;
  progress.value = 0;
  remainingSeconds.value = Math.ceil(duration.value / 1000);

  emit('shutter-start');

  requestAnimationFrame(() => {
    progress.value = 1;
  });

  intervalId = setInterval(() => {
    remainingSeconds.value--;
    if (remainingSeconds.value <= 0) {
      clearInterval(intervalId!);
    }
  }, 1000);

  timeoutId = setTimeout(() => {
    endShutter();
  }, duration.value);
}

import { invoke, Channel } from '@tauri-apps/api/core';

type ImageProgress =
  | { event: 'fetching' }
  | { event: 'downsampling' }
  | { event: 'debayering' }
  | {
    event: 'rendering';
    data: {
      width: number;
      height: number;
      stats: any[]; // or define Stat type
    };
  }
  | { event: 'error'; data: string };

let imgNotificationChannel: Channel<ImageProgress> | null = null;
let imgDataChannel: Channel<unknown> | null = null;

function normalizeToUint8Array(data: unknown): Uint8Array {
  if (data instanceof Uint8Array) return data;
  if (Array.isArray(data)) return new Uint8Array(data);
  throw new Error("Expected Uint8Array or Array<number> from backend");
  }

async function endShutter() {
  if (!isActive.value) return;

  isActive.value = false;
  progress.value = 0;
  clearInterval(intervalId!);
  clearTimeout(timeoutId!);
  emit('shutter-end');

  console.log('Shutter ended');

  cameraBusy.value = true;
  cameraError.value = null;
  cameraInfo.value = null;

  imgNotificationChannel = new Channel<ImageProgress>();
  imgNotificationChannel.onmessage = (message) => {
    switch (message.event) {
      case 'fetching':
        console.log(`[${props.guid}] Fetching image...`);
        cameraInfo.value = "Fetching image...";
        break;
      case 'downsampling':
        console.log(`[${props.guid}] Downsampling image...`);
        cameraInfo.value = "Downsampling...";
        break;
      case 'debayering':
        console.log(`[${props.guid}] Debayering image...`);
        cameraInfo.value = "Debayering...";
        break;
      case 'rendering': {
        const { width, height, stats } = message.data;
        console.log(`[${props.guid}] Receiving image info: ${width}x${height}`);
        imageWidth.value = width;
        imageHeight.value = height;
        console.log(`[${props.guid}] Image stats:`, stats);
        imageStats.value = stats;
        cameraInfo.value = "Rendering...";
        break;
      }
      case 'error':
        console.error(`[${props.guid}] Error:`, message.data);
        cameraError.value = message.data;
        break;
      default:
        console.warn(`[${props.guid}] Unknown event:`, message);
        cameraError.value = "Unknown event";
    }
  };



  imgDataChannel = new Channel<unknown>();
  imgDataChannel.onmessage = (raw) => {
    const u8 = normalizeToUint8Array(raw);
    const buffer = u8.buffer.slice(u8.byteOffset, u8.byteOffset + u8.byteLength);

    const result = new Uint16Array(buffer.byteLength / 2);
    const view = new DataView(buffer);
    for (let i = 0; i < result.length; i++) {
      result[i] = view.getUint16(i * 2, true); // little-endian
    }

    imageData.value = result;
    cameraInfo.value = null;
    cameraBusy.value = false;
  };

  await invoke('get_current_img', {
    guid: props.guid,
    sender: imgNotificationChannel,
    binarySender: imgDataChannel,
  }).catch((e) => {
    console.error(`[${props.guid}] Capture failed:`, e);
  });
}

function handleClick() {
  if (isActive.value) {
    endShutter(); // Stop early
  } else {
    startShutter(); // Start
  }
}
</script>

<style scoped>
.shutter-wrapper {
  width: 100px;
  height: 100px;
  cursor: pointer;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding-top: 10px
}

.shutter-svg {
  width: 70px;
  height: 70px;
}

.idle-ring {
  stroke: white;
  opacity: 0.4;
}

.progress-ring {
  stroke: white;
  stroke-linecap: round;
  fill: none;
  stroke-dasharray: 138.2;
  stroke-dashoffset: 138.2;
}

.shutter-button {
  color: var(--v-theme-primary);
}

.stop-button {
  stroke: none;
  fill: white;
}

.countdown-text {
  margin-top: 4px;
  color: white;
  font-size: 12px;
  font-family: monospace;
  font-weight: bold;
}
</style>
