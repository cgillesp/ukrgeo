<script setup lang="ts">
import { ref, type Ref } from 'vue'
import "normalize.css"
import Tweet from './vue-tweet.vue'
import type { glc, Geowire } from "../utility"
import { jsonToGlc, clampDecimals, tweetIdUrl, gmapsUrl, geohackUrl } from '../utility';
import Header from './header.vue'

defineProps<{
    locations: glc[]
}>()

var expandedid = ref("")


</script>

<template>
    <div id="eventlist">
        <table id="eventstable">
            <tr>
                <td>Time (Approx)</td>
                <td>Lat</td>
                <td>Lon</td>
                <td>Url</td>
                <td>Desc</td>
                <td>Geolocator</td>
            </tr>
            <template v-for="location in locations">
                <tr :class="location.provisional || location.provisional == undefined ? 'provisional' : null">
                    <td>{{ location.datetime?.toISODate() }} {{ location.datetime?.hour }}Z</td>
                    <td>{{ clampDecimals(location.lat) }}</td>
                    <td>{{ clampDecimals(location.lon) }}</td>
                    <td class="urlcell">
                        <a :href="location.url.origin + location.url.pathname" target="_blank"
                            rel="noopener noreferrer">{{ location.url.origin + location.url.pathname }}</a>
                    </td>
                    <td>{{ location.description }}</td>
                    <td class="urlcell">{{ location.geolocator }}</td>
                    <td>
                        <button v-if="location.uuid != expandedid"
                            v-on:click="expandedid = location.uuid">Expand</button>
                        <button v-if="location.uuid == expandedid" v-on:click="expandedid = ''">Close</button>
                    </td>
                </tr>
                <tr v-if="location.uuid == expandedid">
                    <td colspan="6">
                        <div>
                            <span class="maplinks"><a :href="gmapsUrl(location.lat, location.lon)">Google Maps</a>
                                <a :href="geohackUrl(location.lat, location.lon)">Geohack</a></span>
                            <div class="showtweet" v-if="location.url.hostname == 'twitter.com'">
                                <Tweet :tweet-id="tweetIdUrl(location.url)"></Tweet>
                            </div>
                        </div>
                    </td>
                </tr>
            </template>
        </table>
    </div>
</template>

<style>
@import "../assets/base.css";



.provisional {
    color: blue;
}

#eventlist {
    width: 100%;
}

.urlcell {
    word-break: break-all
}

.maplinks>a {
    margin-right: 1em;
}

.showtweet {
    min-height: 620px;
    max-width: 400px;
}

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

button {
    border-radius: 0;
    border: 1px solid;
    margin: 0 5px 0 0px;
    background-color: black;
    color: #eee;
}

#invalidentry {
    color: red;
}

#notesarea,
#descinput {
    width: 550px;
}

#eventstable {
    table-layout: auto;
    width: 100%;
}

@media screen and (max-width: 1280px) {
    header {
        padding-left: 10px;
    }
}

header {
    /* padding-left: 10px; */
    padding-top: 10px;
    padding-bottom: 10px;
}

.biglogo {
    font-weight: bold;
    font-size: 28pt;
}

#map {
    min-height: 300px;
    min-width: 100px;
    width: 100%;
    flex: 1 0 200px;
}

#atf {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    height: 100vh;
    padding-bottom: 20px;
}

#app {
    max-width: 1280px;
    margin: 0 auto;
    /* padding: 2rem; */
    font-weight: normal;
}
</style>
