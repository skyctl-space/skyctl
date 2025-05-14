<template>
  <v-card class="pa-4" width="250">
    <h4 class="text-subtitle-1 mb-2">Camera</h4>
    <v-switch label="Cooling" />
    <v-slider label="Gain" :max="100" :step="1" />
  </v-card>
</template>

<script setup lang="ts">
</script>