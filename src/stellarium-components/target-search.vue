// Stellarium Web - Copyright (c) 2022 - Stellarium Labs SRL
//
// This program is licensed under the terms of the GNU AGPL v3, or
// alternatively under a commercial licence.
//
// The terms of the AGPL v3 license can be found in the main directory of this
// repository.

<template>
  <div class="tsearch">
    <skysource-search v-model="obsSkySource" :floatingList="true"></skysource-search>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import SkysourceSearch from '@/stellarium-components/skysource-search.vue'
import { useStellariumStore } from '@/stores';
import { skySource2SweObj, setSweObjAsSelection } from '@/utils';

const obsSkySource = ref(undefined)
const stellariumStore = useStellariumStore();

watch(obsSkySource, (ss: any) => {
  if (!ss) {
    return
  }
  let obj = skySource2SweObj(stellariumStore.stel, ss)
  if (!obj) {
    obj = stellariumStore.stel.createObj(ss.model, ss)
    stellariumStore.selectionLayer.add(obj)
  }
  if (!obj) {
    console.info("Can't find object in SWE: " + ss.names[0])
    return
  }
  setSweObjAsSelection(stellariumStore.stel, obj)
})
</script>

<style>
@media all and (min-width: 600px) {
  .tsearch {
    z-index: 2;
    min-width: 400px;
  }
}
</style>
