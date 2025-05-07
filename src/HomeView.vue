<script setup lang="ts">
</script>

<template>
    <v-container fluid class="pa-0 fill-height fill-width  d-flex flex-column" style="width: 100%">
        <v-row class="bg-image" align="center" justify="center">
            <v-col cols="12">
                <v-img src="/SkyCtlBanner.png" cover class="banner"/>
            </v-col>
            <div class="bottom-right">
                <a style="text-decoration: none; color: inherit;" href="https://www.esa.int/ESA_Multimedia/Keywords/Description/Milky_Way/(result_type)/images">Image Credit: ESA/Gaia/DPAC, Stefan Payne-Wardenaar</a>
            </div>
        </v-row>
    </v-container>
</template>

<style scoped>
.bg-image {
    background-image: url('/gaia_milkyway.jpg');
    background-size: cover;
    background-position: center;
    background-color: rgba(0, 0, 0, 0.1);
    width: 100%;
    height: 100%;
    background-blend-mode: overlay;
}

.banner {
    opacity: 0.1; /* Adjust opacity to make banner translucent */
}

.bottom-right {
    position: absolute;
    bottom: 20px;
    /* 10px from the bottom */
    right: 30px;
    /* 10px from the right */
    color: white;
    /* optional styling */
    z-index: 20;
    /* make sure it's above other content */
}
</style>