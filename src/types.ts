import { Store } from '@tauri-apps/plugin-store'

export type GeoLocation = {
    valid: boolean;
    latitude: number;
    longitude: number;
    error: string | null;
}

export interface Site {
    name: string;
    latitude: number;
    longitude: number;
    elevation?: number;
}

export class Settings {
    public checkUpdate: boolean;
    public sites: Site[] = [];
    public selectedSite?: string;
    public verifyTimeMatch: boolean;

    constructor() {
        this.checkUpdate = true;
        this.verifyTimeMatch = true;
        this.selectedSite = undefined;
        this.sites = [];
    }

    async loadSettings(store: Store) {
        this.checkUpdate = (await store.get<boolean>("checkUpdate")) ?? true;
        this.verifyTimeMatch = await store.get<boolean>("verifyTimeMatch") ?? true;
        this.selectedSite = await store.get<string>("selectedSite") ?? undefined;
        this.sites = await store.get<Site[]>("sites") ?? [];
    }

    async saveSettings(store: Store) {
        await store.set("checkUpdate", this.checkUpdate);
        await store.set("verifyTimeMatch", this.verifyTimeMatch);
        if (this.selectedSite != undefined) {
            await store.set("selectedSite", this.selectedSite);
        }
        await store.set("sites", this.sites);
        await store.save();
    }
}

export enum ConnectionType {
    ASIAIR = 1,
    INDI = 2,
    ALPACA = 3,
    SEESTAR = 4,
}

export class TelescopeRig {
    name: string;
    description?: string;
    connection: ConnectionType;
    connected: boolean;
    focalLength?: number;
    fov?: number;

    constructor(
        name: string,
        connection: ConnectionType,
        connected: boolean,
        description?: string,
        focalLength?: number,
        fov?: number
    ) {
        this.name = name;
        this.description = description;
        this.connection = connection;
        this.connected = connected;
        this.focalLength = focalLength;
        this.fov = fov;
    }
}

