<script setup lang="ts">
import { ref } from 'vue'


//shows full url on hero mouseover
var hover = ref(false);

</script>

<template>
  <header>
    <div>
      <span @mouseover="hover = true" @mouseleave="hover = false">
        <span class="biglogo">ukrgeo</span>
        <span v-if="hover" class="biglogo">.charliegillespie.com</span>
      </span>
    </div>
    <div>the ukraine invasion collaborative geolocation server</div>
  </header>

</template>

<style>
header {
  /* padding-left: 10px; */
  padding-top: 10px;
  padding-bottom: 10px;
}

.biglogo {
  font-weight: bold;
  font-size: 28pt;
}
</style>
