<script setup lang="ts">
import { onMounted, ref } from 'vue'
//import swh from '@/assets/sw_helpers.js'
import StelWebEngine from '@/assets/js/stellarium-web-engine.js'

const stellarium = ref(null)

onMounted(() => {
//    const wasmFile = await fetch('/src/assets/js/stellarium-web-engine.wasm')
//    const wasmBinary = await wasmFile.arrayBuffer()
//   const module = await WebAssembly.instantiate(wasmBinary)
//   console.log('WASM Module Loaded:', module)
  import('@/assets/js/stellarium-web-engine.wasm').then(f => {
    StelWebEngine({
        wasmFile: f.default,
        canvas: document.getElementById('stel-canvas'),
        translateFn: function (_domain, str) {
            return str
        },
        onReady: function (stel) {
            stellarium.value = stel;

            const core = stel.core
            core.stars.addDataSource({ url: '/skydata/stars' })
            core.skycultures.addDataSource({ url: '/skydata/skycultures/western', key: 'western' })
            core.dsos.addDataSource({ url: '/skydata/dso' })
            core.landscapes.addDataSource({ url: '/skydata/landscapes/guereins', key: 'guereins' })
            core.milkyway.addDataSource({ url: '/skydata/surveys/milkyway' })
            core.minor_planets.addDataSource({ url: '/skydata/mpcorb.dat', key: 'mpc_asteroids' })
            core.planets.addDataSource({ url: '/skydata/surveys/sso/moon', key: 'moon' })
            core.planets.addDataSource({ url: '/skydata/surveys/sso/sun', key: 'sun' })
            core.planets.addDataSource({ url: '/skydata/surveys/sso/moon', key: 'default' })
            core.comets.addDataSource({ url: '/skydata/CometEls.txt', key: 'mpc_comets' })
            core.satellites.addDataSource({ url: '/skydata/tle_satellite.jsonl.gz', key: 'jsonl/sat' })

            // // Force ui update when there is any change.
            // stel.change(function (_obj, attr) {
            //     if (attr !== "hovered")
            //         stellarium.value = Object.assign(Object.create(stel), {}, stel)
            // })

            stel.setSkyCulture('western')
            stel.setFont('regular', '/fonts/Roboto-Regular.ttf', 1.38);
            stel.setFont('bold', '/fonts/Roboto-Bold.ttf', 1.38);
        }
    })
  })
})
</script>

<template>
    <v-container class="fill-height" fluid style="padding: 0">
        <div id="stel">
            <div style="position: relative; width: 100%; height: 100%">
                <canvas id="stel-canvas" ref="stelCanvas"></canvas>
            </div>
        </div>
    </v-container>
</template>

<style>
a {
    color: #82b1ff;
}

a:link {
    text-decoration-line: none;
}

.divider_menu {
    margin-top: 8px;
    margin-bottom: 8px;
}

html {
    overflow-y: visible;
}

html,
body,
#app {
    overflow-y: visible !important;
    overflow-x: visible;
    position: fixed !important;
    width: 100%;
    height: 100%;
    padding: 0 !important;
    font-size: 14px;
}

.fullscreen {
    overflow-y: hidden;
    position: fixed;
    width: 100%;
    height: 100%;
    padding: 0 !important;
}

.click-through {
    pointer-events: none;
}

.get-click {
    pointer-events: all;
}

.dialog {
    background: transparent;
}

.menu__content {
    background-color: transparent !important;
}

#stel {
    height: 100%;
    width: 100%;
    position: absolute;
}

#stel-canvas {
    z-index: -10;
    width: 100%;
    height: 100%;
}

.right_panel {
    padding-right: 400px;
}

.v-btn {
    margin-left: 8px;
    margin-right: 8px;
    margin-top: 6px;
    margin-bottom: 6px;
}

.v-application--wrap {
    min-height: 100% !important;
}
</style>
