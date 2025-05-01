// Stellarium Web - Copyright (c) 2022 - Stellarium Labs SRL
//
// This program is licensed under the terms of the GNU AGPL v3, or
// alternatively under a commercial licence.
//
// The terms of the AGPL v3 license can be found in the main directory of this
// repository.

<template>
  <v-card v-if="selectedObject" transparent style="background: rgba(66, 66, 66, 0.3);">
    <v-btn icon style="position: absolute; right: 0" @click="unselect"><v-icon>mdi-close</v-icon></v-btn>
    <v-card-title primary-title>
      <div style="width: 100%">
        <img :src="icon" height="48" width="48" align="left" style="margin-top: 3px; margin-right: 10px"/>
        <div style="overflow: hidden; text-overflow: ellipsis;">
          <div class="text-h5">{{ title }}</div>
          <div class="grey--text text-body-2">{{ type }}</div>
        </div>
      </div>
    </v-card-title>
    <v-card-text style="padding-bottom: 5px;">
      <v-row v-if="otherNames.length > 1" style="width: 100%;">
        <v-col cols="12">
          <span style="position: absolute;">{{ $t('Also known as') }}</span><span style="padding-left: 33.3333%">&nbsp;</span><span class="text-caption white--text" v-for="mname in otherNames1to7" :key="mname" style="margin-right: 15px; font-weight: 500;">{{ mname }}</span>
          <v-btn small icon class="grey--text" v-if="otherNames.length > 8" @click="showMinorNames = !showMinorNames" style="margin-top: -5px; margin-bottom: -5px;"><v-icon>mdi-dots-horizontal</v-icon></v-btn>
          <span class="text-caption white--text" v-for="mname in otherNames8andMore" :key="mname" style="margin-right: 15px; font-weight: 500">{{ mname }}</span>
        </v-col>
      </v-row>
    </v-card-text>
    <v-card-text>
      <template v-for="item in items" :key="item.key">
        <v-row style="width: 100%" no-gutters>
          <v-col cols="4" style="color: #dddddd">{{ item.key }}</v-col>
          <v-col cols="8" style="font-weight: 500" class="white--text"><span v-html="item.value"></span></v-col>
        </v-row>
      </template>
      <div style="margin-top: 15px" class="white--text" v-html="wikipediaSummary"></div>
    </v-card-text>
    <v-card-actions style="margin-top: -25px">
      <v-spacer/>
    </v-card-actions>
    <v-dialog v-model="showShareLinkDialog" width="500px" absolute>
      <v-card style="height: 180px" class="secondary white--text">
        <v-card-title primary-title>
          <div>
            <h3 class="text-h5 mb-0">Share link</h3>
          </div>
        </v-card-title>
        <v-card-text style="width:100%;">
          <v-row style="width: 100%">
            <v-text-field id="link_inputid" v-model="shareLink" label="Link" solo readonly></v-text-field>
            <v-btn @click.stop="copyLink">Copy</v-btn>
          </v-row>
        </v-card-text>
      </v-card>
    </v-dialog>
    <div v-if="$store.state.showSelectedInfoButtons" style="position: absolute; right: 0px; bottom: -50px;">
      <v-btn v-if="!showPointToButton" fab small color="transparent" @click="showShareLinkDialog = !showShareLinkDialog">
        <v-icon>mdi-link</v-icon>
      </v-btn>
      <v-btn v-if="showPointToButton" fab small color="transparent" @click="lockToSelection">
        <img src="@/assets/images/svg/ui/point_to.svg" height="40px" style="min-height: 40px"></img>
      </v-btn>
      <v-btn v-if="!showPointToButton" fab small color="transparent" @mousedown="zoomOutButtonClicked">
        <img :class="{bt_disabled: !zoomOutButtonEnabled}" src="@/assets/images/svg/ui/remove_circle_outline.svg" height="40px" style="min-height: 40px"></img>
      </v-btn>
      <v-btn v-if="!showPointToButton" fab small color="transparent" @mousedown="zoomInButtonClicked">
        <img :class="{bt_disabled: !zoomInButtonEnabled}" src="@/assets/images/svg/ui/add_circle_outline.svg" height="40px" style="min-height: 40px"></img>
      </v-btn>
    </div>
    <v-snackbar bottom left :timeout="2000" v-model="copied" color="secondary" >
      Link copied
    </v-snackbar>
  </v-card>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import Moment from 'moment'
import swh from '@/assets/sw_helpers.js'

const showMinorNames = ref(false)
const wikipediaData = ref(undefined)
const shareLink = ref(undefined)
const showShareLinkDialog = ref(false)
const copied = ref(false)
const items = ref([])

const $store = inject('$store')
const $stel = inject('$stel')

const selectedObject = computed(() => $store.state.selectedObject)
const stelSelectionId = computed(() => $store.state.stel && $store.state.stel.selection ? $store.state.stel.selection : undefined)
const title = computed(() => selectedObject.value ? otherNames.value[0] : 'Selection')
const otherNames = computed(() => selectedObject.value ? swh.namesForSkySource(selectedObject.value, 26) : undefined)
const otherNames1to7 = computed(() => otherNames.value.slice(1, 8))
const otherNames8andMore = computed(() => showMinorNames.value ? otherNames.value.slice(8) : [])
const wikipediaSummary = computed(() => {
  if (!wikipediaData.value) return ''
  const page = wikipediaData.value.query.pages[Object.keys(wikipediaData.value.query.pages)[0]]
  if (!page || !page.extract) return ''
  const wl = '<b><a style="color: #62d1df;" target="_blank" rel="noopener" href="' + wikipediaLink.value + '">wikipedia</a></b></i>'
  return page.extract.replace(/<p>/g, '').replace(/<\/p>/g, '') + '<span class="grey--text text-caption" style="margin-left:auto; margin-right:0;"><i>&nbsp; ' + $t('more on {0}', [wl]) + '</span>'
})
const wikipediaLink = computed(() => {
  const page = wikipediaData.value.query.pages[Object.keys(wikipediaData.value.query.pages)[0]]
  if (!page || !page.extract) return ''
  return 'https://en.wikipedia.org/wiki/' + page.title
})
const type = computed(() => {
  if (!selectedObject.value) return $t('Unknown')
  let morpho = ''
  if (selectedObject.value.model_data && selectedObject.value.model_data.morpho) {
    morpho = swh.nameForGalaxyMorpho(selectedObject.value.model_data.morpho)
    if (morpho) {
      morpho = morpho + ' '
    }
  }
  return morpho + swh.nameForSkySourceType(selectedObject.value.types[0])
})
const icon = computed(() => swh.iconForSkySource(selectedObject.value))
const showPointToButton = computed(() => {
  if (!$store.state.stel.lock) return true
  if ($store.state.stel.lock !== $store.state.stel.selection) return true
  return false
})
const zoomInButtonEnabled = computed(() => {
  if (!$store.state.stel.lock || !selectedObject.value) return false
  return true
})
const zoomOutButtonEnabled = computed(() => {
  if (!$store.state.stel.lock || !selectedObject.value) return false
  return true
})
const extraButtons = computed(() => swh.selectedObjectExtraButtons)

watch(selectedObject, (s) => {
  showMinorNames.value = false
  wikipediaData.value = undefined
  if (!s) {
    if (timer) clearInterval(timer)
    timer = undefined
    return
  }
  items.value = computeItems()
  if (timer) clearInterval(timer)
  timer = setInterval(() => { items.value = computeItems() }, 1000)

  swh.getSkySourceSummaryFromWikipedia(s).then(data => {
    wikipediaData.value = data
  }, reason => { })
})

watch(stelSelectionId, (s) => {
  if (!$stel.core.selection) {
    $store.commit('setSelectedObject', 0)
    return
  }
  swh.sweObj2SkySource($stel.core.selection).then(res => {
    $store.commit('setSelectedObject', res)
  }, err => {
    console.log("Couldn't find info for object " + s + ':' + err)
    $store.commit('setSelectedObject', 0)
  })
})

watch(showShareLinkDialog, (b) => {
  shareLink.value = swh.getShareLink(this)
})

function computeItems() {
  const obj = $stel.core.selection
  if (!obj) return []
  const ret = []

  const addAttr = (key, attr, format) => {
    const v = obj.getInfo(attr)
    if (v && !isNaN(v)) {
      ret.push({
        key: key,
        value: format ? format(v) : v.toString()
      })
    }
  }

  addAttr($t('Magnitude'), 'vmag', formatMagnitude)
  addAttr($t('Distance'), 'distance', formatDistance)
  if (selectedObject.value.model_data) {
    if (selectedObject.value.model_data.radius) {
      ret.push({
        key: $t('Radius'),
        value: selectedObject.value.model_data.radius.toString() + ' Km'
      })
    }
    if (selectedObject.value.model_data.spect_t) {
      ret.push({
        key: $t('Spectral Type'),
        value: selectedObject.value.model_data.spect_t
      })
    }
    if (selectedObject.value.model_data.dimx) {
      const dimy = selectedObject.value.model_data.dimy ? selectedObject.value.model_data.dimy : selectedObject.value.model_data.dimx
      ret.push({
        key: $t('Size'),
        value: selectedObject.value.model_data.dimx.toString() + "' x " + dimy.toString() + "'"
      })
    }
  }
  const formatInt = function (num, padLen) {
    const pad = new Array(1 + padLen).join('0')
    return (pad + num).slice(-pad.length)
  }
  const formatRA = function (a) {
    const raf = $stel.a2tf(a, 1)
    return '<div class="radecVal">' + formatInt(raf.hours, 2) + '<span class="radecUnit">h</span>&nbsp;</div><div class="radecVal">' + formatInt(raf.minutes, 2) + '<span class="radecUnit">m</span></div><div class="radecVal">' + formatInt(raf.seconds, 2) + '.' + raf.fraction + '<span class="radecUnit">s</span></div>'
  }
  const formatAz = function (a) {
    const raf = $stel.a2af(a, 1)
    return '<div class="radecVal">' + formatInt(raf.degrees < 0 ? raf.degrees + 180 : raf.degrees, 3) + '<span class="radecUnit">°</span></div><div class="radecVal">' + formatInt(raf.arcminutes, 2) + '<span class="radecUnit">\'</span></div><div class="radecVal">' + formatInt(raf.arcseconds, 2) + '.' + raf.fraction + '<span class="radecUnit">"</span></div>'
  }
  const formatDec = function (a) {
    const raf = $stel.a2af(a, 1)
    return '<div class="radecVal">' + raf.sign + formatInt(raf.degrees, 2) + '<span class="radecUnit">°</span></div><div class="radecVal">' + formatInt(raf.arcminutes, 2) + '<span class="radecUnit">\'</span></div><div class="radecVal">' + formatInt(raf.arcseconds, 2) + '.' + raf.fraction + '<span class="radecUnit">"</span></div>'
  }
  const posCIRS = $stel.convertFrame($stel.core.observer, 'ICRF', 'JNOW', obj.getInfo('radec'))
  const radecCIRS = $stel.c2s(posCIRS)
  const raCIRS = $stel.anp(radecCIRS[0])
  const decCIRS = $stel.anpm(radecCIRS[1])
  ret.push({
    key: $t('Ra/Dec'),
    value: formatRA(raCIRS) + '&nbsp;&nbsp;&nbsp;' + formatDec(decCIRS)
  })
  const azalt = $stel.c2s($stel.convertFrame($stel.core.observer, 'ICRF', 'OBSERVED', obj.getInfo('radec')))
  const az = $stel.anp(azalt[0])
  const alt = $stel.anpm(azalt[1])
  ret.push({
    key: $t('Az/Alt'),
    value: formatAz(az) + '&nbsp;&nbsp;&nbsp;' + formatDec(alt)
  })
  addAttr($t('Phase'), 'phase', formatPhase)
  const vis = obj.computeVisibility()
  let str = ''
  if (vis.length === 0) {
    str = $t('Not visible tonight')
  } else if (vis[0].rise === null) {
    str = $t('Always visible tonight')
  } else {
    str = $t('Rise: {0}&nbsp;&nbsp;&nbsp; Set: {1}', [formatTime(vis[0].rise), formatTime(vis[0].set)])
  }
  ret.push({
    key: $t('Visibility'),
    value: str
  })
  return ret
}

function formatPhase(v) {
  return (v * 100).toFixed(0) + '%'
}

function formatMagnitude(v) {
  if (!v) {
    return 'Unknown'
  }
  return v.toFixed(2)
}

function formatDistance(d) {
  // d is in AU
  if (!d) {
    return 'NAN'
  }
  const ly = d * swh.astroConstants.ERFA_AULT / swh.astroConstants.ERFA_DAYSEC / swh.astroConstants.ERFA_DJY
  if (ly >= 0.1) {
    return ly.toFixed(2) + '<span class="radecUnit"> light years</span>'
  }
  if (d >= 0.1) {
    return d.toFixed(2) + '<span class="radecUnit"> AU</span>'
  }
  const meter = d * swh.astroConstants.ERFA_DAU
  if (meter >= 1000) {
    return (meter / 1000).toFixed(2) + '<span class="radecUnit"> km</span>'
  }
  return meter.toFixed(2) + '<span class="radecUnit"> m</span>'
}

function formatTime(jdm) {
  var d = new Date()
  d.setMJD(jdm)
  const utc = new Moment(d)
  utc.utcOffset($store.state.stel.utcoffset)
  return utc.format('HH:mm')
}

function unselect() {
  $stel.core.selection = 0
}

function lockToSelection() {
  if ($stel.core.selection) {
    $stel.pointAndLock($stel.core.selection, 0.5)
  }
}

function zoomInButtonClicked() {
  const currentFov = $store.state.stel.fov * 180 / Math.PI
  $stel.zoomTo(currentFov * 0.3 * Math.PI / 180, 0.4)
  zoomTimeout = setTimeout(() => { zoomInButtonClicked() }, 300)
}

function zoomOutButtonClicked() {
  const currentFov = $store.state.stel.fov * 180 / Math.PI
  $stel.zoomTo(currentFov * 3 * Math.PI / 180, 0.6)
  zoomTimeout = setTimeout(() => { zoomOutButtonClicked() }, 200)
}

function stopZoom() {
  if (zoomTimeout) {
    clearTimeout(zoomTimeout)
    zoomTimeout = undefined
  }
}

function extraButtonClicked(btn) {
  btn.callback()
}

function copyLink() {
  const input = document.querySelector('#link_inputid')
  input.focus()
  input.select()
  copied.value = document.execCommand('copy')
  window.getSelection().removeAllRanges()
  showShareLinkDialog.value = false
}

let timer
let zoomTimeout

onMounted(() => {
  window.addEventListener('mouseup', () => {
    stopZoom()
  })
})
</script>

<style>
.bt_disabled {
  filter: opacity(0.2);
}

.radecVal {
  display: inline-block;
  font-family: monospace;
  padding-right: 2px;
  font-size: 13px;
  font-weight: bold;
}

.radecUnit {
  color: #dddddd;
  font-weight: normal
}
</style>
