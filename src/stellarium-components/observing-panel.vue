// Stellarium Web - Copyright (c) 2022 - Stellarium Labs SRL
//
// This program is licensed under the terms of the GNU AGPL v3, or
// alternatively under a commercial licence.
//
// The terms of the AGPL v3 license can be found in the main directory of this
// repository.

<template>

<div id="observing-panel-container" :class="{observingpanelhidden: !showObservingPanel}" class="get-click">
  <div class="observing-panel-tabsbtn" v-if="showObservingPanelTabsButtons">
    <v-btn class='tab-bt' v-for="tab in tabs" small :key="tab.tabName" :to="tab.url" active-class="tab-bt-active">{{ $t(tab.tabName) }}</v-btn>
  </div>
  <!-- <div id="observing-panel">
    <router-view style="height: 100%"/>
  </div> -->
</div>

</template>

<script setup>
import { computed } from 'vue'
import { useStore } from 'vuex'

const store = useStore()

const showObservingPanel = computed(() => store.state.showSidePanel)
const showObservingPanelTabsButtons = computed(() => store.state.showObservingPanelTabsButtons)

const tabs = computed(() => {
  const res = []
  return res
})
</script>

<style>
#observing-panel-container {
  position:absolute;
  height: 100%;
  width: 400px;
  right: -400px;
}
#observing-panel {
  height: 100%;
  color: white;
  background-color: #212121;
}
.observing-panel-tabsbtn {
  position:absolute;
  right: 400px;
  transform-origin: bottom right;
  transform: rotate(-90deg);
  width: 100vh;
  top: 12px;
  text-align: right;
}
.observingpanelhidden {
  display: none;
}
.tab-bt {
  opacity: 0.5;
}
.tab-bt-active {
  opacity: 1;
}
</style>
