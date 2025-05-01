// Stellarium Web - Copyright (c) 2022 - Stellarium Labs SRL
//
// This program is licensed under the terms of the GNU AGPL v3, or
// alternatively under a commercial licence.
//
// The terms of the AGPL v3 license can be found in the main directory of this
// repository.

<template>
  <v-card width="500">
    <v-container>
      <v-row justify="space-between" no-gutters class="ma-0" style="padding:10px;">
        <div>
          <v-btn text icon class="up_down_bt" style="margin-left: 16px" @mousedown="incTime('years')"
            @touchstart.prevent="incTime('years')"><v-icon>mdi-menu-up</v-icon></v-btn>
          <v-btn text icon class="up_down_bt" style="margin-left: 21px" @mousedown="incTime('months')"
            @touchstart.prevent="incTime('months')"><v-icon>mdi-menu-up</v-icon></v-btn>
          <v-btn text icon class="up_down_bt" style="margin-left: 8px" @mousedown="incTime('days')"
            @touchstart.prevent="incTime('days')"><v-icon>mdi-menu-up</v-icon></v-btn>
          <h1>{{ date }}</h1>
          <v-btn text icon class="up_down_bt" style="margin-left: 16px" @mousedown="decTime('years')"
            @touchstart.prevent="decTime('years')"><v-icon>mdi-menu-down</v-icon></v-btn>
          <v-btn text icon class="up_down_bt" style="margin-left: 21px" @mousedown="decTime('months')"
            @touchstart.prevent="decTime('months')"><v-icon>mdi-menu-down</v-icon></v-btn>
          <v-btn text icon class="up_down_bt" style="margin-left: 8px" @mousedown="decTime('days')"
            @touchstart.prevent="decTime('days')"><v-icon>mdi-menu-down</v-icon></v-btn>
        </div>
        <div>
          <div>
            <v-tooltip top>
              <template v-slot:activator="{ props }">
                <v-btn text icon @click="resetTime" style="margin-top: 5px"
                  v-bind="props"><v-icon>mdi-history</v-icon></v-btn>
              </template>
              <span>Back to real time</span>
            </v-tooltip>
          </div>
          <div>
            <v-tooltip top>
              <template v-slot:activator="{ props }">
                <v-btn text icon @click="togglePauseTime" style="margin-top: 0px" v-bind="props"><v-icon>{{
                    togglePauseTimeIcon }}</v-icon></v-btn>
              </template>
              <span>Pause/unpause time</span>
            </v-tooltip>
          </div>
        </div>
        <div>
          <v-btn text icon class="up_down_bt" @mousedown="incTime('hours')"
            @touchstart.prevent="incTime('hours')"><v-icon>mdi-menu-up</v-icon></v-btn>
          <v-btn text icon class="up_down_bt ml-1" @mousedown="incTime('minutes')"
            @touchstart.prevent="incTime('minutes')"><v-icon>mdi-menu-up</v-icon></v-btn>
          <v-btn text icon class="up_down_bt ml-1" @mousedown="incTime('seconds')"
            @touchstart.prevent="incTime('seconds')"><v-icon>mdi-menu-up</v-icon></v-btn>
          <h1 class="ml-2">{{ time }}</h1>
          <v-btn text icon class="up_down_bt" @mousedown="decTime('hours')"
            @touchstart.prevent="decTime('hours')"><v-icon>mdi-menu-down</v-icon></v-btn>
          <v-btn text icon class="up_down_bt ml-1" @mousedown="decTime('minutes')"
            @touchstart.prevent="decTime('minutes')"><v-icon>mdi-menu-down</v-icon></v-btn>
          <v-btn text icon class="up_down_bt ml-1" @mousedown="decTime('seconds')"
            @touchstart.prevent="decTime('seconds')"><v-icon>mdi-menu-down</v-icon></v-btn>
        </div>
      </v-row>
    </v-container>
    <div style="padding: 20px">
      <div style="position: absolute">
        <svg height="30" width="360">
          <defs>
            <linearGradient id="grad1" x1="0%" y1="0%" x2="100%" y2="0%">
              <stop v-for="stop in stops" :offset="stop.percent" :style="stop.style" :key="stop.percent" />
            </linearGradient>
          </defs>
          <rect width="100%" height="100%" fill="url(#grad1)" />
        </svg>
      </div>
      <v-slider min="0" max="1439" style="padding: 0px; width: 360px;" v-model="timeMinute" :hint="sliderHint"
        persistent-hint></v-slider>
    </div>
  </v-card>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import Moment from 'moment'
import { useStellariumStore } from '@/stores';

const stellariumStore = useStellariumStore();

type Stop = {
  percent: number;
  style: string;
  sunAlt: number;
  moonAlt: number;
};

const stops = ref<Stop[]>([]);

const stopCacheKey = ref<{ sliderStartTime?: string; location?: string }>({
  sliderStartTime: undefined,
  location: undefined
});

const props = defineProps(['modelValue', 'location'])
const emit = defineEmits(['update:modelValue'])

const localTime = computed({
  get: () => {
    const m = Moment(props.modelValue)
    m.local()
    return m
  },
  set: (newValue) => {
    emit('update:modelValue', newValue.format())
  }
})

const time = computed(() => localTime.value.format('HH:mm:ss'))
const date = computed(() => localTime.value.format('YYYY-MM-DD'))

const sliderStartTime = computed(() => {
  const t = localTime.value.clone()
  if (t.hours() < 12) {
    t.subtract(1, 'days')
  }
  t.hours(12)
  t.minutes(0)
  t.seconds(0)
  t.milliseconds(0)
  return t
})

const timeMinute = computed({
  get: () => {
    const t = localTime.value
    return t.hours() < 12 ? (t.hours() + 12) * 60 + t.minutes() : (t.hours() - 12) * 60 + t.minutes()
  },
  set: (newValue) => {
    const t = Moment(sliderStartTime.value)
    t.add(newValue, 'minutes')
    emit('update:modelValue', t.format())
  }
})

const sliderHint = computed(() => {
  const tm = timeMinute.value
  const stop = stops.value[Math.floor(tm * stops.value.length / 1440)]
  if (!stop) return ''
  if (stop.sunAlt > 0) {
    return 'Daylight'
  }
  if (stop.sunAlt < -16) {
    return stop.moonAlt < 5 ? 'Dark night' : 'Moonlight'
  }
  return tm > 720 ? 'Dawn' : 'Twilight'
})

const isTimePaused = computed(() => stellariumStore.tree.time_speed === 0)
const togglePauseTimeIcon = computed(() => isTimePaused.value ? 'mdi-play' : 'mdi-pause')

let clickTimeout: ReturnType<typeof setTimeout> | undefined;
let nbClickRepeat = 0

function resetTime() {
  const m = Moment()
  m.local()
  emit('update:modelValue', m.format())
}

function togglePauseTime() {
  stellariumStore.stel.core.time_speed = (stellariumStore.tree.time_speed === 0) ? 1 : 0
}

type MomentUnit =
  | 'year' | 'years'
  | 'month' | 'months'
  | 'day' | 'days'
  | 'hour' | 'hours'
  | 'minute' | 'minutes'
  | 'second' | 'seconds';

function incTime(unit: MomentUnit) {
  startIncTime(1, unit)
}

function decTime(unit: MomentUnit) {
  startIncTime(-1, unit)
}

function startIncTime(v: number, unit: MomentUnit) {
  clickTimeout = setTimeout(() => {
    const t = localTime.value.clone()
    t.add(v, unit)
    emit('update:modelValue', t.format())
    nbClickRepeat++
    startIncTime(v, unit)
  }, nbClickRepeat === 0 ? 0 : (nbClickRepeat === 1 ? 500 : (nbClickRepeat < 10 ? 100 : (nbClickRepeat < 100 ? 50 : 20))))
}

function stopIncTime() {
  if (clickTimeout) {
    clearTimeout(clickTimeout)
    clickTimeout = undefined
    nbClickRepeat = 0
  }
}

function timeMinuteRangeToUTC(tm: number) {
  return sliderStartTime.value.toDate().getMJD() + tm * 1 / (24 * 60)
}

function refreshStops() {
  if (stopCacheKey.value.sliderStartTime === sliderStartTime.value.format() && stopCacheKey.value.location === props.location) {
    return
  }
  const res = []
  const nbStop = 49
  const obs = stellariumStore.stel.core.observer.clone()
  const sun = stellariumStore.stel.getObj('NAME Sun')
  const moon = stellariumStore.stel.getObj('NAME Moon')
  for (let i = 0; i <= nbStop; ++i) {
    obs.utc = timeMinuteRangeToUTC(1440 * i / nbStop)
    const sunAlt = stellariumStore.stel.anpm(stellariumStore.stel.c2s(stellariumStore.stel.convertFrame(obs, 'ICRF', 'OBSERVED', sun.getInfo('radec', obs)))[1]) * 180.0 / Math.PI
    const moonAlt = stellariumStore.stel.anpm(stellariumStore.stel.c2s(stellariumStore.stel.convertFrame(obs, 'ICRF', 'OBSERVED', moon.getInfo('radec', obs)))[1]) * 180.0 / Math.PI
    const brightnessForAltitude = (sunAlt: number, moonAlt: number) => {
      const moonBrightness = moonAlt < 0 ? 0 : 2 / 35 * Math.min(20, moonAlt) / 20
      if (sunAlt > 0) return Math.min(10, 1 + sunAlt) + moonBrightness
      if (sunAlt < -16) return moonBrightness
      if (sunAlt < -10) return 1 / 35 * (16 + sunAlt) / 6 + moonBrightness
      return (1 - 1 / 35) * (10 + sunAlt) / 10 + 1 / 35 + moonBrightness
    }
    const brightness = Math.log10(1 + brightnessForAltitude(sunAlt, moonAlt) * 10) / 2
    res.push({
      percent: i / nbStop,
      style: 'stop-color:rgb(64,209,255);stop-opacity:' + brightness,
      sunAlt: sunAlt,
      moonAlt: moonAlt
    })
  }
  obs.destroy()
  stopCacheKey.value.sliderStartTime = sliderStartTime.value.format()
  stopCacheKey.value.location = props.location
  stops.value = res
}

onMounted(() => {
  refreshStops()
  window.addEventListener('mouseup', stopIncTime)
  window.addEventListener('touchend', stopIncTime)
})

watch(sliderStartTime, refreshStops)
watch(() => props.location, refreshStops)
</script>

<style>
.up_down_bt {
  margin-bottom: -10px !important;
  margin-top: -10px !important;
  margin-left: 0px;
  margin-right: 0px !important;
}
</style>
