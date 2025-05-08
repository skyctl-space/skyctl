<script setup lang="ts">
import { computed, inject, ref, onMounted, onUnmounted, provide } from 'vue'
import { settings, saveSettings } from "./settings";
import { useStellariumStore } from './stores';
import { listen } from '@tauri-apps/api/event';
import { GeoLocation } from "./types";
import { formatLatitude, formatLongitude } from './utils';

const geoLocation = inject<GeoLocation>('geoLocation', {
  latitude: 0,
  longitude: 0,
  valid: false,
  error: null
});

type LocationPayload = {
  latitude: number;
  longitude: number;
};

// We only get these events on MacOS from the backend
listen<LocationPayload>('location_update', (event) => {
  const { latitude, longitude } = event.payload;
  console.log('New location:', latitude, longitude);

  geoLocation.latitude = latitude;
  geoLocation.longitude = longitude;
  geoLocation.valid = true;
  geoLocation.error = null;
});

const currentTime = ref(new Date().toLocaleString("en-US", {
  hour12: false,
  timeZoneName: "short"
}));

const siteOptions = computed(() =>
  settings.sites.map((site, index) => ({
    title: site.name,
    value: index
  }))
)

const stellariumStore = useStellariumStore();

const selectedSite = computed({
  get() {
    if (settings.selectedSiteIdx === undefined) {
      return undefined;
    }

    return {
      title: settings.sites[settings.selectedSiteIdx].name,
      value: settings.selectedSiteIdx
    }
  },
  set(value) {
    console.log("Selected site index:", value);

    const stel = stellariumStore.stel;
    if (value === undefined) {
      settings.selectedSiteIdx = undefined;
      stel.observer.longitude = 0;
      stel.observer.latitude = 0;
      stel.observer.elevation = 0;
      stellariumStore.currentLatitude = formatLatitude(0);
      stellariumStore.currentLongitude = formatLongitude(0);
      stellariumStore.currentLocation = "Default Location";
    } else {
      settings.selectedSiteIdx = value.value;
      if (stellariumStore.stel != undefined) {
        const site = settings.sites[settings.selectedSiteIdx];
        stel.observer.longitude = site.longitude * stel.D2R;
        stel.observer.latitude = site.latitude * stel.D2R;
        stel.observer.elevation = site.elevation;
        stellariumStore.currentLocation = site.name;
        stellariumStore.currentLatitude = formatLatitude(site.latitude);
        stellariumStore.currentLongitude = formatLongitude(site.longitude);
      }
    }
    saveSettings();
  }
});

const selectedSiteInfo = computed(() => {
  if (settings.selectedSiteIdx === undefined) {
    return undefined;
  }
  return settings.sites[settings.selectedSiteIdx];
});

provide('selectedSite', selectedSiteInfo);

onMounted(() => {
  const interval = setInterval(() => {
    currentTime.value = new Date().toLocaleString("en-US", {
      hour12: false,
      timeZoneName: "short"
    });
  }, 1000);

  if (navigator.userAgent.includes('Macintosh')) {
    // Updates will arrive asynchronously from the backend
  } else if (navigator.geolocation) {
    navigator.geolocation.getCurrentPosition(
      (position) => {
        geoLocation.latitude = position.coords.latitude;
        geoLocation.longitude = position.coords.longitude;
        geoLocation.valid = true;
        geoLocation.error = null;
      },
      (nav_error) => {
        geoLocation.valid = false;
        geoLocation.error = nav_error.message;
      }
    );
  } else {
    geoLocation.valid = false;
    geoLocation.error = "Geolocation is not supported.";
  }

  onUnmounted(() => {
    clearInterval(interval);
  });
});



const nightMode = ref(false);

function toggleNightMode() {
  nightMode.value = !nightMode.value;

  const nightmodeEl = document.getElementById('nightmode');
  if (nightmodeEl) {
    if (window.navigator.userAgent.indexOf('Edge') > -1) {
      nightmodeEl.style.opacity = nightMode.value ? '0.5' : '0';
    }
    nightmodeEl.style.visibility = nightMode.value ? 'visible' : 'hidden';
  }
}

</script>

<template>
  <div id="nightmode"></div>
  <v-app>
    <v-app-bar image="carina_jwst.jpg" elevation="1" rounded density="compact">
      <v-app-bar-title>
        <v-btn size="x-large" to="/" variant="plain" class="bright-btn">
          <template v-slot:prepend>
            <v-icon size="x-large">
              <v-img src="/SkyCtl128x128.png" max-width="128" max-height="128" />
            </v-icon>
          </template>
          <span class="font-weight-bold">SkyCtl</span>
        </v-btn>

      </v-app-bar-title>
      <v-spacer></v-spacer>

      <v-chip color="red" variant="flat" v-if="settings.sites.length === 0">No Observing Sites configured</v-chip>
      <v-combobox hide-details style="color:white; background-color: rgba(128, 0, 0, 0.5); border-radius: 6px; "
        class="my-auto" v-else density="compact" variant="underlined" :items="siteOptions" v-model="selectedSite"
        label="Current Observing site" />
      <v-spacer />

      <span style="color: white; margin-right: 16px;">{{ currentTime }}</span>

      <v-btn icon="mdi-theme-light-dark" @click="toggleNightMode()"></v-btn>
    </v-app-bar>

    <v-navigation-drawer expand-on-hover rail>
      <v-list density="compact" nav>
        <v-list-item prepend-icon="mdi-telescope" title="Telescopes" value="telescopes" to="/telescopes">
        </v-list-item>
        <v-list-item prepend-icon="mdi-weather-night" title="Stellarium" value="stellarium"
          to="/stellarium"></v-list-item>

         <v-tooltip text="Coming soon!" location="end">
          <template v-slot:activator="{ props }">
            <v-list-item v-bind="props" prepend-icon="mdi-list-status" title="Objectives"
              value="objectives"></v-list-item>
          </template>
        </v-tooltip>

        <v-tooltip text="Coming soon!" location="end">
          <template v-slot:activator="{ props }">
            <v-list-item v-bind="props" prepend-icon="mdi-image" title="Image Management"
              value="images"></v-list-item>
          </template>
        </v-tooltip>

        <v-list-item prepend-icon="mdi-weather-night-partly-cloudy" title="Weather Forecast" value="weather"
          to="/weather"></v-list-item>
        <v-list-item prepend-icon="mdi-cog" title="Settings" value="settings" to="/settings"></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <RouterView />
    </v-main>
  </v-app>
</template>

<style>
.translucent-chip {
  background-color: rgba(0, 0, 0, 0.4);
  /* White with 70% opacity */
  color: white;
}

#nightmode {
  background: transparent;
  pointer-events: none;
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  width: 100%;
  z-index: 10000;
  visibility: hidden;
  backdrop-filter: brightness(0.4) contrast(1.0) sepia(1) hue-rotate(-45deg) saturate(5);
}

.bright-btn {
  opacity: 1;
}
</style>