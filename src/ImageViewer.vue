<template>
    <v-sheet v-show="props.showHistogram" class="floating-controls" elevation="8"
        :style="{ left: controlsPos.x + 'px', top: controlsPos.y + 'px', position: 'absolute', zIndex: 10, minWidth: '800px', cursor: dragging ? 'grabbing' : 'grab' }"
        @mousedown.stop="onControlsMouseDown">
        <v-row dense no-gutters class="d-flex" justify="space-between" align="start">
            <v-row dense no-gutters class="d-flex align-center justify-center">
                <v-col cols="12" class="d-flex" style="position:relative;">
                    <canvas :ref="el => histogramRefs[props.telescopeIndex] = el as HTMLCanvasElement"
                        class="histogram-canvas" style="flex: 1; width: 100%; height: 80px;"></canvas>
                    <div v-if="histogramTooltip && hoveredBin !== null"
                        :style="{ position: 'absolute', left: histogramTooltip.x + 'px', top: (histogramTooltip.y + 10) + 'px', pointerEvents: 'none', background: '#222', color: '#fff', padding: '2px 8px', borderRadius: '4px', fontSize: '12px', zIndex: 20, border: '1px solid #444', transform: 'translate(-50%, 0)' }">
                        Bin {{ hoveredBin }}:
                        <span style="color:red">{{ hoveredR }}</span>,
                        <span style="color:green">{{ hoveredG }}</span>,
                        <span style="color:lightblue">{{ hoveredB }}</span>
                    </div>
                </v-col>
            </v-row>
        </v-row>
        <v-row dense no-gutters class="mt-2">
            <v-col cols="2" class="d-flex align-center justify-center">
                <v-btn size="x-small" prepend-icon="mdi-refresh" block class="ml-2 mr-2" @click="resetStretch">Reset</v-btn>
            </v-col>
            <v-col cols="1"/>
            <v-col cols="2" class="d-flex align-center justify-center">
                <v-btn size="x-small" prepend-icon="mdi-chart-histogram" block class="ml-2 mr-2"
                    :variant="logScale ? 'outlined' : 'flat'" @click="toggleLogScale">Log Scale</v-btn>
            </v-col>
            <v-col cols="1"></v-col>
            <v-col cols="6">
                <v-row dense justify="end">
                    <v-col cols="4">
                        <span class="font-weight-bold">Min:</span> {{ Math.round(stats.min) }}
                    </v-col>
                    <v-col cols="4">
                        <span class="font-weight-bold">Max:</span> {{ Math.round(stats.max) }}
                    </v-col>
                    <v-col cols="4">
                        <span class="font-weight-bold">Avg:</span> {{ Math.round(stats.avg) }}
                    </v-col>
                </v-row>
            </v-col>
        </v-row>
        <v-row dense no-gutters class="d-flex justify-center align-center mt-2" @mousedown.stop>
            <v-slider v-model="targetBkg" :min="0" :max="1" :step="0.01" label="Target Background" hide-details
                thumb-label></v-slider>
            <v-slider v-model="shadowsClip" :min="-5" :max="0" :step="0.01" label="Shadows Clip" hide-details
                thumb-label></v-slider>
        </v-row>
    </v-sheet>
    <v-container fluid fill-height style="height:100%;width:100%;padding:0;">
        <canvas :ref="el => canvasRefs[props.telescopeIndex] = el as HTMLCanvasElement" class="fits-canvas"></canvas>
    </v-container>
</template>

<script setup lang="ts">
// --- Imports ---
import { ref, onMounted, watch } from 'vue';
import { listen, Event } from '@tauri-apps/api/event';

// --- Props and Models ---
// Props received from parent
const props = defineProps({
    telescopeIndex: { type: Number, required: true },
    showHistogram: { type: Boolean, default: true },
});
// v-model for busy state
const busy = defineModel<boolean>('busy');

// --- Canvas and Histogram References ---
// References to canvas and histogram elements for each telescope index
const canvasRefs = ref<Record<number, HTMLCanvasElement | null>>({});
const histogramRefs = ref<Record<number, HTMLCanvasElement | null>>({});

// --- Stretch Parameters and Stats ---
// Per-channel stretch parameters
const stretchParamsR = ref({ c0: 0, c1: 1, m: 0.5 });
const stretchParamsG = ref({ c0: 0, c1: 1, m: 0.5 });
const stretchParamsB = ref({ c0: 0, c1: 1, m: 0.5 });
// Image statistics
const stats = ref({ min: 0, max: 0, avg: 0 });
const logScale = ref(false);

// --- Stretch and Channel Parameters ---
const targetBkg = ref(0.25);
const shadowsClip = ref(-1.25);
const channelParams = ref([
    { median: 0, avg_dev: 0, max: 0 }, // R
    { median: 0, avg_dev: 0, max: 0 }, // G
    { median: 0, avg_dev: 0, max: 0 }, // B
]);

// --- Loaded Image Dimensions ---
const loadedImageHeight = ref(0);
const loadedImageWith = ref(0);

// --- Watchers ---
// Watch for changes in stretch controls and update image
watch([
    targetBkg,
    shadowsClip
], ([tBkg, sClip]) => {
    for (let i = 0; i < 3; ++i) {
        const median = channelParams.value[i].median;
        const avg_dev = channelParams.value[i].avg_dev;
        const max = channelParams.value[i].max;
        const { c0, c1, m } = autoStretchParams(median, avg_dev, max, tBkg, sClip);
        if (i === 0) stretchParamsR.value = { c0, c1, m };
        if (i === 1) stretchParamsG.value = { c0, c1, m };
        if (i === 2) stretchParamsB.value = { c0, c1, m };
    }
    updateImageUniformsAndRedraw(props.telescopeIndex);
});

// --- Floating Controls (Histogram) Position and Drag State ---
const controlsPos = ref({ x: 40, y: 40 });
const dragging = ref(false);
const dragOffset = ref({ x: 0, y: 0 });
function onControlsMouseDown(e: MouseEvent) {
    dragging.value = true;
    dragOffset.value = {
        x: e.clientX - controlsPos.value.x,
        y: e.clientY - controlsPos.value.y
    };
    window.addEventListener('mousemove', onControlsMouseMove);
    window.addEventListener('mouseup', onControlsMouseUp);
}
function onControlsMouseMove(e: MouseEvent) {
    if (!dragging.value) return;
    controlsPos.value.x = e.clientX - dragOffset.value.x;
    controlsPos.value.y = e.clientY - dragOffset.value.y;
}
function onControlsMouseUp() {
    dragging.value = false;
    window.removeEventListener('mousemove', onControlsMouseMove);
    window.removeEventListener('mouseup', onControlsMouseUp);
}

// --- Stretch Controls ---
function resetStretch() {
    targetBkg.value = 0.25;
    shadowsClip.value = -1.25;
    logScale.value = false;
}
function toggleLogScale() {
    logScale.value = !logScale.value;
    renderHistogram(props.telescopeIndex);
}

// --- Histogram Drawing and Data ---
const bins = 256;
const rBins = new Uint32Array(bins);
const gBins = new Uint32Array(bins);
const bBins = new Uint32Array(bins);

function smoothBins(bins: Uint32Array, window: number = 3): Float32Array {
    // Simple moving average smoothing
    const smoothed = new Float32Array(bins.length);
    const half = Math.floor(window / 2);
    for (let i = 0; i < bins.length; ++i) {
        let sum = 0;
        let count = 0;
        for (let j = -half; j <= half; ++j) {
            const idx = i + j;
            if (idx >= 0 && idx < bins.length) {
                sum += bins[idx];
                count++;
            }
        }
        smoothed[i] = sum / count;
    }
    return smoothed;
}

function renderHistogram(index: number) {
    // Draw histogram using lines
    const histogramCanvas = histogramRefs.value[index];
    if (!histogramCanvas) return;
    const ctx = histogramCanvas.getContext('2d');
    if (!ctx) return;
    ctx.clearRect(0, 0, histogramCanvas.width, histogramCanvas.height);
    // Smooth the bins before drawing
    const rSmoothed = smoothBins(rBins, 5);
    const gSmoothed = smoothBins(gBins, 5);
    const bSmoothed = smoothBins(bBins, 5);
    const maxVal = Math.max(...rSmoothed, ...gSmoothed, ...bSmoothed, 1);
    function drawLine(color: string, binsArr: Float32Array) {
        if (!ctx) return;
        if (!histogramCanvas) return;
        ctx.beginPath();
        ctx.strokeStyle = color;
        ctx.lineWidth = 1;
        for (let x = 0; x < bins; x++) {
            let value = binsArr[x];
            let y;
            if (logScale.value) {
                value = Math.log10(value + 1);
                const maxLog = Math.log10(maxVal + 1);
                y = histogramCanvas.height - (value / maxLog) * histogramCanvas.height;
            } else {
                y = histogramCanvas.height - (value / maxVal) * histogramCanvas.height;
            }
            if (x === 0) ctx.moveTo(x * histogramCanvas.width / bins, y);
            else ctx.lineTo(x * histogramCanvas.width / bins, y);
        }
        ctx.stroke();
    }
    drawLine('red', rSmoothed);
    drawLine('green', gSmoothed);
    drawLine('blue', bSmoothed);
}

// --- Canvas Resolution Helper ---
function ensureCanvasResolution(canvas: HTMLCanvasElement) {
    const dpr = window.devicePixelRatio || 1;
    const displayWidth = Math.round(canvas.clientWidth * dpr);
    const displayHeight = Math.round(canvas.clientHeight * dpr);
    if (canvas.width !== displayWidth || canvas.height !== displayHeight) {
        canvas.width = displayWidth;
        canvas.height = displayHeight;
    }
}

// --- Histogram Calculation ---
function updateHistogram(index: number) {
    const imgW = loadedImageWith.value;
    const imgH = loadedImageHeight.value;
    const canvas = canvasRefs.value[index];
    if (!canvas) return;
    const canvasW = canvas.width;
    const canvasH = canvas.height;
    const gl = canvas.getContext('webgl2');
    if (!gl) return;
    // Read pixels from the WebGL canvas
    const pixels = new Uint8Array(canvasW * canvasH * 4);
    gl.readPixels(0, 0, canvasW, canvasH, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
    // Compute the drawn image rectangle (preserving aspect ratio)
    const imgAspect = imgW / imgH;
    const canvasAspect = canvasW / canvasH;
    let drawWidth, drawHeight, offsetX, offsetY;
    if (imgAspect < canvasAspect) {
        drawHeight = canvasH;
        drawWidth = canvasH * imgAspect;
        offsetX = (canvasW - drawWidth) / 2;
        offsetY = 0;
    } else {
        drawWidth = canvasW;
        drawHeight = canvasW / imgAspect;
        offsetX = 0;
        offsetY = (canvasH - drawHeight) / 2;
    }
    rBins.fill(0);
    gBins.fill(0);
    bBins.fill(0);
    // Loop over canvas pixels, not image pixels
    for (let y = 0; y < canvasH; y++) {
        for (let x = 0; x < canvasW; x++) {
            // Only include pixels inside the drawn image area
            if (
                x >= offsetX && x < offsetX + drawWidth &&
                y >= offsetY && y < offsetY + drawHeight
            ) {
                const idx = (y * canvasW + x) * 4;
                const r = pixels[idx + 0];
                const g = pixels[idx + 1];
                const b = pixels[idx + 2];
                rBins[r]++;
                gBins[g]++;
                bBins[b]++;
            }
        }
    }
    renderHistogram(index);
}

// --- Histogram Hover State ---
const hoveredBin = ref<number | null>(null);
const hoveredR = ref<number | null>(null);
const hoveredG = ref<number | null>(null);
const hoveredB = ref<number | null>(null);
const histogramTooltip = ref<{ x: number; y: number } | null>(null);
function onHistogramMouseMove(e: MouseEvent) {
    const histogramCanvas = histogramRefs.value[props.telescopeIndex];
    if (!histogramCanvas) return;
    const rect = histogramCanvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    const bin = Math.floor((x / rect.width) * bins);
    if (bin < 0 || bin >= bins) {
        hoveredBin.value = null;
        hoveredR.value = null;
        hoveredG.value = null;
        hoveredB.value = null;
        histogramTooltip.value = null;
        return;
    }
    hoveredBin.value = bin;
    hoveredR.value = rBins[bin];
    hoveredG.value = gBins[bin];
    hoveredB.value = bBins[bin];
    histogramTooltip.value = { x, y };
}
function onHistogramMouseLeave() {
    hoveredBin.value = null;
    hoveredR.value = null;
    hoveredG.value = null;
    hoveredB.value = null;
    histogramTooltip.value = null;
}

// --- WebGL State Cache ---
const glStates: Record<number, any> = {};

// --- Canvas Resize Helper ---
function useResizeCanvasToParent(canvasRef: () => HTMLCanvasElement | null) {
    let resizeObserver: ResizeObserver | null = null;
    onMounted(() => {
        const canvas = canvasRef();
        if (!canvas || !canvas.parentElement) return;
        function resize() {
            if (!canvas) return;
            const parent = canvas.parentElement;
            if (!parent) return;
            const dpr = window.devicePixelRatio || 1;
            const width = parent.clientWidth;
            const height = parent.clientHeight;
            canvas.width = width * dpr;
            canvas.height = height * dpr;
            canvas.style.width = width + 'px';
            canvas.style.height = height + 'px';
        }
        resizeObserver = new ResizeObserver(resize);
        resizeObserver.observe(canvas.parentElement);
        resize();
    });
}

// --- Event Interface ---
interface image_update_event {
    index: number;
    image_data: {
        width: number;
        height: number;
        pixels: Uint16Array,
        stats: [
            { min: number; max: number; avg: number; median: number; avg_dev: number },
            { min: number; max: number; avg: number; median: number; avg_dev: number },
            { min: number; max: number; avg: number; median: number; avg_dev: number },
        ]
    };
}

// --- Main Mount Logic ---
onMounted(() => {
    // Build GL state for each canvas on mount
    const canvas = canvasRefs.value[props.telescopeIndex];
    if (!canvas) {
        console.error('Canvas not found for telescope index', props.telescopeIndex);
        return;
    }
    const gl = canvas.getContext('webgl2');
    if (!gl) {
        console.error('WebGL2 not supported');
        return;
    }
    gl.getExtension('EXT_color_buffer_float');
    // --- Shader sources ---
    const vertexShaderSource = `#version 300 es
        precision mediump float;
        in vec2 a_position;
        out vec2 v_texCoord;
        uniform vec2 u_scale;
        void main() {
            vec2 scaled = a_position * u_scale;
            v_texCoord = (scaled + 1.0) * 0.5;
            gl_Position = vec4(scaled, 0, 1);
        }`;
    const fragmentShaderSource = `#version 300 es
        precision highp float;
        precision highp sampler2D;
        in vec2 v_texCoord;
        out vec4 outColor;
        uniform sampler2D u_texture;
        uniform float r_c0, r_c1, r_m;
        uniform float g_c0, g_c1, g_m;
        uniform float b_c0, b_c1, b_m;
        float mtf(float m, float x) {
            if (x <= 0.0) return 0.0;
            if (x >= 1.0) return 1.0;
            if (x == m) return 0.5;
            return ((m - 1.0) * x) / (((2.0 * m - 1.0) * x) - m);
        }
        void main() {
            vec4 color = texture(u_texture, vec2(v_texCoord.x, 1.0 - v_texCoord.y));
            float r = (color.r - r_c0) / (r_c1 - r_c0);
            float g = (color.g - g_c0) / (g_c1 - g_c0);
            float b = (color.b - b_c0) / (b_c1 - b_c0);
            r = mtf(r_m, r);
            g = mtf(g_m, g);
            b = mtf(b_m, b);
            vec3 rgb = clamp(vec3(r, g, b), 0.0, 1.0);
            outColor = vec4(rgb.r, rgb.g, rgb.b, 1.0);
        }`;
    function createShader(gl: WebGL2RenderingContext, type: number, source: string) {
        const shader = gl.createShader(type);
        if (!shader) throw new Error('Failed to create shader');
        gl.shaderSource(shader, source);
        gl.compileShader(shader);
        if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
            gl.deleteShader(shader);
            throw new Error('Failed to compile shader');
        }
        return shader;
    }
    const vertexShader = createShader(gl, gl.VERTEX_SHADER, vertexShaderSource);
    const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fragmentShaderSource);
    const program = gl.createProgram();
    if (!program) throw new Error('Failed to create program');
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        gl.deleteProgram(program);
        throw new Error('Failed to link program');
    }
    const positionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([
        -1, -1, 1, -1, -1, 1,
        -1, 1, 1, -1, 1, 1,
    ]), gl.STATIC_DRAW);
    const positionLocation = gl.getAttribLocation(program, 'a_position');
    const uScaleLocation = gl.getUniformLocation(program, 'u_scale');
    const uTextureLocation = gl.getUniformLocation(program, 'u_texture');
    const rC0Location = gl.getUniformLocation(program, 'r_c0');
    const rC1Location = gl.getUniformLocation(program, 'r_c1');
    const rMLocation = gl.getUniformLocation(program, 'r_m');
    const gC0Location = gl.getUniformLocation(program, 'g_c0');
    const gC1Location = gl.getUniformLocation(program, 'g_c1');
    const gMLocation = gl.getUniformLocation(program, 'g_m');
    const bC0Location = gl.getUniformLocation(program, 'b_c0');
    const bC1Location = gl.getUniformLocation(program, 'b_c1');
    const bMLocation = gl.getUniformLocation(program, 'b_m');
    const texture = gl.createTexture();
    glStates[props.telescopeIndex] = {
        program, positionBuffer, positionLocation, uScaleLocation, uTextureLocation,
        rC0Location, rC1Location, rMLocation, gC0Location, gC1Location, gMLocation, bC0Location, bC1Location, bMLocation,
        texture
    };
    // Attach histogram hover event listeners
    const histogramCanvas = histogramRefs.value[props.telescopeIndex];
    if (histogramCanvas) {
        histogramCanvas.addEventListener('mousemove', onHistogramMouseMove);
        histogramCanvas.addEventListener('mouseleave', onHistogramMouseLeave);
    }
    // Attach ResizeObserver to fits-canvas
    useResizeCanvasToParent(() => canvasRefs.value[props.telescopeIndex]);
    // Listen to fits_image_updated event using Tauri's event system
    listen<image_update_event>('fits_image_updated', (event: Event<image_update_event>) => {
        const { index, image_data } = event.payload;
        if (index === props.telescopeIndex) {
            // Compute auto-stretch parameters
            if (!image_data.stats || image_data.stats.length < 3) {
                console.error('Invalid image data received');
                return;
            }
            {
                const { c0, c1, m } = autoStretchParams(image_data.stats[0].median, image_data.stats[0].avg_dev, image_data.stats[0].max);
                stretchParamsR.value = { c0, c1, m };
            }
            {
                const { c0, c1, m } = autoStretchParams(image_data.stats[1].median, image_data.stats[1].avg_dev, image_data.stats[1].max);
                stretchParamsG.value = { c0, c1, m };
            }
            {
                const { c0, c1, m } = autoStretchParams(image_data.stats[2].median, image_data.stats[2].avg_dev, image_data.stats[2].max);
                stretchParamsB.value = { c0, c1, m };
            }
            // Set per-channel median and avg_dev
            for (let i = 0; i < 3; ++i) {
                channelParams.value[i].median = image_data.stats[i].median;
                channelParams.value[i].avg_dev = image_data.stats[i].avg_dev;
                channelParams.value[i].max = image_data.stats[i].max;
            }

            // Convert Uint16Array to normalized Float32Array (per channel)
            let floatPixels: Float32Array;
            if (image_data.stats && image_data.stats.length === 3) {
                floatPixels = new Float32Array(image_data.pixels.length);
                const maxR = image_data.stats[0].max || 1;
                const maxG = image_data.stats[1].max || 1;
                const maxB = image_data.stats[2].max || 1;
                for (let i = 0; i < image_data.pixels.length; i += 3) {
                    floatPixels[i] = image_data.pixels[i] / maxR;
                    floatPixels[i + 1] = image_data.pixels[i + 1] / maxG;
                    floatPixels[i + 2] = image_data.pixels[i + 2] / maxB;

                    if (i < 10) { // Debugging: log first 10 pixels
                        console.log('Pixel:', i, 'R:', floatPixels[i], 'G:', floatPixels[i + 1], 'B:', floatPixels[i + 2]);
                    }
                }
            } else {
                floatPixels = new Float32Array(image_data.pixels);
            }

            renderImage(index, image_data.width, image_data.height, floatPixels);
            loadedImageHeight.value = image_data.height;
            loadedImageWith.value = image_data.width;
            updateHistogram(index);
            // Update stats
            const combinedStats = image_data.stats.reduce(
                (acc, stat) => {
                    acc.min = Math.min(acc.min, stat.min);
                    acc.max = Math.max(acc.max, stat.max);
                    acc.sum += stat.avg;
                    acc.count++;
                    return acc;
                },
                { min: Infinity, max: -Infinity, sum: 0, count: 0 }
            );
            stats.value = {
                min: combinedStats.min,
                max: combinedStats.max,
                avg: combinedStats.sum / combinedStats.count
            };
            busy.value = false;
        }
    });
});

// --- WebGL Image Redraw and Uniform Update ---
function updateImageUniformsAndRedraw(index: number) {
    const canvas = canvasRefs.value[index];
    if (!canvas) return;
    const gl = canvas.getContext('webgl2');
    if (!gl) return;
    const state = glStates[index];
    if (!state) return;
    gl.uniform1f(state.rC0Location, stretchParamsR.value.c0);
    gl.uniform1f(state.rC1Location, stretchParamsR.value.c1);
    gl.uniform1f(state.rMLocation, stretchParamsR.value.m);
    gl.uniform1f(state.gC0Location, stretchParamsG.value.c0);
    gl.uniform1f(state.gC1Location, stretchParamsG.value.c1);
    gl.uniform1f(state.gMLocation, stretchParamsG.value.m);
    gl.uniform1f(state.bC0Location, stretchParamsB.value.c0);
    gl.uniform1f(state.bC1Location, stretchParamsB.value.c1);
    gl.uniform1f(state.bMLocation, stretchParamsB.value.m);
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.drawArrays(gl.TRIANGLES, 0, 6);
    updateHistogram(props.telescopeIndex);
}

// --- WebGL Image Rendering ---
function renderImage(index: number, width: number, height: number, pixels: Float32Array) {
    const canvas = canvasRefs.value[index];
    if (!canvas) return;
    ensureCanvasResolution(canvas);
    const gl = canvas.getContext('webgl2');
    if (!gl) {
        console.error('WebGL2 not supported');
        return;
    }
    gl.getExtension('EXT_color_buffer_float');
    const imgW = width;
    const imgH = height;
    const canvasW = canvas.width;
    const canvasH = canvas.height;
    // Calculate scale to fit image fully inside the canvas (contain, never clipped)
    const scale = Math.min(canvasW / imgW, canvasH / imgH);
    const drawWidth = imgW * scale;
    const drawHeight = imgH * scale;
    // Set scale for vertex shader (normalized to [-1,1] clip space)
    const scaleX = drawWidth / canvasW;
    const scaleY = drawHeight / canvasH;
    const state = glStates[index];
    if (!state) return;
    gl.useProgram(state.program);
    gl.bindBuffer(gl.ARRAY_BUFFER, state.positionBuffer);
    gl.enableVertexAttribArray(state.positionLocation);
    gl.vertexAttribPointer(state.positionLocation, 2, gl.FLOAT, false, 0, 0);
    gl.uniform2f(state.uScaleLocation, scaleX, scaleY);
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, state.texture);
    gl.texImage2D(
        gl.TEXTURE_2D, 0, gl.RGB32F, imgW, imgH, 0,
        gl.RGB, gl.FLOAT, pixels
    );
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
    gl.uniform1i(state.uTextureLocation, 0);
    gl.uniform1f(state.rC0Location, stretchParamsR.value.c0);
    gl.uniform1f(state.rC1Location, stretchParamsR.value.c1);
    gl.uniform1f(state.rMLocation, stretchParamsR.value.m);
    gl.uniform1f(state.gC0Location, stretchParamsG.value.c0);
    gl.uniform1f(state.gC1Location, stretchParamsG.value.c1);
    gl.uniform1f(state.gMLocation, stretchParamsG.value.m);
    gl.uniform1f(state.bC0Location, stretchParamsB.value.c0);
    gl.uniform1f(state.bC1Location, stretchParamsB.value.c1);
    gl.uniform1f(state.bMLocation, stretchParamsB.value.m);
    gl.viewport(0, 0, canvas.width, canvas.height);
    gl.clearColor(0, 0, 0, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.drawArrays(gl.TRIANGLES, 0, 6);
}

// --- Auto-Stretch Parameter Calculation ---
function autoStretchParams(
    median: number, avgDev: number, max: number,
    targetBkg = 0.25,
    shadowsClip = -1.25
) {
    function mtf(m: number, x: number): number {
        if (x === 0) return 0;
        if (x === 1) return 1;
        if (x === m) return 0.5;
        return ((m - 1) * x) / (((2 * m - 1) * x) - m);
    }
    // Normalize median and avgDev
    const normalizedMedian = median / max;
    const normalizedAvgDev = avgDev / max;
    const c0 = Math.max(0, Math.min(1, normalizedMedian + (shadowsClip * normalizedAvgDev)));
    const c1 = 1;
    const m = mtf(targetBkg, normalizedMedian - c0);
    return { c0, c1, m };
}
</script>

<style scoped>
.fits-canvas {
    width: 100% !important;
    height: 100% !important;
    background: black;
}

.v-row.d-flex.align-center.justify-center {
    flex: 1 1 0%;
    height: 100%;
    min-height: 0;
}

.v-col.d-flex.align-center.justify-center {
    flex: 1 1 0%;
    height: 100%;
    min-height: 0;
}

.histogram-canvas {
    width: 100%;
    height: 100px;
    background-color: rgba(0, 0, 0, 0.4);
}

.floating-controls {
    user-select: none;
    background-color: rgba(0, 0, 0, 0.25);
    /* Optionally add more styling */
}
</style>