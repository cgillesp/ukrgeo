<script setup lang="ts">
import { ref, type Ref } from 'vue'
import "normalize.css"
import Tweet from './components/vue-tweet.vue'
import type { glc, Geowire } from "./utility"
import { jsonToGlc, clampDecimals, tweetIdUrl } from './utility';
import Header from './components/header.vue'
import LocationsTable from './components/LocationsTable.vue'


// Every location we know about
const locations: Ref<glc[]> = ref([])

// Gets provisional entries from the api
const getLocs = () => fetch("/api/provisional.json").then((r) => r.json()).then((r) => locations.value = jsonToGlc(r))

getLocs()


var expandedid = ref("")


</script>

<template>
  <Header> </Header>

  <LocationsTable :locations="locations"></LocationsTable>

</template>

<style>
@import "./assets/base.css";

/* tr > span > * + * {
  padding-left: 1em;
} */

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
