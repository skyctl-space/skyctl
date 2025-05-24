<template>
  <v-card class="pa-4" width="250">
    <h4 class="text-subtitle-1 mb-2">Settings</h4>
    <v-switch label="Dark Mode" />
    <v-slider label="Brightness" :max="100" :step="1" />
  </v-card>
</template>

<script setup lang="ts">
</script>