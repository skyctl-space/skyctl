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

export enum ConnectionType {
    ASIAIR = "ASIAir",
    INDI = "Indi",
    ALPACA = "Alpaca",
    SEESTAR = "SeeStar",
}

export interface Connection {
    name: string;
    description: string;
    type: ConnectionType;
    host: string;
}

export class Settings {
    public checkUpdate: boolean;
    public sites: Site[] = [];
    public selectedSiteIdx?: number;
    public verifyTimeMatch: boolean;
    public connections?: Connection[] = [];

    constructor() {
        this.checkUpdate = true;
        this.verifyTimeMatch = true;
        this.selectedSiteIdx = undefined;
        this.sites = [];
        this.connections = [];
    }

    async loadSettings(store: Store) {
        this.checkUpdate = (await store.get<boolean>("checkUpdate")) ?? true;
        this.verifyTimeMatch = await store.get<boolean>("verifyTimeMatch") ?? true;
        this.selectedSiteIdx = await store.get<number>("selectedSiteIdx") ?? undefined;
        this.sites = await store.get<Site[]>("sites") ?? [];
        this.connections = await store.get<Connection[]>("connections") ?? [];
    }

    async saveSettings(store: Store) {
        console.log("Saving settings to store " + this.selectedSiteIdx);
        await store.set("checkUpdate", this.checkUpdate);
        await store.set("verifyTimeMatch", this.verifyTimeMatch);
        if (this.selectedSiteIdx != undefined) {
            await store.set("selectedSiteIdx", this.selectedSiteIdx);
        }
        await store.set("sites", this.sites);
        await store.set("connections", this.connections);
        await store.save();
    }
}
