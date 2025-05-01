<script setup lang="ts">
import { ref, inject } from 'vue';
import { useRouter } from 'vue-router';
import { settings } from "./settings";
import { Site } from './types';
import { saveSettings } from "./settings";
import { formatLatitude, formatLongitude } from './utils'
import { GeoLocation } from "./types";

const geoLocation = inject<GeoLocation>('geoLocation', {
    latitude: 0,
    longitude: 0,
    valid: false,
    error: null
});

const router = useRouter();
const showAddSiteModal = ref(false); // State to control the modal visibility

const showMapModal = ref(false); // State to control the map modal visibility
const selectedLocation = ref<{ latitude: number; longitude: number }>({latitude: 0, longitude: 0});

function openMapDialog(siteIdx: number) {
  selectedLocation.value = {
    latitude: settings.sites[siteIdx].latitude,
    longitude: settings.sites[siteIdx].longitude,
  }
  showMapModal.value = true
}


const newSite = ref<Site>({
    name: 'New Site',
    latitude: 0,
    longitude: 0,
    elevation: undefined,
}); // Temporary object to hold new site data
const newSiteErrorMessage = ref(''); // Error message for new site validation
const unsavedChanges = ref(false); // Track if there are unsaved changes

const rules = {
    required: (value: string | number) => !!value || 'Field is required',
    valid_latitude: (value: number) => (value >= -90 && value <= 90) || 'Latitude must be between -90 and 90',
    valid_longitude: (value: number) => (value >= -180 && value <= 180) || 'Longitude must be between -180 and 180',
    valid_altitude: (value: number) => (value >= 0 && value <= 8849) || 'Elevation must be above sea level and below Mount Everest',
};


// Function to add a new site
function addSite() {
    // Validate the new site data (the latitude and longitude could be 0)
    if (newSite.value.name) {
        // Check if the site already exists
        const siteExists = settings.sites.some((site) => site.name === newSite.value.name);
        if (siteExists) {
            newSiteErrorMessage.value = "Site with this name already exists.";
            return;
        }

        // Validate latitude and longitude
        if (newSite.value.latitude < -90 || newSite.value.latitude > 90) {
            newSiteErrorMessage.value = "Latitude must be between -90 and 90.";
            return;
        }
        if (newSite.value.longitude < -180 || newSite.value.longitude > 180) {
            newSiteErrorMessage.value = "Longitude must be between -180 and 180.";
            return;
        }

        settings.sites.push({ ...newSite.value }); // Add the new site to the settings
        resetNewSite(); // Reset the form
        showAddSiteModal.value = false; // Close the modal
        unsavedChanges.value = true; // Mark changes as unsaved
    }
}

// Function to reset the new site form
function resetNewSite() {
    newSite.value = {
        name: '',
        latitude: 0,
        longitude: 0,
        elevation: undefined,
    };
    newSiteErrorMessage.value = ''; // Clear any error messages
}

// Function to delete a site
function deleteSite(index: number) {
    settings.sites.splice(index, 1); // Remove the site from the list
    unsavedChanges.value = true; // Mark changes as unsaved
    deleteDialog.value = false; // Close the delete confirmation dialog

    if (settings.selectedSiteIdx === index) {
        settings.selectedSiteIdx = undefined; // Clear the selected site if it was deleted
    }
}
const deleteDialog = ref(false); // State to control the delete confirmation dialog
const deleteIndex = ref<number>(0); // Index of the site to be deleted

function confirmDelete(index: number) {
    deleteIndex.value = index; // Set the index of the site to be deleted
    deleteDialog.value = true; // Open the delete confirmation dialog
}

function findLocation() {
    if (geoLocation.valid) {
        newSite.value.latitude = geoLocation.latitude; // Set the latitude
        newSite.value.longitude = geoLocation.longitude; // Set the longitude
        return;
    } else if (geoLocation.error) {
        newSiteErrorMessage.value = geoLocation.error;
        return;
    }
}

// Save settings before navigating away
async function viewSaveSettings() {
    await saveSettings(); // Save settings to the store
    unsavedChanges.value = false; // Reset unsaved changes flag
}

// Navigation guard to prompt the user before leaving
router.beforeEach((_to, from, next) => {
    if (from.name == "settings" && unsavedChanges.value) {
        viewSaveSettings().then(() => next());
    }
    else {
        next();
    }
});
</script>

<template>
    <v-container fluid>
        <v-toolbar class="py-3 ps-3" density="compact" title="Settings">
            <v-btn elevation="4" class=".v-col-4 .offset-4" prepend-icon="mdi-content-save" @click="viewSaveSettings"
                :disabled="!unsavedChanges">
                Save Settings
            </v-btn>
        </v-toolbar>
        <v-expansion-panels class="overflow-y-auto pe-3" variant="popout">
            <v-expansion-panel title="General Settings">
                <v-expansion-panel-text>
                    <v-switch color="indigo" inset label="Check for updates on startup." v-model="settings.checkUpdate"
                        @change="unsavedChanges = true"></v-switch>
                </v-expansion-panel-text>
            </v-expansion-panel>
            <v-expansion-panel>
                <v-expansion-panel-title>
                    <template v-slot:default="{ expanded }">
                        <v-row no-gutters>
                            <v-col class="d-flex justify-start" cols="4"> Sites </v-col>
                            <v-col class="d-flex justify-end" cols="8">
                                <v-fade-transition leave-absolute>
                                    <v-btn v-if="expanded" prepend-icon="mdi-plus" @click="showAddSiteModal = true">Add
                                        New Site</v-btn>
                                </v-fade-transition>
                            </v-col>
                        </v-row>
                    </template>
                </v-expansion-panel-title>
                <v-expansion-panel-text>
                    <div v-if="settings.sites.length === 0" class="text-center">
                        <p>No sites available. Click "Add New Site" to create one.</p>
                    </div>
                    <v-list density="compact" v-else>
                        <v-list-item v-for="(site, i) in settings.sites" :key="i" :value="site">
                            <template v-slot:prepend>
                                <v-icon icon="mdi-telescope"></v-icon>
                            </template>

                            <v-list-item-title> {{ site.name }} ({{ formatLatitude(site.latitude) }}, {{
                                formatLongitude(site.longitude)
                                }} )</v-list-item-title>

                            <template v-slot:append>
                                <v-btn icon="mdi-map-marker" @click="openMapDialog(i)"></v-btn>
                                <v-btn icon="mdi-delete" variant="text" @click="confirmDelete(i)"></v-btn>

                                <v-dialog v-model="deleteDialog" max-width="400px">
                                    <v-card>
                                        <v-card-title class="text-h6">Confirm Deletion</v-card-title>
                                        <v-card-text>Are you sure you want to delete this site?</v-card-text>
                                        <v-card-actions>
                                            <v-btn text @click="deleteDialog = false">Cancel</v-btn>
                                            <v-btn color="red" text @click="deleteSite(deleteIndex)">Delete</v-btn>
                                        </v-card-actions>
                                    </v-card>
                                </v-dialog>
                            </template>
                        </v-list-item>
                    </v-list>
                </v-expansion-panel-text>
            </v-expansion-panel>
            <v-expansion-panel title="Telescope Control">
                <v-expansion-panel-text>
                    <v-switch color="indigo" inset
                        label="Alert if time of the active site does not match telescope when connecting."
                        v-model="settings.verifyTimeMatch" @change="unsavedChanges = true"></v-switch>
                </v-expansion-panel-text>
            </v-expansion-panel>
        </v-expansion-panels>

        <!-- Modal for Adding New Site -->
        <v-dialog v-model="showAddSiteModal" max-width="500px">
            <v-card prepend-icon="mdi-telescope" title="Add New Site">
                <v-card-text>
                    <v-form>
                        <v-row dense>
                            <v-text-field v-model="newSite.name" label="Site Name" required></v-text-field>
                        </v-row>
                        <v-row dense>
                            <v-col cols="5">
                                <v-text-field v-model="newSite.latitude" label="Latitude" type="number"
                                    :rules="[rules.valid_latitude]" required></v-text-field>
                            </v-col>
                            <v-col cols="5">
                                <v-text-field v-model="newSite.longitude" label="Longitude" type="number"
                                    :rules="[rules.valid_longitude]" required></v-text-field>
                            </v-col>
                            <v-spacer></v-spacer>
                            <v-col cols="2">
                                <v-btn icon="mdi-map-marker" @click="findLocation()"></v-btn>
                            </v-col>
                        </v-row>
                        <v-text-field v-model="newSite.elevation" label="Elevation (optional)" type="number"
                            :rules="[rules.valid_altitude]"></v-text-field>
                    </v-form>
                </v-card-text>
                <v-divider></v-divider>
                <v-card-actions>
                    <v-messages color="red" :messages="newSiteErrorMessage"
                        :active="!!newSiteErrorMessage"></v-messages>
                    <v-btn color="primary" @click="addSite">Add</v-btn>
                    <v-btn text @click="showAddSiteModal = false">Cancel</v-btn>
                </v-card-actions>
            </v-card>
        </v-dialog>



        <!-- Modal for Google Maps -->
        <v-dialog v-model="showMapModal" max-width="800px">
            <v-card>
                <v-card-title>
                    Location on Google Maps
                    <v-spacer></v-spacer>
                    <v-btn icon="mdi-close" @click="showMapModal = false"></v-btn>
                </v-card-title>
                <v-card-text>
                    <iframe
                        :src="`https://www.google.com/maps?q=${selectedLocation.latitude},${selectedLocation.longitude}&z=15&output=embed`"
                        width="100%" height="400" style="border:0;" allowfullscreen=false loading="lazy"></iframe>
                </v-card-text>
            </v-card>
        </v-dialog>
    </v-container>
</template>
