<script setup lang="ts">
import { inject } from 'vue'
import { TelescopeConnection } from './types'

const { telescopeIndex = 0 } = defineProps({
    telescopeIndex: Number
})

const telescopes = inject<TelescopeConnection[]>('telescopes', [])
</script>

<template>
    <v-container fluid>
        TBD {{ telescopes[telescopeIndex].connected ? 'connected' : 'disconnected' }}
    </v-container>
</template>
