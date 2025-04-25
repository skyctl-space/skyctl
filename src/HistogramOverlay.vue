<template>
    <v-sheet class="histogram-sheet" elevation="4" rounded :style="{ top: position.y + 'px', left: position.x + 'px' }"
        @mousedown="startDrag">
        <v-row class="d-flex" justify="space-between" align="start" no-gutters>
            <!-- Histogram Canvas (10 columns) -->
            <v-col cols="auto">
                <canvas ref="canvasRef" :width="width" :height="height" class="histogram-canvas"
                    @mousemove="onMouseMove" @mouseup="stopDrag" @mouseleave="stopDrag" />
            </v-col>

            <!-- Buttons (2 columns) -->
            <v-col cols="auto" class="d-flex flex-column justify-start align-center pl-2">
                <v-btn size="x-small" prepend-icon="mdi-refresh" block class="mb-2"
                    @click="resetHistogram">Reset</v-btn>
                <v-btn size="x-small" prepend-icon="mdi-chart-histogram" block :variant="logScale ? 'flat' : 'outlined'"
                    @click="logScale = !logScale">LogScale</v-btn>

                <v-btn size="x-small" prepend-icon="mdi-arrow-down" block class="mt-2"
                    @click="showAdjustments = !showAdjustments">
                    {{ showAdjustments ? 'Hide Adjustments' : 'Show Adjustments' }}
                </v-btn>
            </v-col>
        </v-row>
        <v-row v-if="showAdjustments" class="d-flex justify-center align-center" @mousedown.stop>
            <v-slider v-model="targetBkg" :min="0" :max="1" step="0.01" label="Target Background" thumb-label
                @end="emitSliderChanges" />
            <v-slider v-model="shadowsClip" :min="-5" :max="0" step="0.01" label="Shadows Clipping" thumb-label
                @end="emitSliderChanges" />
        </v-row>
    </v-sheet>
</template>

<script lang="ts" setup>
import { defineEmits, ref, watch, onMounted } from 'vue'

const showAdjustments = ref(false)
const targetBkg = ref(0.25)
const shadowsClip = ref(-1.0)

const emit = defineEmits<{
  (event: 'update:targetBkg', value: number): void
  (event: 'update:shadowsClip', value: number): void
}>()

const emitSliderChanges = () => {
    // Emit updated values to parent component
    console.log('Emitting targetBkg:', targetBkg.value)
    console.log('Emitting shadowsClip:', shadowsClip.value)
    emit('update:targetBkg', targetBkg.value)
    emit('update:shadowsClip', shadowsClip.value)
}

function resetHistogram() {
    targetBkg.value = 0.25
    shadowsClip.value = -1.0
    emit('update:targetBkg', targetBkg.value)
    emit('update:shadowsClip', shadowsClip.value)
}


interface Props {
    r?: Uint32Array
    g?: Uint32Array
    b?: Uint32Array
    width: number
    height: number
}

const props = defineProps<Props>()

const canvasRef = ref<HTMLCanvasElement | null>(null)
const width = props.width ?? 512
const height = props.height ?? 100

const position = ref({ x: 20, y: 20 }) // initial floating position
let dragging = false
let dragStart = { x: 0, y: 0 }

const startDrag = (e: MouseEvent) => {
    dragging = true
    dragStart = { x: e.clientX - position.value.x, y: e.clientY - position.value.y }
    window.addEventListener('mousemove', onDrag)
    window.addEventListener('mouseup', stopDrag)
}

const onDrag = (e: MouseEvent) => {
    if (dragging) {
        position.value.x = e.clientX - dragStart.x
        position.value.y = e.clientY - dragStart.y
    }
}

const stopDrag = () => {
    dragging = false
    window.removeEventListener('mousemove', onDrag)
    window.removeEventListener('mouseup', stopDrag)
}

// --- Histogram drawing ---
function getScaledBins(bins: Uint32Array, targetWidth: number): Uint32Array {
    const binScale = bins.length / targetWidth
    const scaled = new Uint32Array(targetWidth)
    for (let i = 0; i < targetWidth; i++) {
        const start = Math.floor(i * binScale)
        const end = Math.floor((i + 1) * binScale)
        for (let j = start; j < end; j++) {
            scaled[i] += bins[j]
        }
    }
    return scaled
}

const emptyBins = new Float32Array(512).fill(0)

const logScale = ref(false)

watch(logScale, () => drawHistogram())

function smoothBins(bins: Uint32Array, radius: number = 1): Float32Array {
    const smoothed = new Float32Array(bins.length)
    for (let i = 0; i < bins.length; i++) {
        let sum = 0
        let count = 0
        for (let j = -radius; j <= radius; j++) {
            const idx = i + j
            if (idx >= 0 && idx < bins.length) {
                sum += bins[idx]
                count++
            }
        }
        smoothed[i] = sum / count
    }
    return smoothed
}

function drawHistogram() {
    const canvas = canvasRef.value
    if (!canvas) return
    const ctx = canvas.getContext('2d')
    if (!ctx) return

    ctx.clearRect(0, 0, width, height)

    const rBins = props.r ? smoothBins(getScaledBins(props.r, width)) : emptyBins
    const gBins = props.g ? smoothBins(getScaledBins(props.g, width)) : emptyBins
    const bBins = props.b ? smoothBins(getScaledBins(props.b, width)) : emptyBins

    const maxVal = Math.max(...rBins, ...gBins, ...bBins, 1)

    function drawLine(color: string, bins: Float32Array) {
        if (!ctx) return
        ctx.beginPath()
        ctx.strokeStyle = color
        ctx.lineWidth = 1

        const scaleFactor = logScale.value ? Math.log(1 + maxVal) : maxVal

        for (let x = 0; x < width; x++) {
            const val = bins[x]
            const scaledVal = logScale.value ? Math.log(1 + val) : val
            const y = height - (scaledVal / scaleFactor) * height
            x === 0 ? ctx.moveTo(x, y) : ctx.lineTo(x, y)
        }

        ctx.stroke()
    }

    drawLine('red', rBins)
    drawLine('green', gBins)
    drawLine('blue', bBins)
}

watch(() => [props.r, props.g, props.b], drawHistogram, { deep: true })
onMounted(() => {
    console.log('Histogram mounted ' + canvasRef.value)

    drawHistogram
}
)

function onMouseMove(e: MouseEvent) {
    if (dragging) {
        onDrag(e)
    }
}
</script>

<style scoped>
.histogram-sheet {
    position: absolute;
    z-index: 1000;
    cursor: grab;
    padding: 8px;
    background-color: rgba(0, 0, 0, 0.25);
}

.histogram-canvas {
    background-color: rgba(0, 0, 0, 0.4);
    border-radius: 8px;
    display: block;
    width: 100%;
}

.draggable-line {
    position: absolute;
    top: 0;
    bottom: 0;
    width: 6px;
    background-color: rgba(255, 255, 255, 0.3);
    cursor: ew-resize;
}
</style>