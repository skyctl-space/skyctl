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
