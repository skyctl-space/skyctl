// Stellarium Web - Copyright (c) 2022 - Stellarium Labs SRL
//
// This program is licensed under the terms of the GNU AGPL v3, or
// alternatively under a commercial licence.
//
// The terms of the AGPL v3 license can be found in the main directory of this
// repository.

<template>
  <toolbar class="get-click"></toolbar>
  <selected-object-info style="position: absolute; top: 48px; left: 0px; width: 380px; max-width: calc(100vw - 12px); margin: 6px" class="get-click"></selected-object-info>
  <progress-bars style="position: absolute; bottom: 64px; right: 12px;"></progress-bars> 
  <bottom-bar style="position:absolute; width: 100%; justify-content: center; bottom: 0; display:flex; margin-bottom: 0px" class="get-click"></bottom-bar> 
  <data-credits-dialog></data-credits-dialog>
</template>

<script setup lang="ts">
import Toolbar from '@/stellarium-components/toolbar.vue'
import BottomBar from '@/stellarium-components/bottom-bar.vue'
import SelectedObjectInfo from '@/stellarium-components/selected-object-info.vue'
import ProgressBars from '@/stellarium-components/progress-bars.vue'

import DataCreditsDialog from '@/stellarium-components/data-credits-dialog.vue'
</script>

<style>
</style>
