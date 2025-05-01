// Stellarium Web - Copyright (c) 2022 - Stellarium Labs SRL
//
// This program is licensed under the terms of the GNU AGPL v3, or
// alternatively under a commercial licence.
//
// The terms of the AGPL v3 license can be found in the main directory of this
// repository.

<template>
  <div style="position: absolute; display:flex; align-items: flex-end; padding:10px;">
    <bottom-button label="Constellations"
      :img="btnConstellations" img_alt="Constellations Button"
      :toggled="stellariumStore.tree?.constellations?.lines_visible"
      @clicked="(b: boolean) => { stellariumStore.tree?.constellations && (stellariumStore.core.constellations.lines_visible = b, stellariumStore.core.constellations.labels_visible = b) }">
    </bottom-button>
    <bottom-button label="Constellations Art" 
      :img="btnConstellationsArt" img_alt="Constellations Art Button"
      :toggled="stellariumStore.tree?.constellations?.images_visible"
      @clicked="(b: boolean) => { stellariumStore.tree?.constellations && (stellariumStore.core.constellations.images_visible = b) }">
    </bottom-button>
    <bottom-button label="Atmosphere" :img="btnAtmosphere"
      img_alt="Atmosphere Button" :toggled="stellariumStore.tree?.atmosphere?.visible"
      @clicked="(b: boolean) => { stellariumStore.tree?.atmosphere && (stellariumStore.core.atmosphere.visible = b) }">
    </bottom-button>
    <bottom-button label="Landscape" :img="btnLandscape"
      img_alt="Landscape Button" :toggled="stellariumStore.tree?.landscapes?.visible"
      @clicked="(b: boolean) => { stellariumStore.tree?.landscapes && (stellariumStore.core.landscapes.visible = b) }">
    </bottom-button>
    <bottom-button label="Azimuthal Grid" :img="btnAzimuthalGrid"
      img_alt="Azimuthal Button" :toggled="stellariumStore.tree?.lines?.azimuthal?.visible"
      @clicked="(b: boolean) => { stellariumStore.tree?.lines?.azimuthal && (stellariumStore.core.lines.azimuthal.visible = b) }">
    </bottom-button>
    <bottom-button label="Equatorial Grid" 
      :img="btnEquatorialGrid" img_alt="Equatorial Grid Button"
      :toggled="stellariumStore.tree?.lines?.equatorial_jnow?.visible"
      @clicked="(b: boolean) => { stellariumStore.tree?.lines?.equatorial_jnow && (stellariumStore.core.lines.equatorial_jnow.visible = b) }">
    </bottom-button>
    <bottom-button label="Meridian" :img="btnEquatorialGrid" img_alt="Meridian Button" class="mr-auto"
      :toggled="stellariumStore.tree?.lines?.meridian?.visible"
      @clicked="(b: boolean) => { stellariumStore.tree?.lines?.meridian && (stellariumStore.core.lines.meridian.visible = b) }">
    </bottom-button>
    <bottom-button label="Eliptic" :img="btnEquatorialGrid" img_alt="Eliptic Button" class="mr-auto"
      :toggled="stellariumStore.tree?.lines?.ecliptic?.visible"
      @clicked="(b: boolean) => { stellariumStore.tree?.lines?.ecliptic && (stellariumStore.core.lines.ecliptic.visible = b) }">
    </bottom-button>
    <bottom-button label="Deep Sky Objects" :img="btnNebulae" img_alt="Deep Sky Objects Button" class="mr-auto"
      :toggled="stellariumStore.tree?.dsos?.visible"
      @clicked="(b: boolean) => { stellariumStore.core?.dsos && (stellariumStore.core.dsos.visible = b) }">
    </bottom-button>

    <v-spacer></v-spacer>

    <div class="subheader grey--text hidden-sm-and-down pr-2" style="user-select: none;">{{ stellariumStore.currentLatitude }}</div>
    <div class="subheader grey--text hidden-sm-and-down pr-2" style="user-select: none;">{{ stellariumStore.currentLongitude }}</div>

    <v-menu :close-on-content-click="false" transition="fade-transition"
      offset-y top left>
      <template v-slot:activator="{ props }">
        <v-btn size="x-large" class="tmenubt"  v-bind="props">
          <v-icon class="hidden-sm-and-up">mdi-clock-outline</v-icon>
          <span class="hidden-xs-only">
            <div class="text-subtitle-2">{{ time }}</div>
            <div class="text-caption">{{ date }}</div>
          </span>
        </v-btn>
      </template>
      <date-time-picker v-model="pickerDate" :location="stellariumStore.currentLocation"></date-time-picker>
    </v-menu>

  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useStellariumStore } from '@/stores';
import BottomButton from '@/stellarium-components/bottom-button.vue';
import DateTimePicker from '@/stellarium-components/date-time-picker.vue';
import Moment from 'moment';
import btnConstellations from '@/assets/images/btn-cst-lines.svg';
import btnConstellationsArt from '@/assets/images/btn-cst-art.svg';
import btnAtmosphere from '@/assets/images/btn-atmosphere.svg';
import btnLandscape from '@/assets/images/btn-landscape.svg';
import btnAzimuthalGrid from '@/assets/images/btn-azimuthal-grid.svg';
import btnEquatorialGrid from '@/assets/images/btn-equatorial-grid.svg';
import btnNebulae from '@/assets/images/btn-nebulae.svg';
import '@/date-extensions.js';

const stellariumStore = useStellariumStore();

const getLocalTime = () => {
  var d = new Date();
  d.setMJD(stellariumStore.tree?.observer.utc);
  const m = Moment(d);
  m.local();
  return m;
};

const time = computed(() => getLocalTime().format('HH:mm:ss'));
const date = computed(() => getLocalTime().format('YYYY-MM-DD'));

const pickerDate = computed({
  get() {
    const t = getLocalTime();
    t.milliseconds(0);
    return t.format();
  },
  set(v) {
    const m = Moment(v);
    m.local();
    m.milliseconds(getLocalTime().milliseconds());
    stellariumStore.stel.observer.utc = m.toDate().getMJD();
  }
});

</script>

<style>
@media all and (max-width: 600px) {
  .tmenubt {
    min-width: 30px;
  }
}

@media all and (min-width: 600px) {
  .tbtcontainer {
    width: 300px;
  }
}
</style>
