<template>
  <div class="shutter-wrapper" :class="{ disabled: props.disabled }" @click="handleClick">
    <svg viewBox="0 0 50 50" class="shutter-svg">
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

const props = defineProps({
  guid: {
    type: String,
    required: true,
  },
  disabled: {
    type: Boolean,
    default: false,
  },
});

const { trigger_capture, mainCamera } = useASIAirController(props.guid, undefined);

const exposureTime = computed(() => {
  return mainCamera.value.exposure_us;
});

const duration = computed(() => {
  return exposureTime.value / 1000; // Convert to milliseconds
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
  
  trigger_capture();

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

async function endShutter() {
  if (!isActive.value) return;

  isActive.value = false;
  progress.value = 0;
  clearInterval(intervalId!);
  clearTimeout(timeoutId!);

  console.log('Shutter ended');
}

function handleClick() {
  if (props.disabled) return;
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
  padding-top: 10px;
}

.shutter-wrapper.disabled {
  pointer-events: none;
  opacity: 0.5;
  cursor: not-allowed;
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
