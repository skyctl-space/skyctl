<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useStellariumStore } from './stores';
import { settings } from './settings';
import { formatLatitude, formatLongitude } from './utils';
import _ from 'lodash';

// Import the ES module factory and WASM URL
import StelWebEngine from '@/assets/js/stellarium-web-engine.js';

import Gui from './stellarium-components/gui.vue';

const canvasRef = ref<HTMLCanvasElement | null>(null);

const stellariumStore = useStellariumStore();

onMounted(async () => {
  if (canvasRef.value) {
    StelWebEngine({
      canvas: canvasRef.value,
      translateFn: (_domain: string, str: string) => str,
      onReady: (stel: any) => {
        stellariumStore.stel = stel;
        stellariumStore.core = stel.core;
        stellariumStore.tree = stel.core.getTree();
        stel.onValueChanged(function (path: any, value: any) {
          const tree = stellariumStore.tree;
          _.set(tree, path, value);
          stellariumStore.tree = tree;
        });

        const baseUrl = '/skydata/';
        const core = stel.core;
        core.stars.addDataSource({ url: baseUrl + 'stars/swe', key : "swe"});
        core.stars.addDataSource({ url: baseUrl + 'stars/minimal', key : "minimal"});
        core.stars.addDataSource({ url: baseUrl + 'stars/base', key : "base"});
        core.stars.addDataSource({ url: baseUrl + 'stars/extended', key : "extended"});
        core.stars.addDataSource({ url: baseUrl + 'surveys/gaia/v1', key : "gaia"});
        core.skycultures.addDataSource({ url: baseUrl + 'skycultures/western', key: 'western' });
        core.dss.addDataSource({ url: baseUrl + 'surveys/dss/colored' });
        core.dsos.addDataSource({ url: baseUrl + 'dso/base' });
        core.dsos.addDataSource({ url: baseUrl + 'dso/extended' });
        core.landscapes.addDataSource({ url: baseUrl + 'landscapes/guereins', key: 'guereins' });
        core.milkyway.addDataSource({ url: baseUrl + 'surveys/milkyway' });
        core.minor_planets.addDataSource({ url: baseUrl + 'mpcorb.dat', key: 'mpc_asteroids' });
        core.comets.addDataSource({ url: baseUrl + 'CometEls.txt', key: 'mpc_comets' });
        core.satellites.addDataSource({ url: baseUrl + 'tle_satellite.jsonl.gz', key: 'jsonl/sat' });
        stel.setFont('regular', '/fonts/Roboto-Regular.ttf', 1.38);
        stel.setFont('bold', '/fonts/Roboto-Bold.ttf', 1.38);
        core.constellations.lines_visible = true;
        core.lines.equatorial_jnow.visible = true;
        core.lines.meridian.visible = false;
        core.lines.ecliptic.visible = false;
        core.atmosphere.visible = true;
        core.landscapes.visible = true;
        core.lines.azimuthal.visible = false;
        core.dsos.visible = true;
        core.constellations.images_visible = false;
        core.time_speed = 1;
        core.observer.utc = new Date().getMJD();

        if (settings.selectedSiteIdx !== undefined) {
          const site = settings.sites[settings.selectedSiteIdx];
          core.observer.latitude = site.latitude * stel.D2R;
          core.observer.longitude = site.longitude * stel.D2R;
          core.observer.elevation = site.elevation;
          stellariumStore.currentLocation = site.name;
          stellariumStore.currentLatitude = formatLatitude(site.latitude);
          stellariumStore.currentLongitude = formatLongitude(site.longitude);
        } else {
          core.observer.latitude = 0;
          core.observer.longitude = 0;
          core.observer.elevation = 0;
          stellariumStore.currentLocation = "Default Location"
          stellariumStore.currentLatitude = formatLatitude(0);
          stellariumStore.currentLongitude = formatLongitude(0);
        }
      }
    });
  }
});
</script>

<template>
    <v-container fluid class="d-flex align-center justify-center" style="position:relative;height:100%;width:100%;padding:0;">
        <Gui />
        <canvas ref="canvasRef"
                style="width:100%;height:100%;max-width:100%;background:black;"></canvas>
    </v-container>
</template>