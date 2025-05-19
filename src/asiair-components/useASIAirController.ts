import { reactive, toRefs } from 'vue';

type ChannelStat = {
    min: number;
    max: number;
    avg: number;
    median: number;
    avg_dev: number;
}

type ASIAirState = {
    imageData: Uint16Array | null;
    imageWidth: number | null;
    imageHeight: number | null;
    imageStats: ChannelStat[];
    cameraInfo: string | null;
    cameraError: string | null;
    cameraBusy: boolean;
    selectedCamera: string | null;
    temperature: number | null;
    gain: number | null;
};

const asiairControllers = new Map<string, ReturnType<typeof createASIAirState>>();

function createASIAirState() {
    const state = reactive<ASIAirState>({
        imageData: null,
        imageWidth: null,
        imageHeight: null,
        imageStats: [],
        cameraInfo: null,
        cameraError: null,
        cameraBusy: false,
        selectedCamera: null,
        temperature: null,
        gain: null,
    });


    async function updateTemperature(newTemp: number) {
        state.temperature = newTemp;
        // optionally send to backend
    }

    async function updateGain(newGain: number) {
        state.gain = newGain;
        // optionally send to backend
    }

    async function selectCamera(cameraId: string) {
        state.selectedCamera = cameraId;
        // optionally send to backend
    }

    return {
        ...toRefs(state),
        updateTemperature,
        updateGain,
        selectCamera,
    };
}

export function useASIAirController(guid: string) {
    if (!asiairControllers.has(guid)) {
        asiairControllers.set(guid, createASIAirState());
    }

    return asiairControllers.get(guid)!;
}