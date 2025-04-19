<script setup lang="ts">
import { ref, computed } from 'vue'
import { settings } from "./store";
import { Connection } from "./types";

interface TelescopeConnection {
  config: Connection;
  configIdx: number;
  connected: boolean;
};

const telescopes = computed<TelescopeConnection[] | undefined>(() => {
  if (settings.connections === undefined) {
    return [];
  }
  const t = settings.connections.map((config, index) => ({
    config: config,
    configIdx: index,
    connected: false,
  }))

  return t;
});

const tab = ref(null)
</script>

<template>
      <v-tabs align-tabs="end" v-model="tab" background-color="transparent">
        <v-tab rounded="lg" prepend-icon="mdi-telescope" v-for="(telescope, index) in telescopes" :value="index"
          :color="telescope.connected ? 'green' : 'red'">
          {{ telescope.config.name }}
        </v-tab>
      </v-tabs>
      <!-- <v-btn prepend-icon="mdi-plus"></v-btn> -->

      <v-tabs-window v-model="tab">
        <v-tabs-window-item v-for="(telescope, index) in telescopes" :value="index">
          <v-container fluid>

            <v-card-text> {{ telescope.config.description }}
            </v-card-text>


            <v-card-subtitle>
              <v-row>
                <v-col cols="6">
                  <v-list-item prepend-icon="mdi-connection"
                    :title="telescope.connected ? 'Connected' : 'Disconnected'"></v-list-item>
                </v-col>
                <!-- <v-col cols="6">
                  <v-list-item prepend-icon="mdi-camera" :title="telescope.config.fl + 'mm'"></v-list-item>
                </v-col> -->
              </v-row>
            </v-card-subtitle>


            <v-card-actions>
              <v-btn :color="telescope.connected ? 'red' : 'green'" @click="telescope.connected = !telescope.connected">
                {{ telescope.connected ? 'Disconnect' : 'Connect' }}
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
