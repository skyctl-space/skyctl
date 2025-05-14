<template>
    <v-card class="pa-4" width="250">
        <h4 class="text-subtitle-1 mb-2">Guide</h4>
        <v-switch label="Enable Guiding" />
        <v-select :items="['Aggressive', 'Moderate', 'Passive']" label="Aggression" />
    </v-card>
</template>

<script setup lang="ts">
</script>