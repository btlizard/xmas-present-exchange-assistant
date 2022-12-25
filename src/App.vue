<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import drawPlayer from "./components/drawPlayer.vue"
</script>

<template>
  <div class=" bg-primary bg">
    <q-layout view="lhh LpR lff" container >
      <div data-tauri-drag-region class="bg-primary">
        <q-header class="navbar left">
          <!-- <q-btn-group class="q-pa-md" >
            <q-btn round label="about" color="mgray" size="xs" />
          </q-btn-group> -->
        </q-header>
      </div>
      <q-page-container class="container">
        <drawPlayer/>
      </q-page-container>
    </q-layout>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.bg {
  height: 700px;
}
.navbar {
  height: 30px;
}
</style>
