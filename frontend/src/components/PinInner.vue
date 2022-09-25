<script setup lang="ts">
import { ref } from 'vue'
import { jsonToGlc, clampDecimals, tweetIdUrl, gmapsUrl, geohackUrl } from '../utility';
import type { glc } from '../utility';
import Tweet from './vue-tweet.vue'



defineProps<{
    location: glc
}>()

</script>
    
<template>
    <span class="maplinks">
        <a :href="gmapsUrl(location.lat, location.lon)">Google Maps</a>
        <a :href="geohackUrl(location.lat, location.lon)">Geohack</a></span>
    <div class="tweetcontainer" v-if="location.url.hostname == 'twitter.com'">
        <Tweet :tweet-id="tweetIdUrl(location.url)"></Tweet>
    </div>
    <div>
        {{location.description}}
    </div>
    <div>
        <a :href="location.url.toString()">Link</a>
    </div>
</template>
    
<style>
.tweetcontainer {
    min-width: 250px;
}

.maplinks>a {
    margin-right: 4px;
}
</style>
    