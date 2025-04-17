<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ConnectionType, TelescopeRig } from './types.ts'
//import { invoke } from "@tauri-apps/api/core";

const tab = ref(null)

const rigs = ref([] as TelescopeRig[]);

onMounted(() => {
  rigs.value.push(
    new TelescopeRig(
      "Rokinon",
      ConnectionType.ASIAIR,
      true,
      "Rokinon 135mm f/2.0",
      135,
      0.5
    )
  );
  rigs.value.push(
    new TelescopeRig(
      "EdgeHD",
      ConnectionType.INDI,
      false,
      'Celestron EdgeHD 11"',
      2032,
      0.25
    )
  );
  rigs.value.push(
    new TelescopeRig(
      "RedCat",
      ConnectionType.ALPACA,
      false,
      "William Optics RedCat 71",
      250,
      0.2
    )
  );
});

</script>

<template>
      <v-tabs align-tabs="end" v-model="tab" background-color="transparent">
        <v-tab rounded="lg" prepend-icon="mdi-telescope" v-for="(rig, index) in rigs" :value="index"
          :color="rig.connected ? 'green' : 'red'">
          {{ rig.name }}
        </v-tab>
      </v-tabs>
      <!-- <v-btn prepend-icon="mdi-plus"></v-btn> -->

      <v-tabs-window v-model="tab">
        <v-tabs-window-item v-for="(rig, index) in rigs" :value="index">
          <v-container fluid>

            <v-card-text> {{ rig.description }}
            </v-card-text>


            <v-card-subtitle>
              <v-row>
                <v-col cols="6">
                  <v-list-item prepend-icon="mdi-connection"
                    :title="rig.connected ? 'Connected' : 'Disconnected'"></v-list-item>
                </v-col>
                <v-col cols="6">
                  <v-list-item prepend-icon="mdi-camera" :title="rig.focalLength + 'mm'"></v-list-item>
                </v-col>
              </v-row>
            </v-card-subtitle>


            <v-card-actions>
              <v-btn :color="rig.connected ? 'red' : 'green'" @click="rig.connected = !rig.connected">
                {{ rig.connected ? 'Disconnect' : 'Connect' }}
              </v-btn>
            </v-card-actions>

            <v-speed-dial location="top center" transition="slide-y-reverse-transition">
              <template v-slot:activator="{ props: activatorProps }">
                <v-fab v-bind="activatorProps" size="large" icon="$vuetify"></v-fab>
              </template>

              <v-btn key="1" icon="$success"></v-btn>
              <v-btn key="2" icon="$info"></v-btn>
              <v-btn key="3" icon="$warning"></v-btn>
              <v-btn key="4" icon="$error"></v-btn>
            </v-speed-dial>

          </v-container>
        </v-tabs-window-item>
      </v-tabs-window>
</template>
