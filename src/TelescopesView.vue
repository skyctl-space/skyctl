<script setup lang="ts">
import { ref, watch } from 'vue'
import { settings, saveSettings } from "./store";
import { Connection, ConnectionType } from "./types";
import { invoke } from '@tauri-apps/api/core';
import { listen } from "@tauri-apps/api/event";

async function startASIAIRDiscovery() {
  await invoke("start_asiair_discovery");
}

async function stopASIAIRDiscovery() {
  await invoke("stop_asiair_discovery");
}

interface ASIAIRDevice {
  title: string;
  value: string;
}

listen("discovered_device", (event) => {
  console.log("Device found:", event.payload);
  detectedASIAIRDevices.value = event.payload as ASIAIRDevice[];
});

const detectedASIAIRDevices = ref<ASIAIRDevice[]>([
]);

interface TelescopeConnection {
  config: Connection;
  configIdx: number;
  connected: boolean;
};

const telescopes = ref<TelescopeConnection[]>([])

function updateTelescopes() {
  if (settings.connections === undefined) {
    settings.connections = [];
  }

  telescopes.value = settings.connections.map((config, index) => ({
    config,
    configIdx: index,
    connected: false,
  }))
}

watch(
  () => settings.connections,
  (_) => {
    updateTelescopes();
  },
  { immediate: true }
)

const maximizedIndex = ref<number | null>(null);

function toggleMaximize(index: number | null) {
  maximizedIndex.value = maximizedIndex.value === index ? null : index;
}

// Modal state for adding a new connection
const showAddConnectionModal = ref(false);

async function showConnectionModal() {
  showAddConnectionModal.value = true;
  startASIAIRDiscovery();
}

async function hideConnectionModal() {
  showAddConnectionModal.value = false;
  stopASIAIRDiscovery();
}


// Temporary object to hold new connection data
const newConnection = ref<Connection>({
  name: '',
  description: '',
  type: ConnectionType.ASIAIR,
  host: '',
});
const newConnectionErrorMessage = ref('');

// Modal state for confirming connection removal
const showConfirmRemoveModal = ref(false);
const connectionToRemove = ref<number | null>(null);

// Save the new connection to settings
async function saveNewConnection() {
  if (newConnection.value.name && newConnection.value.host) {
    if (settings.connections === undefined) {
      settings.connections = [];
    }
    settings.connections.push({ ...newConnection.value });
    await saveSettings();
    resetNewConnection();
    updateTelescopes();
    await hideConnectionModal();
    newConnectionErrorMessage.value = '';
  } else {
    newConnectionErrorMessage.value = 'Please fill in all required fields.';
  }
}

// Reset the new connection form
function resetNewConnection() {
  newConnection.value = {
    name: '',
    description: '',
    type: ConnectionType.ASIAIR,
    host: '',
  };
}

// Remove a connection from settings
function removeConnection(index: number) {
  if (settings.connections && index >= 0 && index < settings.connections.length) {
    if (index == maximizedIndex.value) {
      toggleMaximize(index);
    }

    settings.connections.splice(index, 1);
    saveSettings();
    updateTelescopes();
  }
}

// Confirm before removing a connection
function confirmRemoveConnection(index: number) {
  connectionToRemove.value = index;
  showConfirmRemoveModal.value = true;
}

// Handle removal confirmation
function handleRemoveConnection() {
  if (connectionToRemove.value !== null) {
    removeConnection(connectionToRemove.value);
    connectionToRemove.value = null;
    showConfirmRemoveModal.value = false;
  }
}
</script>

<template>
  <v-container fluid class="pa-4 fill-height d-flex flex-column" style="width: 100%">
    <v-toolbar border density="compact" class="mb-4" :class="{ hidden: maximizedIndex !== null }">
      <v-toolbar-title text="Telescope Control"></v-toolbar-title>
      <v-btn elevation="4" prepend-icon="mdi-plus" @click="showConnectionModal()">Add Connection...</v-btn>
    </v-toolbar>
    <div class="panel-grid">
      <v-card v-for="(telescope, i) in telescopes" :key="i" class="panel"
        :class="{ maximized: maximizedIndex === i, hidden: maximizedIndex !== null && maximizedIndex !== i }"
        elevation="4">
        <v-card-title density="compact" class="d-flex justify-space-between align-center text-white">
          <v-menu>
            <template v-slot:activator="{ props }">
              <v-btn icon="mdi-dots-vertical" v-bind="props">
              </v-btn>
            </template>
            <v-list>
              <v-list-item>Connection string: {{ telescope.config.host }}</v-list-item>
              <v-list-item>
                <v-btn prepend-icon="mdi-delete" text="Remove" @click="confirmRemoveConnection(telescope.configIdx)" />
              </v-list-item>
            </v-list>
          </v-menu>


          <v-spacer></v-spacer>
          <span>{{ telescope.config.name }} ({{ telescope.config.type }})</span>
          <v-spacer></v-spacer>
          <v-btn size="small" variant="text" icon @click="telescope.connected = !telescope.connected"
            class="text-white">
            <v-icon :color="telescope.connected ? 'green' : 'red'" icon="mdi-connection"></v-icon>
          </v-btn>
          <v-btn size="small" variant="text" icon @click="toggleMaximize(i)" class="text-white">
            <v-icon>{{ maximizedIndex === i ? "mdi-arrow-collapse" : "mdi-arrow-expand" }}</v-icon>
          </v-btn>
        </v-card-title>

        <v-card-text class="panel-body bg-image-card">
          {{ telescope.config.description }}
        </v-card-text>
      </v-card>
    </div>

    <!-- Add Connection Modal -->
    <v-dialog v-model="showAddConnectionModal" persistent max-width="800px">
      <v-card prepend-icon="mdi-telescope" title="Add Connection">
        <v-card-text>
          <v-text-field v-model="newConnection.name" label="Name" required></v-text-field>
          <v-text-field v-model="newConnection.description" label="Description"></v-text-field>
          <v-select v-model="newConnection.type" :items="Object.values(ConnectionType)" label="Type"></v-select>
          <v-text-field v-model="newConnection.host" label="Connection String" required></v-text-field>
          <div v-if="newConnection.type === ConnectionType.ASIAIR">
            <v-divider />
            Discovering ASIAir devices... <v-progress-circular indeterminate></v-progress-circular>
            <v-list elevation="4">
              <v-list-item v-for="device in detectedASIAIRDevices" :key="device.value"
                @click="newConnection.host = device.value; newConnection.name = device.title" class="cursor-pointer">
                <template #prepend>
                  <v-icon icon="mdi-telescope" />
                </template>
                <v-list-item-title>{{ device.title }}</v-list-item-title>
              </v-list-item>
            </v-list>
          </div>
        </v-card-text>
        <v-card-actions>
          <v-messages color="red" :messages="newConnectionErrorMessage"
            :active="!!newConnectionErrorMessage"></v-messages>
          <v-btn color="primary" @click="saveNewConnection">Save</v-btn>
          <v-btn @click="hideConnectionModal()">Cancel</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Confirm Remove Connection Modal -->
    <v-dialog v-model="showConfirmRemoveModal" persistent max-width="400px">
      <v-card title="Confirm Removal">
        <v-card-text>
          Are you sure you want to remove this connection?
        </v-card-text>
        <v-card-actions>
          <v-btn color="red" @click="handleRemoveConnection">Yes, Remove</v-btn>
          <v-btn @click="showConfirmRemoveModal = false">Cancel</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-container>
</template>

<style scoped>
/* Adjust the height of the panel grid to fit below the nav bar */
.panel-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  /* 2 columns */
  grid-template-rows: repeat(2, auto);
  /* Max of 2 rows visible */
  gap: 1rem;
  width: 100%;
  height: calc(100vh - 140px);
  /* Subtract nav-bar height (adjust this value if needed) */
  overflow-y: auto;
  /* Enable scrolling */
  position: relative;
  padding-right: 1rem;
  /* Prevent content overflow if scrollbar appears */
}

/* Fix panel height and make sure they do not stretch */
.panel {
  position: relative;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  max-width: 100%;
  /* Ensures panels do not overflow horizontally */
  height: calc((100vh / 2) - 80px);
  /* Set fixed height for each card */
}

.panel-body {
  overflow: auto;
  flex: 1;
}

.panel.maximized {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  z-index: 10;
  margin: 0;
  border-radius: 0.5rem;
  max-width: 100%;
  /* Prevents the panel from overflowing horizontally */
  max-height: 100%;
  /* Prevents the panel from overflowing vertically */
  height: 100%;
  /* Ensure maximized panel takes full height inside container */
}

/* When a panel inside is maximized */
.panel-grid:has(.panel.maximized) {
  height: calc(100vh - 80px);
  overflow: hidden; /* optional: prevent scrolling behind maximized panel */
}

.hidden {
  display: none;
}

.bg-image-card {
  background-image: url('/gaia_milkyway.jpg');
  background-size: cover;
  background-position: center;
  position: relative;
  color: white; /* for text visibility */
  overflow: hidden;
  filter: brightness(0.4); /* Lower = darker image */
}
</style>