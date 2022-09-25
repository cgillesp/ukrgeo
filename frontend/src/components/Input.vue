<script setup lang="ts">
import { ref, onMounted, type Ref, watchEffect } from 'vue'
import * as L from 'leaflet'
import "leaflet/dist/leaflet.css";
import "normalize.css"
import coordinates from 'coordinate-parser'
import { DateTime } from "luxon"
import Tweet from '../components/vue-tweet.vue'
import type { glc, Geowire } from "../utility"
import { jsonToGlc, clampDecimals, tweetIdUrl } from '../utility';
import Header from '../components/header.vue'
// @ts-ignore
import { LMap, LTileLayer, LMarker, LPopup } from "@vue-leaflet/vue-leaflet"
import LocationsTable from '../components/LocationsTable.vue'


const props = defineProps<{
    locations: glc[]
    getLocs: () => void
    map?: Ref<LMarker>
    edit?: glc
    center: Ref<Number[]>
}>()

enum rqstatus {
    success,
    failure
}

var status: Ref<rqstatus | undefined> = ref()

var inputStarted = ref(false)

var statusmessage = ref("")

// Form entry values
var coordsin = ref("")
var urlin = ref("")
var descin = ref("")
var notesin = ref("")
var timein = ref("")
var datein = ref(DateTime.utc().toISODate())
var geolocatorin = ref("")

if (props.edit) {
    coordsin.value = props.edit.lat.toString() + " " + props.edit.lon.toString()
    urlin.value = props.edit.url.toString()
    descin.value = props.edit.description ?? ""
    notesin.value = props.edit.notes ?? ""
    timein.value = props.edit.datetime?.toLocaleString(DateTime.TIME_24_SIMPLE) ?? ""
    datein.value = props.edit.datetime?.toISODate() ?? ""
    geolocatorin.value = props.edit.geolocator ?? ""
}


//Tracks validity of form
var invalidEntry = ref(false)
var isValidated = ref(false)

//Tracks the 
var markers = new Map<number, L.Marker>();
var tempmarker: L.Marker


const validateInput = () => {
    var coords: coordinates
    var url: URL
    try {
        coords = new coordinates(coordsin.value)
        url = new URL(urlin.value)
    } catch (error) {
        isValidated.value = false
        invalidEntry.value = true
        return
    }

    try {
        const time = DateTime.fromISO(timein.value, { zone: 'utc' })
        const dt = DateTime.fromISO(datein.value, { zone: 'utc' })
            .plus({ hours: time.hour, minutes: time.minute })
    } catch (error) {
        isValidated.value = false
        invalidEntry.value = true
        return
    }

    console.log(props.map)
    if (props.map) {
        // @ts-ignore
        let leaflet = props.map.leafletObject
        invalidEntry.value = false
        if (tempmarker) {
            tempmarker.removeFrom(leaflet)
        }
        tempmarker = L.marker([coords.getLatitude(), coords.getLongitude()], {}).addTo(leaflet)
        leaflet.flyTo(tempmarker.getLatLng(), 13, { animate: true, duration: 0.3 })
    }
    isValidated.value = true
}

const submitInput = () => {

    if (props.map && tempmarker) {
        // @ts-ignore
        tempmarker.removeFrom(props.map.leafletObject)

        props.center.value = [];
    }

    var coords: coordinates
    var url: URL
    try {
        coords = new coordinates(coordsin.value)
        url = new URL(urlin.value)
    } catch (error) {
        invalidEntry.value = true
        isValidated.value = false
        return
    }

    const time = DateTime.fromISO(timein.value, { zone: 'utc' })
    const dt = DateTime.fromISO(datein.value, { zone: 'utc' })
        .plus({ hours: time.hour, minutes: time.minute })
    // dt.hour
    const newgl: Geowire = {
        lat: coords.getLatitude(),
        lon: coords.getLongitude(),
        url: url,
        description: descin.value,
        notes: notesin.value,
        geolocator: geolocatorin.value,
        datetime: dt.toISO().slice(0, -1)
    }

    fetch("/api/location",

        {
            method: 'POST', // *GET, POST, PUT, DELETE, etc.
            // mode: 'cors', // no-cors, *cors, same-origin
            // cache: 'no-cache', // *default, no-cache, reload, force-cache, only-if-cached
            // credentials: 'same-origin', // include, *same-origin, omit
            headers: {
                'Content-Type': 'application/json'
            },
            // redirect: 'follow', // manual, *follow, error
            // referrerPolicy: 'no-referrer', // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
            body: JSON.stringify(newgl) // body data type must match "Content-Type" header

        }


    ).then((r) => {
        status.value = r.ok ? rqstatus.success : rqstatus.failure
        if (r.ok) {


            coordsin.value = ""
            urlin.value = ""
            descin.value = ""
            notesin.value = ""
            timein.value = ""
            datein.value = DateTime.utc().toISODate()
            geolocatorin.value = ""


            isValidated.value = false

        }

    }).then(() => props.getLocs())


}


const validateOrSubmit = () => { if (isValidated.value) { submitInput() } else { validateInput() } }

</script>

<template>
    <div>contribute a geolocation!</div>

    <div>
        <div>
            <form v-on:submit.prevent="validateOrSubmit">
                <div class="successfulEntry" v-if="status == rqstatus.success">Submitted! Should be verified soon :)
                </div>
                <div class="invalidEntry" v-if="status == rqstatus.failure">Error prossesing your submission</div>
                <div class="invalidentry" v-if="invalidEntry">Entry invalid</div>
                <input class="geninput" v-on:focus="inputStarted = true" v-model="coordsin" placeholder="coordinates"
                    v-on:change="isValidated = false" />
                <input class="geninput" v-on:focus="inputStarted = true" v-model="urlin" placeholder="url"
                    v-on:change="isValidated = false" />
                <div v-if="inputStarted">
                    <div>
                        <input v-model="descin" class="geninput" id="descinput" placeholder="description" />
                    </div>
                    <div>
                        <textarea id="notesarea" v-model="notesin" placeholder="notes"></textarea>
                    </div>
                    <div>
                        <input type="time" v-model="timein" />
                        <input type="date" v-model="datein" />
                        approx. time recorded (utc)
                    </div>
                    <div>
                        <input v-model="geolocatorin" class="geninput" placeholder="geolocated by (url/twitter @)" />
                    </div>

                    <div>
                        URL must be a link to a public post on a major social media platform (e.g. Twitter, Facebook,
                        Telegram).
                    </div>
                </div>
                <button v-if="!isValidated" v-on:click="validateInput">validate</button>
                <input v-if="isValidated" type="submit" id="submitbutton" value="submit" />
            </form>
        </div>
    </div>
</template>

<style>
input,
textarea {
    border-radius: 0;
    border: 1px solid;
    margin: 0px 5px 5px 0px;
    font-size: 11pt;
}

textarea {
    margin-bottom: 1px;
}

.geninput {
    width: 270px;
}

.provisional {
    color: #777;
}

#submitbutton {
    background-color: hsl(108, 66%, 45%);
}

button,
#submitbutton {
    border-radius: 0;
    border: 1px solid;
    margin: 0 5px 0 0px;
    background-color: black;
    color: #eee;
}

.invalidentry {
    color: red;
}

.successfulEntry {
    color: hsl(108, 66%, 45%);
}

#notesarea,
#descinput {
    width: 550px;
}
</style>
