// Stellarium Web - Copyright (c) 2022 - Stellarium Labs SRL
//
// This program is licensed under the terms of the GNU AGPL v3, or
// alternatively under a commercial licence.
//
// The terms of the AGPL v3 license can be found in the main directory of this
// repository.

<template>
  <v-toolbar style="position: absolute; top: 0; left: 0; width: 100%; z-index: 10;" color="transparent"
    image="/src/assets/images/header.png" elevation="2">
    <a href="https://stellarium-web.org" target="_blank" rel="noopener">

      <img class="tbtitle hidden-xs-only" id="stellarium-web-toolbar-logo" src="@/assets/images/logo.svg" width="30"
        height="30" alt="Stellarium Web Logo" />
    </a>
    <div class="tbtitle hidden-sm-and-down" style="text-align: center;">
      <div style="font-size: 8px;text-align:left">Powered by</div>
      Stellarium<sup>Web</sup>
    </div>
    <v-btn icon="mdi-information-outline" @click="stellariumStore.showCreditsDialog = true" </v-btn>
      <v-spacer></v-spacer>
      <target-search></target-search>
      <v-spacer></v-spacer>
      <div class="subheader grey--text hidden-sm-and-down pr-2" style="user-select: none;">FPS {{ stellariumStore.stel ?
        stellariumStore.tree.fps.toFixed(1) : '?' }}</div>
      <div class="subheader grey--text hidden-sm-and-down" style="user-select: none;">FOV {{ fov }}</div>
  </v-toolbar>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useStellariumStore } from '@/stores';

import TargetSearch from '@/stellarium-components/target-search.vue'

const stellariumStore = useStellariumStore();

const fov = computed(() => {
  if (!stellariumStore.stel) return '-'
  const fovValue = stellariumStore.tree.fov * 180 / Math.PI
  return fovValue.toPrecision(3) + '°'
})
</script>

<style>
#stellarium-web-toolbar-logo {
  margin-right: 10px;
  margin-left: 30px;
}

.tbtitle {
  font-size: 20px;
  font-weight: 500;
  user-select: none;
}
</style>
