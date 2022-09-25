import { DateTime } from "luxon"
import coordinates from 'coordinate-parser'


export interface Geolocation {
    description?: string,
    lat: number,
    lon: number,
    url: URL,
    notes?: string,
    geolocator?: string,
    datetime?: DateTime | string,
    provisional?: boolean
}

export interface Geowire extends Geolocation {
    datetime: string
}

export interface glc extends Geolocation {
    id: number,
    uuid: string,
    datetime?: DateTime
}

export const jsonToGlc = function (r: Array<any>): Array<glc> {
    return r.map((c): glc => { return { ...c, datetime: DateTime.fromISO(c.datetime, { zone: 'utc' }), url: new URL(c.url) } })
        .sort((a, b) => (b.datetime?.toMillis() ?? 0) - (a.datetime?.toMillis() ?? 0))
}

export const clampDecimals = (n: number) => {
    return n.toFixed(5)
}

export const getCoords = function (cs: string) {
    var coords: coordinates
    try {
        coords = new coordinates(cs)
    } catch (error) {
        return null
    }

    return [coords.getLatitude(), coords.getLongitude()]
}

export const tweetIdUrl = (url: URL) => {
    const rex = /\/(\d*$)/;
    const match = url.pathname.match(rex)
    return match ? match[1] : ""
}

export const gmapsUrl = (lat: number, lon: number) => {
    const slat = lat.toString()
    const slon = lon.toString()
    return "https://www.google.com/maps?ll=" + slat + "," + slon + "&q=" + slat + "," + slon + "&hl=en&t=m&z=15"
}

export const geohackUrl = (lat: number, lon: number) => {
    const slat = lat.toString()
    const slon = lon.toString()
    return "https://geohack.toolforge.org/geohack.php?params="+lat+";"+lon
}