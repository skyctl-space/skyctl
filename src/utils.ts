export const formatCoordinate = (value: number, positive: string, negative: string) => {
    const cardinal = value >= 0 ? positive : negative
    const absValue = Math.abs(value)
    const degrees = Math.floor(absValue)
    const minutes = Math.floor((absValue - degrees) * 60)
    const seconds = Math.floor(((absValue - degrees) * 60 - minutes) * 60)
    return `${degrees}° ${minutes}' ${seconds}" ${cardinal}`
}


export const formatLatitude = (value: number) => {
    return formatCoordinate(value, 'N', 'S')
}

export const formatLongitude = (value: number) => {
    return formatCoordinate(value, 'E', 'W')
}


export function culturalNameToList(cn: any) {
    const res = []

    const formatNative = function (_cn: any) {
        if (cn.name_native && cn.name_pronounce) {
            return cn.name_native + ', <i>' + cn.name_pronounce + '</i>'
        }
        if (cn.name_native) {
            return cn.name_native
        }
        if (cn.name_pronounce) {
            return cn.name_pronounce
        }
    }

    const nativeName = formatNative(cn)
    if (cn.user_prefer_native && nativeName) {
        res.push(nativeName)
    }
    if (cn.name_translated) {
        res.push(cn.name_translated)
    }
    if (!cn.user_prefer_native && nativeName) {
        res.push(nativeName)
    }
    return res
}

export function namesForSkySource(stel: any, ss: any, flags: number) {
    // Return a list of cleaned up names
    if (!ss || !ss.names) {
        return []
    }
    if (!flags) flags = 10
    let res = <string[] | never>[]
    if (ss.culturalNames) {
        for (const i in ss.culturalNames) {
            res = res.concat(culturalNameToList(ss.culturalNames[i]))
        }
    }
    res = res.concat(ss.names.map((n: any) => stel.designationCleanup(n, flags)))
    // Remove duplicates, this can happen between * and V* catalogs
    res = res.filter(function (v, i) { return res.indexOf(v) === i })
    res = res.filter(function (v, _i) { return !v.startsWith('CON ') })
    return res
}


export function iconForSkySourceTypes(skySourceTypes: string[]) {
    const iconForType: Record<string, string> = {
        // Stars
        'Pec?': 'star',
        '**?': 'double_star',
        '**': 'double_star',
        'V*': 'variable_star',
        'V*?': 'variable_star',
        '*': 'star',

        // Candidates
        'As?': 'group_of_stars',
        'SC?': 'group_of_galaxies',
        'Gr?': 'group_of_galaxies',
        'C?G': 'group_of_galaxies',
        'G?': 'galaxy',

        // Multiple objects
        reg: 'region_defined_in_the_sky',
        SCG: 'group_of_galaxies',
        ClG: 'group_of_galaxies',
        GrG: 'group_of_galaxies',
        IG: 'interacting_galaxy',
        PaG: 'pair_of_galaxies',
        'C?*': 'open_galactic_cluster',
        'Gl?': 'globular_cluster',
        GlC: 'globular_cluster',
        OpC: 'open_galactic_cluster',
        'Cl*': 'open_galactic_cluster',
        'As*': 'group_of_stars',
        mul: 'multiple_objects',

        // Interstellar matter
        'PN?': 'planetary_nebula',
        PN: 'planetary_nebula',
        SNR: 'planetary_nebula',
        'SR?': 'planetary_nebula',
        ISM: 'interstellar_matter',

        // Galaxies
        PoG: 'part_of_galaxy',
        QSO: 'quasar',
        G: 'galaxy',

        dso: 'deep_sky',

        // Solar System
        Asa: 'artificial_satellite',
        Moo: 'moon',
        Sun: 'sun',
        Pla: 'planet',
        DPl: 'planet',
        Com: 'comet',
        MPl: 'minor_planet',
        SSO: 'minor_planet',

        Con: 'constellation'
    }
    for (const type of skySourceTypes) {
        if (type in iconForType) {
            return '/images/svg/target_types/' + iconForType[type] + '.svg'
        }
    }
    return '/images/svg/target_types/unknown.svg'
}

export function iconForSkySource(skySource: any) {
    return iconForSkySourceTypes(skySource.types)
}


const galTab: { [key: string]: string } = {
    E: 'Elliptical',
    SB: 'Barred Spiral',
    SAB: 'Intermediate Spiral',
    SA: 'Spiral',
    S0: 'Lenticular',
    S: 'Spiral',
    Im: 'Irregular',
    dSph: 'Dwarf Spheroidal',
    dE: 'Dwarf Elliptical',
};

export function nameForGalaxyMorpho(morp: string): string {
    return galTab[morp] || 'Unknown';
}

export function nameForSkySourceType(stel: any, otype: any) {
    const res = stel.otypeToStr(otype)
    return res || 'Unknown Type'
}

export function cleanupOneSkySourceName(stel: any, name: string, flags: number) {
    flags = flags || 4
    return stel.designationCleanup(name, flags)
}

export function nameForSkySource(stel: any, skySource: any) {
    if (!skySource || !skySource.names) {
        return '?'
    }
    return cleanupOneSkySourceName(stel, skySource.names[0], 0)
}

const API_SERVER = 'https://api.noctuasky.com'

export function lookupSkySourceByName(name: string): Promise<any> {
    return fetch(API_SERVER + '/api/v1/skysources/name/' + name)
        .then(function (response) {
            if (!response.ok) {
                throw response.body
            }
            return response.json()
        }, err => {
            throw err.response.body
        })
    // throw new Error('SkySource API not available')
}

export function querySkySources(str: string, limit: number): Promise<any> {
    if (!limit) {
        limit = 10
    }
    return fetch(API_SERVER + '/api/v1/skysources/?q=' + str + '&limit=' + limit)
        .then(function (response) {
            if (!response.ok) {
                throw response.body
            }
            return response.json()
        }, err => {
            throw err.response.body
        })
}

export function getSkySourceSummaryFromWikipedia(stel: any, ss: any) {
    let title
    if (ss.model === 'jpl_sso') {
        title = cleanupOneSkySourceName(stel, ss.names[0], 0).toLowerCase()
        if (['mercury', 'venus', 'earth', 'mars', 'jupiter', 'saturn', 'neptune', 'pluto'].indexOf(title) > -1) {
            title = title + '_(planet)'
        }
        if (ss.types[0] === 'Moo') {
            title = title + '_(moon)'
        }
    }
    if (ss.model === 'mpc_asteroid') {
        title = cleanupOneSkySourceName(stel, ss.names[0], 0).toLowerCase()
    }
    if (ss.model === 'constellation') {
        title = cleanupOneSkySourceName(stel, ss.names[0], 0).toLowerCase() + '_(constellation)'
    }
    if (ss.model === 'dso') {
        for (const i in ss.names) {
            if (ss.names[i].startsWith('M ')) {
                title = 'Messier_' + ss.names[i].substr(2)
                break
            }
            if (ss.names[i].startsWith('NGC ')) {
                title = ss.names[i]
                break
            }
            if (ss.names[i].startsWith('IC ')) {
                title = ss.names[i]
                break
            }
        }
    }
    if (ss.model === 'star') {
        for (const i in ss.names) {
            if (ss.names[i].startsWith('* ')) {
                title = cleanupOneSkySourceName(stel, ss.names[i], 0)
                break
            }
        }
    }
    if (!title) return Promise.reject(new Error("Can't find wikipedia compatible name"))

    return fetch('https://en.wikipedia.org/w/api.php?action=query&redirects&prop=extracts&exintro&exlimit=1&exchars=300&format=json&origin=*&titles=' + title,
        { headers: { 'Content-Type': 'application/json; charset=UTF-8' } })
        .then(response => {
            return response.json()
        })
}

export function sweObj2SkySource(obj: any) {
    const names = obj.designations()

    if (!names || !names.length) {
        throw new Error("Can't find object without names")
    }

    // Several artifical satellites share the same common name, so we use
    // the unambiguous NORAD number instead
    for (const j in names) {
        if (names[j].startsWith('NORAD ')) {
            const tmpName = names[0]
            names[0] = names[j]
            names[j] = tmpName
        }
    }

    const printErr = function (n: any) {
        console.log("Couldn't find online skysource data for name: " + n)

        const ss = obj.jsonData
        if (!ss.model_data) {
            ss.model_data = {}
        }
        // Names fixup
        let i
        for (i in ss.names) {
            if (ss.names[i].startsWith('GAIA')) {
                ss.names[i] = ss.names[i].replace(/^GAIA /, 'Gaia DR2 ')
            }
        }
        ss.culturalNames = obj.culturalDesignations()
        return ss
    }

    return lookupSkySourceByName(names[0]).then((res: any) => {
        return res
    }, () => {
        if (names.length === 1) return printErr(names)
        return lookupSkySourceByName(names[1]).then((res: any) => {
            return res
        }, () => {
            if (names.length === 2) return printErr(names)
            return lookupSkySourceByName(names[2]).then((res: any) => {
                return res
            }, () => {
                return printErr(names[2])
            })
        })
    }).then((res: any) => {
        res.culturalNames = obj.culturalDesignations()
        return res
    })
}

export const astroConstants = {
    // Light time for 1 au in s
    ERFA_AULT: 499.004782,
    // Seconds per day
    ERFA_DAYSEC: 86400.0,
    // Days per Julian year
    ERFA_DJY: 365.25,
    // Astronomical unit in m
    ERFA_DAU: 149597870000,
};

export function skySource2SweObj(stel: any, ss: any) {
    if (!ss || !ss.model) {
        return undefined
    }
    let obj
    if (ss.model === 'tle_satellite') {
        const id = 'NORAD ' + ss.model_data.norad_number
        obj = stel.getObj(id)
    } else if (ss.model === 'constellation' && ss.model_data.iau_abbreviation) {
        const id = 'CON western ' + ss.model_data.iau_abbreviation
        obj = stel.getObj(id)
    }
    if (!obj) {
        obj = stel.getObj(ss.names[0])
    }
    if (!obj && ss.names[0].startsWith('Gaia DR2 ')) {
        const gname = ss.names[0].replace(/^Gaia DR2 /, 'GAIA ')
        obj = stel.getObj(gname)
    }
    if (obj === null) return undefined
    return obj
}

export function setSweObjAsSelection(stel: any, obj: any) {
    stel.core.selection = obj
    stel.pointAndLock(obj)
}