<script setup lang="ts">
import { useTheme } from 'vuetify'
import md5 from 'crypto-js/md5'
import { computed, inject, ref, onMounted, onUnmounted } from 'vue'
//import { invoke } from "@tauri-apps/api/core";
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

const showMapModal = ref(false) // State to control the modal visibility

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

// Format latitude and longitude
// const formattedGeoLocation = computed(() => {
//   const latitude = formatLatitude(geo_latitude.value);
//   const longitude = formatLongitude(geo_longitude.value);

//   return `Lat: ${latitude}, Lon: ${longitude}`
// })


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
      <span style="color: white; margin-right: 16px;">{{ currentTime }}</span>

      <!-- <v-progress-circular color="dark-blue" indeterminate v-if="geo_isLoading"></v-progress-circular>
      <v-chip variant="text" color="red" class="translucent-chip" v-if="geo_error">
        {{ geo_error }}
      </v-chip>
      <v-chip variant="text" color="white" class="translucent-chip" v-else @click="showMapModal = true">
        <v-icon icon="mdi-map-marker" size="16" class="mr-2"></v-icon>
        <span style="color: white;">{{ formattedGeoLocation }}</span>
      </v-chip> -->

      <v-btn icon="mdi-theme-light-dark" @click="toggleTheme()"></v-btn>
    </v-app-bar>

    <v-navigation-drawer expand-on-hover rail>
      <v-list>
        <v-list-item :prepend-avatar="gravatarUrl" :subtitle="email" :title="username"></v-list-item>
      </v-list>

      <v-divider></v-divider>

      <v-list density="compact" nav>
        <v-list-item prepend-icon="mdi-telescope" title="Telescopes" value="telescopes" to="/telescopes"></v-list-item>
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

    <!-- Modal for Google Maps -->
    <v-dialog v-model="showMapModal" max-width="800px">
      <v-card>
        <v-card-title>
          Location on Google Maps
          <v-spacer></v-spacer>
          <v-btn icon="mdi-close" @click="showMapModal = false"></v-btn>
        </v-card-title>
        <v-card-text>
          <iframe :src="`https://www.google.com/maps?q=${geoLocation.latitude},${geoLocation.longitude}&z=15&output=embed`" width="100%"
            height="400" style="border:0;" allowfullscreen=false loading="lazy"></iframe>
        </v-card-text>
      </v-card>
    </v-dialog>
  </v-app>
</template>

<style>
.translucent-chip {
  background-color: rgba(0, 0, 0, 0.4);
  /* White with 70% opacity */
  color: white;
}
</style>