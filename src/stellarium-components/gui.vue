// Stellarium Web - Copyright (c) 2022 - Stellarium Labs SRL
//
// This program is licensed under the terms of the GNU AGPL v3, or
// alternatively under a commercial licence.
//
// The terms of the AGPL v3 license can be found in the main directory of this
// repository.

<template>

<!-- <div class="click-through" style="position:relative; width: 100%; height: 100%; display:flex; align-items: flex-end;"> -->
  <toolbar class="get-click"></toolbar>
  <!-- <observing-panel></observing-panel>
  <template v-for="(item, i) in dialogs" :key="i">
    <component :is="item" ></component>
  </template>
  <selected-object-info style="position: absolute; top: 48px; left: 0px; width: 380px; max-width: calc(100vw - 12px); margin: 6px" class="get-click"></selected-object-info>
  -->
  <!-- <progress-bars style="position: absolute; bottom: 54px; right: 12px;"></progress-bars>  -->
  <bottom-bar style="position:absolute; width: 100%; justify-content: center; bottom: 0; display:flex; margin-bottom: 0px" class="get-click"></bottom-bar> 
<!-- </div> -->

</template>

<script setup lang="ts">
import { computed } from 'vue'


import Toolbar from '@/stellarium-components/toolbar.vue'
import BottomBar from '@/stellarium-components/bottom-bar.vue'
// import SelectedObjectInfo from '@/stellarium-components/selected-object-info.vue'
// import ProgressBars from '@/stellarium-components/progress-bars.vue'

// import DataCreditsDialog from '@/stellarium-components/data-credits-dialog.vue'
// import ViewSettingsDialog from '@/stellarium-components/view-settings-dialog.vue'
// import PlanetsVisibility from '@/stellarium-components/planets-visibility.vue'
// // import LocationDialog from '@/stellarium-components/location-dialog.vue'
// import ObservingPanel from '@/stellarium-components/observing-panel.vue'


const dialogs = computed(() => {
  let res = [
    'data-credits-dialog',
    'view-settings-dialog',
    'planets-visibility',
    'location-dialog'
  ]
  return res
})
</script>

<style>
</style>
