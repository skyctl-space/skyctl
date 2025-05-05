// Stellarium Web - Copyright (c) 2022 - Stellarium Labs SRL
//
// This program is licensed under the terms of the GNU AGPL v3, or
// alternatively under a commercial licence.
//
// The terms of the AGPL v3 license can be found in the main directory of this
// repository.

<template>
  <div style="position: relative;" v-click-outside="resetSearch">
    <v-text-field prepend-icon="mdi-magnify" label="Search..." v-model="searchText" @keyup.native.esc="resetSearch()" hide-details single-line></v-text-field>
    <v-list dense v-if="showList" two-line :style="listStyle">
      <v-list-item v-for="source in autoCompleteChoices" :key="source.names[0]" @click="sourceClicked(source)">
        <v-list-item-action>
          <img :src="iconForSkySource(source)"/>
        </v-list-item-action>
          <v-list-item-title>{{ nameForSkySource(source) }}</v-list-item-title>
          <v-list-item-subtitle>{{ typeToName(source.types[0]) }}</v-list-item-subtitle>
      </v-list-item>
    </v-list>
  </div>
</template>

<script lang="ts" setup>
import { ref, computed, watch, onMounted } from 'vue';
import { querySkySources, cleanupOneSkySourceName, nameForSkySourceType, iconForSkySource as utilIconForSkySource, nameForSkySource as utilNameForSkySource } from '@/utils';
import { useStellariumStore } from '@/stores';
import _ from 'lodash';

const props = defineProps({
  value: { type: Object, required: false },
  floatingList: { type: Boolean, required: false }
});

const emit = defineEmits(['input']);

const stellariumStore = useStellariumStore();
const autoCompleteChoices = ref<{ names: string[]; types: string[] }[]>([]);
const searchText = ref('');
const lastQuery = ref<string | undefined>(undefined);

const listStyle = computed(() => {
  return props.floatingList ? 'position: absolute; z-index: 1000; margin-top: 8px' : '';
});

const showList = computed(() => {
  return searchText.value !== '';
});

const sourceClicked = (val: any) => {
  emit('input', val);
  resetSearch();
};

const resetSearch = () => {
  searchText.value = '';
};

const refresh = _.debounce(() => {
  const str = searchText.value.toUpperCase().replace(/\s+/g, '');
  if (lastQuery.value === str) {
    return;
  }
  lastQuery.value = str;
  querySkySources(str, 10).then(
    (results: { names: string[]; types: string[] }[]) => {
      if (str !== lastQuery.value) {
        console.log('Cancelled query: ' + str);
        return;
      }
      autoCompleteChoices.value = results;
    },
    (err: any) => {
      console.log(err);
    }
  );
}, 200);

const nameForSkySource = (s: any) => {
  const cn = cleanupOneSkySourceName(stellariumStore.stel, s.match, 0);
  const n = utilNameForSkySource(stellariumStore.stel, s);
  return cn === n ? n : `${cn} (${n})`;
};

const typeToName = (t: any) => {
  return nameForSkySourceType(stellariumStore.stel, t);
};

const iconForSkySource = (s: any) => {
  return utilIconForSkySource(s);
};

onMounted(() => {
  const onClick = (_e: Event) => {
    if (searchText.value !== '') {
      searchText.value = '';
    }
  };
  const guiParent = document.querySelector('stel') || document.body;
  guiParent.addEventListener('click', onClick, false);
});

watch(searchText, (newValue) => {
  if (newValue === '') {
    autoCompleteChoices.value = [];
    lastQuery.value = undefined;
    return;
  }
  refresh();
});
</script>

<style>

</style>
