<template>
    <v-row class="fill-height" align="center" justify="center">
        <v-col cols="auto">
            <v-container elevation="3">
                <v-row class="px-4 pt-2" justify="space-between" align="center">
                    <v-col class="d-flex align-center" cols="auto">
                        <v-icon class="mr-2">mdi-weather-partly-cloudy</v-icon>
                        <span class="text-h6 font-weight-medium">Weather Forecast for {{ siteName }}</span>
                    </v-col>

                    <v-col cols="auto">
                        <a href="https://open-meteo.com/" target="_blank" rel="noopener noreferrer"
                            class="d-flex align-center text-caption" style="text-decoration: none; color: inherit;">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="currentColor" class="-mt-1"
                                style="filter: drop-shadow( 3px 3px 2px rgba(0, 0, 0, .7))" viewBox="0 0 16 16"
                                width="48" height="48">
                                <path
                                    d="M7 8a3.5 3.5 0 0 1 3.5 3.555.5.5 0 0 0 .624.492A1.503 1.503 0 0 1 13 13.5a1.5 1.5 0 0 1-1.5 1.5H3a2 2 0 1 1 .1-3.998.5.5 0 0 0 .51-.375A3.502 3.502 0 0 1 7 8zm4.473 3a4.5 4.5 0 0 0-8.72-.99A3 3 0 0 0 3 16h8.5a2.5 2.5 0 0 0 0-5h-.027z">
                                </path>
                                <path
                                    d="M10.5 1.5a.5.5 0 0 0-1 0v1a.5.5 0 0 0 1 0v-1zm3.743 1.964a.5.5 0 1 0-.707-.707l-.708.707a.5.5 0 0 0 .708.708l.707-.708zm-7.779-.707a.5.5 0 0 0-.707.707l.707.708a.5.5 0 1 0 .708-.708l-.708-.707zm1.734 3.374a2 2 0 1 1 3.296 2.198c.199.281.372.582.516.898a3 3 0 1 0-4.84-3.225c.352.011.696.055 1.028.129zm4.484 4.074c.6.215 1.125.59 1.522 1.072a.5.5 0 0 0 .039-.742l-.707-.707a.5.5 0 0 0-.854.377zM14.5 6.5a.5.5 0 0 0 0 1h1a.5.5 0 0 0 0-1h-1z">
                                </path>
                            </svg>
                            <span>Powered by Open-Meteo</span>
                        </a>
                    </v-col>

                    <v-col cols="auto">
                        <v-combobox v-model="selectedDays" :items="[1, 3, 7, 14]" label="Days" density="compact"
                            hide-details variant="outlined" style="max-width: 200px" />
                    </v-col>
                </v-row>


                <v-row />

                <v-skeleton-loader v-if="loading" type="card" class="mb-4" :loading="loading" />

                <v-alert v-if="error" type="error" title="Error fetching weather" class="mb-4">
                    {{ error }}
                </v-alert>

                <v-container>
                    <!-- Forecast display -->
                    <div v-if="!loading && forecast.length" class="d-flex">
                        <!-- ❄️ Fixed left labels column -->
                        <div class="fixed-labels">
                            <v-row dense>
                                <v-col cols="12" class="font-weight-bold text-center pa-">Time</v-col>
                                <v-divider class="pa-1" />

                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-white-balance-sunny</v-icon> Sun Elev
                                </v-col>
                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-weather-night</v-icon> Moon Elev
                                </v-col>

                                <v-divider class="pa-1" />

                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-thermometer</v-icon> Temp
                                </v-col>
                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-thermometer</v-icon> Feels Like
                                </v-col>

                                <v-divider class="pa-1" />


                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-weather-cloudy</v-icon> Total %
                                </v-col>
                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-weather-cloudy</v-icon> Low %
                                </v-col>
                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-weather-cloudy</v-icon> Mid %
                                </v-col>
                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-weather-cloudy</v-icon> High %
                                </v-col>

                                <v-divider class="pa-1" />

                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-water-percent</v-icon> Humidity
                                </v-col>
                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-weather-pouring</v-icon> Probability
                                </v-col>
                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-weather-pouring</v-icon> Amount
                                </v-col>
                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-weather-fog</v-icon> Dew Point
                                </v-col>

                                <v-divider class="pa-1" />
                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-weather-windy</v-icon> Wind
                                </v-col>
                                <v-col cols="12" class="d-flex align-center">
                                    <v-icon size="18" class="mr-1">mdi-binoculars</v-icon> Visibility
                                </v-col>
                            </v-row>
                        </div>

                        <!-- 🔁 Scrollable hourly data -->
                        <div class="forecast-scroll">
                            <template v-for="(group, _index) in groupedForecast" :key="_index">
                                <div class="forecast-day-group">
                                    <div class="forecast-day-header">
                                        {{ formatDate(group.date) }}
                                    </div>
                                    <template v-for="(entry, idx) in group.entries" :key="idx">
                                        <div class="forecast-hour" :class="{
                                            'current-hour': isCurrentLocalHour(entry.time),
                                            'even-hour': idx % 2 === 0
                                        }">
                                            <v-row dense>
                                                <v-col cols="12" class="text-center font-weight-medium">
                                                    {{ formatHour(entry.time) }}
                                                </v-col>
                                                <v-divider class="pa-1" />

                                                <v-col cols="12" class="text-center ml-0"
                                                    :style="{ backgroundColor: `rgba(251, 192, 45, ${illuminationBars[_index].sunElevations[DateTime.fromISO(entry.time, { zone: timezone }).hour].toFixed(2)})` }">
                                                    {{
                                                        Math.round(illuminationBars[_index].sunElevations[DateTime.fromISO(entry.time,
                                                    { zone: timezone }).hour] * 100) }}%
                                                </v-col>
                                                <v-col cols="12" class="text-center ml-0"
                                                    :style="{ backgroundColor: `rgba(144, 202, 249, ${illuminationBars[_index].moonElevations[DateTime.fromISO(entry.time, { zone: timezone }).hour].toFixed(2)})` }">
                                                    {{
                                                        Math.round(illuminationBars[_index].moonElevations[DateTime.fromISO(entry.time,
                                                    { zone: timezone }).hour] * 100) }}%
                                                </v-col>
                                                <v-divider class="pa-1" />

                                                <v-col cols="12" class="text-center">{{ entry.temperature_2m
                                                }}°C</v-col>
                                                <v-col cols="12" class="text-center">{{ entry.apparent_temperature
                                                }}°C</v-col>
                                                <v-divider class="pa-1" />
                                                <v-col cols="12" class="text-center ml-1"
                                                    :style="{ backgroundColor: cloudBackground(entry.cloudcover), color: entry.cloudcover > 75 ? 'black' : 'white' }">
                                                    {{ entry.cloudcover }}%</v-col>
                                                <v-col cols="12" class="text-center ml-1"
                                                    :style="{ backgroundColor: cloudBackground(entry.cloudcoverlow), color: entry.cloudcoverlow > 75 ? 'black' : 'white' }">
                                                    {{ entry.cloudcoverlow }}%</v-col>
                                                <v-col cols="12" class="text-center ml-1"
                                                    :style="{ backgroundColor: cloudBackground(entry.cloudcovermid), color: entry.cloudcovermid > 75 ? 'black' : 'white' }">
                                                    {{ entry.cloudcovermid }}%</v-col>
                                                <v-col cols="12" class="text-center ml-1"
                                                    :style="{ backgroundColor: cloudBackground(entry.cloudcoverhigh), color: entry.cloudcoverhigh > 75 ? 'black' : 'white' }">
                                                    {{ entry.cloudcoverhigh }}%</v-col>
                                                <v-divider class="pa-1" />
                                                <v-col cols="12" class="text-center">{{ entry.relative_humidity_2m
                                                }}%</v-col>
                                                <v-col cols="12" class="text-center">{{ entry.precipitation }}%</v-col>
                                                <v-col cols="12" class="text-center">{{ entry.rain }}mm</v-col>
                                                <v-col cols="12" class="text-center">{{ entry.dewpoint_2m }}°C</v-col>
                                                <v-divider class="pa-1" />
                                                <v-col cols="12" class="text-center">{{ entry.wind_speed_10m }}
                                                    km/h</v-col>
                                                <v-col cols="12" class="text-center">{{ entry.visibility }} m</v-col>
                                            </v-row>
                                        </div>
                                    </template>
                                </div>
                            </template>
                        </div>
                    </div>
                </v-container>
            </v-container>
        </v-col>
    </v-row>
</template>


<script setup lang="ts">
import { ref, computed, watchEffect } from 'vue'
import { settings } from './settings'
import { DateTime } from 'luxon'
import SunCalc from 'suncalc'

const daylightInfo = computed(() => {
    return groupedForecast.value.map(group => {
        const date = DateTime.fromISO(group.date, { zone: timezone.value }).startOf('day')
        const times = SunCalc.getTimes(date.toJSDate(), latitude.value, longitude.value)
        const moonTimes = SunCalc.getMoonTimes(date.toJSDate(), latitude.value, longitude.value)

        return {
            date: group.date,
            sunrise: DateTime.fromJSDate(times.sunrise).setZone(timezone.value).toFormat('HH:mm'),
            sunset: DateTime.fromJSDate(times.sunset).setZone(timezone.value).toFormat('HH:mm'),
            moonrise: moonTimes.rise ? DateTime.fromJSDate(moonTimes.rise).setZone(timezone.value).toFormat('HH:mm') : null,
            moonset: moonTimes.set ? DateTime.fromJSDate(moonTimes.set).setZone(timezone.value).toFormat('HH:mm') : null,
            sunriseRaw: times.sunrise,
            sunsetRaw: times.sunset,
            moonriseRaw: moonTimes.rise ?? null,
            moonsetRaw: moonTimes.set ?? null,
        }
    })
})

const illuminationBars = computed(() => {
    return daylightInfo.value.map(day => {
        const sunElevations = Array(24).fill(0)
        const moonElevations = Array(24).fill(0)

        for (let h = 0; h < 24; h++) {
            const dt = DateTime.fromISO(day.date, { zone: timezone.value }).plus({ hours: h })
            const jsDate = dt.toJSDate()

            const sunPos = SunCalc.getPosition(jsDate, latitude.value, longitude.value)
            const moonPos = SunCalc.getMoonPosition(jsDate, latitude.value, longitude.value)

            // Altitude in radians, max ~1.57 (90°); clamp to 0–1 range
            sunElevations[h] = Math.max(0, sunPos.altitude) / (Math.PI / 2)
            moonElevations[h] = Math.max(0, moonPos.altitude) / (Math.PI / 2)
        }

        return { sunElevations, moonElevations }
    })
})

const selectedDays = ref(3)

interface WeatherEntry {
    time: string
    cloudcover: number
    cloudcoverlow: number
    cloudcovermid: number
    cloudcoverhigh: number
    rain: number
    dewpoint_2m: number
    relative_humidity_2m: number
    temperature_2m: number
    apparent_temperature: number
    precipitation: number
    wind_speed_10m: number
    visibility: number
}

interface OpenMeteoResponse {
    timezone: string
    hourly: {
        time: string[]
        cloudcover: number[]
        cloud_cover_low: number[]
        cloud_cover_mid: number[]
        cloud_cover_high: number[]
        rain: number[]
        dewpoint_2m: number[]
        relative_humidity_2m: number[]
        temperature_2m: number[]
        apparent_temperature: number[]
        precipitation: number[]
        wind_speed_10m: number[]
        visibility: number[]
    }
}

const latitude = computed(() =>
    settings.selectedSiteIdx === undefined
        ? -22.935
        : settings.sites[settings.selectedSiteIdx].latitude
)

const longitude = computed(() =>
    settings.selectedSiteIdx === undefined
        ? -68.193
        : settings.sites[settings.selectedSiteIdx].longitude
)

const siteName = computed(() =>
    settings.selectedSiteIdx === undefined
        ? 'San Pedro de Atacama'
        : settings.sites[settings.selectedSiteIdx].name
)

const forecast = ref<WeatherEntry[]>([])
const loading = ref(true)
const error = ref<string>('')

const weatherApiUrl = computed(() =>
    `https://api.open-meteo.com/v1/forecast?latitude=${latitude.value}&longitude=${longitude.value}&hourly=cloudcover,cloud_cover_low,cloud_cover_mid,cloud_cover_high,rain,dewpoint_2m,relative_humidity_2m,temperature_2m,apparent_temperature,precipitation,wind_speed_10m,visibility&timezone=auto&forecast_days=${selectedDays.value}`
)

const groupedForecast = computed(() => {
    const groups: { date: string; entries: WeatherEntry[] }[] = []
    let currentDate = ''

    for (const entry of forecast.value) {
        const local = DateTime.fromISO(entry.time, { zone: timezone.value })
        const dateStr = local.toFormat('yyyy-MM-dd')

        if (dateStr !== currentDate) {
            currentDate = dateStr
            groups.push({ date: dateStr, entries: [] })
        }

        groups[groups.length - 1].entries.push(entry)
    }

    return groups
})

watchEffect(async () => {
    loading.value = true
    error.value = ''

    try {
        const res = await fetch(weatherApiUrl.value)
        if (!res.ok) throw new Error('Network error')

        const data: OpenMeteoResponse = await res.json()

        timezone.value = data.timezone;
        const now = DateTime.now().setZone(data.timezone)

        forecast.value = data.hourly.time.map((_, i) => ({
            time: data.hourly.time[i],
            cloudcover: data.hourly.cloudcover[i],
            cloudcoverlow: data.hourly.cloud_cover_low[i],
            cloudcovermid: data.hourly.cloud_cover_mid[i],
            cloudcoverhigh: data.hourly.cloud_cover_high[i],
            rain: data.hourly.rain[i],
            dewpoint_2m: data.hourly.dewpoint_2m[i],
            relative_humidity_2m: data.hourly.relative_humidity_2m[i],
            temperature_2m: data.hourly.temperature_2m[i],
            apparent_temperature: data.hourly.apparent_temperature[i],
            precipitation: data.hourly.precipitation[i],
            wind_speed_10m: data.hourly.wind_speed_10m[i],
            visibility: data.hourly.visibility[i]
        }))
            .filter(entry => DateTime.fromISO(entry.time, { zone: data.timezone }) >= now.startOf('hour'))
    } catch (e) {
        error.value = e instanceof Error ? e.message : 'Unknown error occurred'
    } finally {
        loading.value = false
    }
})

function formatHour(isoString: string): string {
    return DateTime.fromISO(isoString, { zone: timezone.value }).toFormat('HH:00')
}

function formatDate(dateStr: string): string {
    return DateTime.fromISO(dateStr).toFormat('ccc, MMM d')
}

const timezone = ref<string>('')

const isCurrentLocalHour = (forecastTime: string) => {
    const forecast = DateTime.fromISO(forecastTime, { zone: timezone.value })
    const now = DateTime.now().setZone(timezone.value)

    return (
        forecast.hasSame(now, 'hour') &&
        forecast.hasSame(now, 'day')
    )
}

function cloudBackground(clouds: number): string {
    const clamp = (v: number) => Math.min(100, Math.max(0, v));
    const p = clamp(clouds) / 100;

    // Interpolate between regular blue and white-blue
    const r = Math.round(0 + p * (173 - 0));   // from 0 to 173 (light blue)
    const g = Math.round(123 + p * (216 - 123)); // from 123 to 216
    const b = Math.round(255 + p * (255 - 255)); // stay at 255

    return `rgb(${r}, ${g}, ${b})`;
}
</script>

<style scoped>
.forecast-day-group {
    display: flex;
    flex-direction: row;
    margin-right: 8px;
}

.forecast-day-header {
    writing-mode: vertical-lr;
    text-orientation: mixed;
    font-weight: bold;
    padding: 4px;
    margin-right: 4px;
    background-color: rgba(255, 255, 255, 0.05);
    border-right: 1px solid rgba(255, 255, 255, 0.1);
}

.forecast-scroll {
    display: flex;
    gap: 2px;
    flex-wrap: nowrap;
    scroll-behavior: smooth;
    overflow-x: auto;
    border-left: 1px solid rgba(255, 255, 255, 0.1);
    position: relative;
    padding-left: 8px;
    background: linear-gradient(to right, rgba(0, 0, 0, 0.35) 0px, transparent 20px);
}

.forecast-hour {
    width: 80px;
    flex: 0 0 auto;
    background-color: rgba(255, 255, 255, 0.03);
    border-right: 1px solid rgba(255, 255, 255, 0.07);
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    padding: 1px;
    transition: background-color 0.2s;
}

.forecast-hour:hover {
    background-color: rgba(255, 255, 255, 0.39);
}

.even-hour {
    background-color: rgba(255, 255, 255, 0.10);
}

.fixed-labels .v-icon {
    color: rgba(255, 255, 255, 0.7);
}

.fixed-labels {
    min-width: 120px;
    padding-right: 8px;
    border-right: 1px solid rgba(255, 255, 255, 0.15);

}

.current-hour {
    background-color: rgba(255, 255, 255, 0.35) !important;
}

.sun-elevation-row,
.moon-elevation-row {
    display: flex;
    flex-direction: row;
    margin-left: 124px;
    margin-bottom: 2px;
}

.elevation-cell {
    flex: 0 0 80px;
    height: 18px;
    font-size: 10px;
    text-align: center;
    line-height: 18px;
    color: white;
    font-weight: bold;
}
</style>