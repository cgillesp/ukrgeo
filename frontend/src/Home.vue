<script setup lang="ts">
import { ref, onMounted, type Ref, watchEffect, createApp } from 'vue'
// import * as L from 'leaflet'
import "leaflet/dist/leaflet.css";
import "normalize.css"
import coordinates from 'coordinate-parser'
import { DateTime } from "luxon"
import Tweet from './components/vue-tweet.vue'
import type { glc, Geowire } from "./utility"
import { jsonToGlc, clampDecimals, tweetIdUrl } from './utility';
import Header from './components/header.vue'
import LocationsTable from './components/LocationsTable.vue'
import InputBlock from './components/Input.vue'
// @ts-ignore
import { LMap, LTileLayer, LMarker, LPopup } from "@vue-leaflet/vue-leaflet"
import PinInner from "./components/PinInner.vue"


//Tracks the 
var markers = new Map<number, L.Marker>();

// Every location we know about
const locations: Ref<glc[]> = ref([])

// Gets latest from the api
const getLocs = () => fetch("/api/recent.json")
  .then((r) => r.json()).then((r) => locations.value = jsonToGlc(r))

getLocs()

const map = ref(undefined)

const center: Ref<Number[]> = ref([50.4504, 30.5225])

</script>

<template>
  <div id="atf">
    <Header></Header>

    <l-map ref="map" :center="center" :zoom="6" :maxZoom="13">
      <l-tile-layer url="https://stamen-tiles.a.ssl.fastly.net/terrain/{z}/{x}/{y}.png" layer-type="base"
        name="Stamen" />

      <template v-for="location in locations">
        <l-marker :lat-lng="[location.lat, location.lon]">
          <l-popup>
            <PinInner :location="location" />
          </l-popup>
        </l-marker>
      </template>
    </l-map>


  </div>

  <InputBlock :locations="locations" :get-locs="getLocs" :map="map" :center="ref(center)"></InputBlock>
  <LocationsTable :locations="locations"></LocationsTable>
</template>

<style>
@import "./assets/base.css";

button {
  border-radius: 0;
  border: 1px solid;
  margin: 0 5px 0 0px;
  background-color: black;
  color: #eee;
}

@media screen and (max-width: 1280px) {
  header {
    padding-left: 10px;
  }
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
  width: 100%;
  /* padding: 2rem; */
  font-weight: normal;
}
</style>
