import { reactive, toRefs, toRef, Ref } from 'vue';
import { invoke, Channel } from '@tauri-apps/api/core';
import { listen } from "@tauri-apps/api/event";

type ChannelStat = {
    min: number;
    max: number;
    avg: number;
    median: number;
    avg_dev: number;
}

type CameraInfo = {
  chip_size: [number, number];
  bins: number[];
  pixel_size_um: number;
  unity_gain: number;
  has_cooler: boolean;
  is_color: boolean;
  is_usb3_host: boolean;
  debayer_pattern: string;
};

type CurrentImage = {
    data: Uint16Array | null;
    width: number | null;
    height: number | null;
    stats: ChannelStat[];
}

type ImageProgress =
  | { event: 'fetching' }
  | { event: 'downsampling' }
  | { event: 'debayering' }
  | {
    event: 'rendering';
    data: {
      width: number;
      height: number;
      stats: any[]; // or define Stat type
    };
  }
  | { event: 'error'; data: string };

enum BayerPattern {
  NONE = 'NONE',
  RGGB = 'RGGB',
  BGGR = 'BGGR',
  GRBG = 'GRBG',
  GBRG = 'GBRG',
}

type Camera = {
    name: string;
    info: CameraInfo;
    isConnected: boolean;
    infoMessage: string | null;
    errorMessage: string | null;
    isBusy: boolean;
    bayer_pattern: BayerPattern;
}

type ConnectedCamera = {
  name: string;
  id: number;
  path: string;
  dslr: boolean;
};

type CameraState =
  | { state: 'close' }
  | { state: 'idle'; name: string; path: string };

type ASIAirPublicState = {
    isASIAirConnected: boolean; // Indicates if the ASIAir is actually connected at this point, may be retrying to connect if false
    shouldASIAIRBeConnected: boolean; // Indicates if the ASIAir should be connected (e.g. if the user has clicked "Connect")
    availableCameras: ConnectedCamera[];
    mainCameraName: string | null;
    mainCameraState: CameraState;
    guideCameraName: string | null;
    guideCameraState: CameraState;
    mainCamera: Camera;
    currentImage: CurrentImage;
}

type ASIAirState = {
    public: ASIAirPublicState;

    // Private state
    guid: string;
    connection: string;
};

const asiairControllers = new Map<string, ReturnType<typeof createASIAirState>>();

interface ConnectionChange {
  guid: string;
  connected: boolean;
}

// Listen for the "asiair_connection_state" event
listen<ConnectionChange>("asiair_connection_state", (event) => {
  const { guid, connected } = event.payload;

  const controller = asiairControllers.get(guid);
  if (controller) {
    controller.isASIAirConnected.value = connected;
  }
});

listen<string>("asiair_camera_state_change", async (event) => {
  const guid = event.payload;
  const controller = asiairControllers.get(guid);
  if (controller) {
    console.log(`[${guid}] Camera state changed`);
    await invoke<CameraState>("main_camera_get_state", {
        guid: guid,
    }).then((cameraState) => {
        controller.mainCameraState.value = cameraState;
        if (cameraState.state === 'idle') {
            controller.mainCamera.value.isConnected = true;
            updateCameraInfo(guid, controller.mainCamera);
        } else {
            controller.mainCamera.value.isConnected = false;
        }
    }).catch((error: any) => {
        console.error(`[${guid}] Error getting main camera state:`, error);
    });
  }
});

async function updateCameraInfo(guid: string, mainCamera: Ref<Camera>) {
    await invoke<CameraInfo>("main_camera_get_info", {
        guid: guid,
    }).then((cameraInfo) => {
        mainCamera.value.info = cameraInfo;
        switch (cameraInfo.debayer_pattern) {
            case 'RG':
                mainCamera.value.bayer_pattern = BayerPattern.RGGB;
                break;
            case 'BG':
                mainCamera.value.bayer_pattern = BayerPattern.BGGR;
                break;
            case 'GR':
                mainCamera.value.bayer_pattern = BayerPattern.GRBG;
                break;
            case 'GB':
                mainCamera.value.bayer_pattern = BayerPattern.GBRG;
                break;
            default:
                mainCamera.value.bayer_pattern = BayerPattern.NONE;
                break;
        }
    }).catch((error: any) => {
        console.error(`[${guid}] Error getting camera info:`, error);
    });
}


async function onConnection(state: ASIAirState) {
    // Get list of cameras available
    await invoke<ConnectedCamera[]>("get_connected_cameras", {
        guid: state.guid,
    }).then((cameras) => {
        console.log(`[${state.guid}] Connected cameras:`, cameras);
        state.public.availableCameras = cameras;
    }).catch((error: any) => {
        console.error(`[${state.guid}] Error getting connected cameras:`, error);
        state.public.availableCameras = [];
    });

    await invoke<CameraState>("main_camera_get_state", {
        guid: state.guid,
    }).then((cameraState) => {
        state.public.mainCameraState = cameraState;
        if (cameraState.state === 'idle') {
            state.public.mainCamera.isConnected = true;
            updateCameraInfo(state.guid, toRef(state.public, 'mainCamera'));
        } else {
            state.public.mainCamera.isConnected = false;
        }
    }).catch((error: any) => {
        console.error(`[${state.guid}] Error getting main camera state:`, error);
        state.public.mainCameraState = { state: 'close' };
    });

    await invoke<string>("main_camera_get_name", {
        guid: state.guid,
    }).then((cameraName) => {
        state.public.mainCameraName = cameraName;
    }).catch((error: any) => {
        console.error(`[${state.guid}] Error getting main camera name:`, error);
        state.public.mainCameraName = null;
    });

    await invoke<string>("guide_camera_get_name", {
        guid: state.guid,
    }).then((cameraName) => {
        state.public.guideCameraName = cameraName;
    }).catch((error: any) => {
        console.error(`[${state.guid}] Error getting main camera name:`, error);
        state.public.guideCameraName = null;
    });
}

function normalizeToUint8Array(data: unknown): Uint8Array {
  if (data instanceof Uint8Array) return data;
  if (Array.isArray(data)) return new Uint8Array(data);
  throw new Error("Expected Uint8Array or Array<number> from backend");
}

function createASIAirState(guid: string, connection: string | undefined) {
    const state = reactive<ASIAirState>({
        guid,
        connection : connection || "auto",
        public: {
            isASIAirConnected: false,
            shouldASIAIRBeConnected: false,
            availableCameras: [],
            mainCameraName: null,
            mainCameraState: { state: 'close' },
            guideCameraName: null,
            guideCameraState: { state: 'close' },
            mainCamera: {
                name: "",
                info: {
                    chip_size: [0, 0],
                    bins: [],
                    pixel_size_um: 0,
                    unity_gain: 0,
                    has_cooler: false,
                    is_color: false,
                    is_usb3_host: false,
                    debayer_pattern: "",
                },
                bayer_pattern: BayerPattern.NONE,
                isConnected: false,
                infoMessage: null,
                errorMessage: null,
                isBusy: false,
            },
            currentImage: {
                data: null,
                width: null,
                height: null,
                stats: [],
            },
        },
    });

    async function connect() {
        if (state.public.shouldASIAIRBeConnected) {
            return;
        }

        try {
            await invoke("asiair_attach", {
                guid: state.guid,
                connection: state.connection,
            });
            state.public.isASIAirConnected = true;
            state.public.shouldASIAIRBeConnected = true;

            // TODO:
            // Detect state
        } catch (error: any) {
            state.public.isASIAirConnected = false;
            throw error;
        }

        onConnection(state).then(() => {
            console.log("ASIAir initialized");
        });
    }

    async function disconnect() {
        if (!state.public.shouldASIAIRBeConnected) {
            return;
        }

        state.public.shouldASIAIRBeConnected = false;
        await invoke("asiair_deattach", {
            'guid': state.guid,
        }).then(() => {
            state.public.isASIAirConnected = false;

            // TODO: clean up state

        }).catch((error: any) => {
            // We failed to disconnect
            console.log(error);
        });

    }

    async function trigger_capture() {
        let mainCamera = state.public.mainCamera;
        if (!mainCamera) {
            throw new Error("No main camera selected");
        }

        let imgNotificationChannel: Channel<ImageProgress> = new Channel<ImageProgress>();

        imgNotificationChannel.onmessage = (message) => {
            switch (message.event) {
            case 'fetching':
                mainCamera.infoMessage = "Fetching image...";
                break;
            case 'downsampling':
                mainCamera.infoMessage = "Downsampling...";
                break;
            case 'debayering':
                mainCamera.infoMessage = "Debayering...";
                break;
            case 'rendering': {
                const { width, height, stats } = message.data;
                let currentImage = state.public.currentImage;
                currentImage.width= width;
                currentImage.height = height;
                currentImage.stats = stats;
                mainCamera.infoMessage = "Rendering...";
                break;
            }
            case 'error':
                console.error(`[${state.guid}] Error:`, message.data);
                mainCamera.errorMessage = message.data;
                mainCamera.infoMessage = null;
                mainCamera.isBusy = false;
                break;
            default:
                mainCamera.errorMessage = "Unknown event";
                mainCamera.infoMessage = null;
                mainCamera.isBusy = false;
            }
        };

        mainCamera.errorMessage = null;
        mainCamera.infoMessage = null;
        mainCamera.isBusy = true;

        let imgDataChannel: Channel<unknown> = new Channel<unknown>();
        imgDataChannel.onmessage = (raw) => {
            const u8 = normalizeToUint8Array(raw);
            const buffer = u8.buffer.slice(u8.byteOffset, u8.byteOffset + u8.byteLength);

            const result = new Uint16Array(buffer.byteLength / 2);
            const view = new DataView(buffer);
            for (let i = 0; i < result.length; i++) {
            result[i] = view.getUint16(i * 2, true); // little-endian
            }

            let currentImage = state.public.currentImage;
            currentImage.data = result;
            mainCamera.infoMessage = null;
            mainCamera.isBusy = false;
        };


        await invoke('main_camera_get_current_img', {
            guid: state.guid,
            bayerPattern: state.public.mainCamera.bayer_pattern,
            sender: imgNotificationChannel,
            binarySender: imgDataChannel,
        }).catch((e) => {
            console.error(`[${state.guid}] Capture failed:`, e);
            mainCamera.errorMessage = "Image capture failed";
        });
    }

    async function main_camera_open(camera_name: string) {
        if ((state.public.mainCameraState.state !== 'close') &&
            (state.public.mainCameraName !== camera_name)) {
            throw new Error("Another camera is already open");
        }

        await invoke("main_camera_set_name", {
            guid: state.guid,
            name: camera_name,
        }).then(() => {
            console.log(`[${state.guid}] Camera name set to ${camera_name}`);
            state.public.mainCameraName = camera_name;
        }
        ).catch((error: any) => {
            console.error(`[${state.guid}] Error setting camera name:`, error);
            state.public.mainCameraName = null;
        });
            

        await invoke("main_camera_open", {
            guid: state.guid
        }); // Let the callbacks handle the state changes
    }

    async function main_camera_close() {
        let mainCamera = state.public.mainCamera;
        if (!mainCamera) {
            throw new Error("No main camera selected");
        }
        
        await invoke("main_camera_close", {
            guid: state.guid
        }).then(() => {
            console.log(`[${state.guid}] Camera closed`);
            mainCamera.isConnected = false;
        }).catch((error: any) => {
            console.error(`[${state.guid}] Error closing camera:`, error);
            mainCamera.isConnected = true;
        }); // Let the callbacks handle the state changes
    }

    async function main_camera_set_exposure(exposure: number) {
        await invoke("main_camera_set_exposure", {
            guid: state.guid,
            exposure: exposure,
        }).then(() => {
            console.log(`[${state.guid}] Camera exposure set to ${exposure}`);
        }).catch((error: any) => {
            console.error(`[${state.guid}] Error setting camera exposure:`, error);
        });
    }


    return {
        ...toRefs(state.public),
        connect,
        disconnect,
        trigger_capture,
        main_camera_open,
        main_camera_close,
        main_camera_set_exposure,
    };
}

export function useASIAirController(guid: string, connection: string | undefined) {
    if (!asiairControllers.has(guid)) {
        asiairControllers.set(guid, createASIAirState(guid, connection));
    }

    return asiairControllers.get(guid)!;
}