// Stellarium Web - Copyright (c) 2022 - Stellarium Labs SRL
//
// This program is licensed under the terms of the GNU AGPL v3, or
// alternatively under a commercial licence.
//
// The terms of the AGPL v3 license can be found in the main directory of this
// repository.

<template>
<v-dialog max-width='600' v-model="$store.state.showViewSettingsDialog">
<v-card v-if="$store.state.showViewSettingsDialog" class="secondary white--text">
  <v-card-title><div class="text-h5">{{ $t('View settings') }}</div></v-card-title>
  <v-card-text>
    <v-checkbox hide-details :label="$t('Milky Way')" v-model="milkyWayOn"></v-checkbox>
    <v-checkbox hide-details :label="$t('DSS')" v-model="dssOn"></v-checkbox>
    <v-checkbox hide-details :label="$t('Meridian Line')" v-model="meridianOn"></v-checkbox>
    <v-checkbox hide-details :label="$t('Ecliptic Line')" v-model="eclipticOn"></v-checkbox>
  </v-card-text>
  <v-card-actions>
    <v-spacer></v-spacer><v-btn class="blue--text darken-1" text @click.native="$store.state.showViewSettingsDialog = false">Close</v-btn>
  </v-card-actions>
</v-card>
</v-dialog>
</template>

<script setup>
import { computed } from 'vue'
import { useStore } from 'vuex'

const store = useStore()

const dssOn = computed({
  get: () => store.state.stel.dss.visible,
  set: (newValue) => {
    store.state.stel.dss.visible = newValue
  }
})

const milkyWayOn = computed({
  get: () => store.state.stel.milkyway.visible,
  set: (newValue) => {
    store.state.stel.milkyway.visible = newValue
  }
})

const meridianOn = computed({
  get: () => store.state.stel.lines.meridian.visible,
  set: (newValue) => {
    store.state.stel.lines.meridian.visible = newValue
  }
})

const eclipticOn = computed({
  get: () => store.state.stel.lines.ecliptic.visible,
  set: (newValue) => {
    store.state.stel.lines.ecliptic.visible = newValue
  }
})
</script>

<style>
.input-group {
  margin: 0px;
}
</style>
