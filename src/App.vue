<script setup lang="ts">
import { useTheme } from 'vuetify'
import md5 from 'crypto-js/md5'
import { computed, inject, ref, onMounted, onUnmounted, provide } from 'vue'
//import { invoke } from "@tauri-apps/api/core";
import { settings, saveSettings } from "./store";
import { listen } from '@tauri-apps/api/event';
import { GeoLocation } from "./types";

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

const theme = useTheme()

function toggleTheme() {
  theme.global.name.value = theme.global.current.value.dark ? 'light' : 'dark'
}

const username = "Diego Dompe";
const email = "ddompe@gmail.com";
const gravatarUrl = computed(() => {
  return `https://www.gravatar.com/avatar/${md5(email.trim().toLowerCase())}`
})

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
    if (value === undefined) {
      settings.selectedSiteIdx = undefined;
    } else {
      settings.selectedSiteIdx = value.value;
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
</script>

<template>
  <v-app>
    <v-app-bar image="carina_jwst.jpg" elevation="1" rounded density="compact">
      <v-app-bar-title>
        <router-link to="/">
          <p class="font-weight-bold" style="color: white;">SkyCtl</p>
        </router-link>
      </v-app-bar-title>
      <v-spacer></v-spacer>

      <v-chip color="red" variant="flat" v-if="settings.sites.length === 0">No Observing Sites configured</v-chip>
      <v-combobox hide-details 
        style="color:white; background-color: rgba(128, 0, 0, 0.5); border-radius: 6px; " 
        class="my-auto" v-else density="compact" variant="underlined" :items="siteOptions" v-model="selectedSite"
        label="Current Observing site" />
      <v-spacer/>

      <span style="color: white; margin-right: 16px;">{{ currentTime }}</span>

      <v-btn icon="mdi-theme-light-dark" @click="toggleTheme()"></v-btn>
    </v-app-bar>

    <v-navigation-drawer expand-on-hover rail>
      <v-list>
        <v-list-item :prepend-avatar="gravatarUrl" :subtitle="email" :title="username"></v-list-item>
      </v-list>

      <v-divider></v-divider>

      <v-list density="compact" nav>
        <v-list-item prepend-icon="mdi-telescope" title="Telescopes" value="telescopes" to="/telescopes">
          </v-list-item>
        <v-list-item prepend-icon="mdi-weather-night" title="Stellarium" value="stellarium"
          to="/stellarium"></v-list-item>
        <v-list-item prepend-icon="mdi-list-status" title="Objectives" value="objectives"
          to="/objectives"></v-list-item>
        <v-list-item prepend-icon="mdi-image" title="Image Management" value="images" to="/images"></v-list-item>
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
</style>