<template>
    <v-row no-gutters align="center" class="control-container border-thin pa-2" justify="center">
        <v-col class="joystick-col" cols="12">
            <div class="joystick-area">
                <v-btn icon @click="move('up')">
                    <v-icon>mdi-chevron-up</v-icon>
                </v-btn>

                <div class="middle-row">
                    <v-btn icon @click="move('left')">
                        <v-icon>mdi-chevron-left</v-icon>
                    </v-btn>

                    <v-chip class="tracking-chip" :color="tracking ? 'green' : 'red'" label
                        @click="tracking = !tracking">
                        {{ tracking ? 'Tracking' : 'Stopped' }}
                    </v-chip>

                    <v-btn icon @click="move('right')">
                        <v-icon>mdi-chevron-right</v-icon>
                    </v-btn>
                </div>

                <v-btn icon @click="move('down')">
                    <v-icon>mdi-chevron-down</v-icon>
                </v-btn>
            </div>
        </v-col>
        <v-col cols="12" class="slider-row">
            <v-slider v-model="rate" min="0" :max="steps.length - 1" step="1" thumb-size="15" label="Rate" show-ticks
                class="rate-slider pt-8" thumb-label="always" hide-details>
                <template v-slot:thumb-label="{ modelValue }">
                    {{ steps[modelValue] }}x
                </template>
            </v-slider>
        </v-col>
    </v-row>
</template>



<script setup lang="ts">
import { ref } from 'vue';

const tracking = ref(false);

const steps = [0.25, 0.5, 1, 2, 4, 8, 20, 60, 720, 1440]
const rate = ref(9)

// const selectedRate = computed({
//     get: () => rate.value,
//     set: (val: number) => {
//         rate.value = val
//     }
// })

function move(direction: string) {
    console.log(`Moving ${direction} at rate ${rate.value}`);
}
</script>

<style scoped>
.control-container {
    background-color: rgba(0, 0, 0, 0.1);
}

.joystick-area {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    height: 100%;
    padding-left: 8px;
}

.middle-row {
    display: flex;
    align-items: center;
    gap: 8px;
}

.slider-row {
    padding-top: 8px;
}

.rate-slider {
    margin: 0 8px;
}
</style>
