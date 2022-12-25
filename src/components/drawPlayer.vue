<template>    
  <div class="row">
    <div class="col-3 q-pa-md">
      <h3 class="step-title">Step 1</h3>
      <div class="card">
        <input id="player-input" v-model="name" @keyup.enter="addPlayer()" placeholder="Enter a name..." />
        <button type="button" @click="addPlayer()">Add</button>
      </div>
      <div class="q-pa-md">
        <div class="q-gutter-xs column" style="max-width: 320px" :class="{ 'truncate-chip-labels': false}">
          <div v-for="(name, index) in players" class="col" >
            <q-chip
              removable
              icon="face"
              color="secondary"
              text-color="white"
              class="chip-player"
              :title="name"
              :label="name"
              :key="index"
              @remove="removePlayer(index)"
            />
          </div>
        </div>
      </div>
      <p>{{ message }}</p>
    </div>
    <div class="col-8 q-pa-md">
      <h3 class="step-title">Step 2</h3>
      <div class="">
        <q-carousel
          v-model="slide"
          transition-prev="slide-right"
          transition-next="slide-left"
          swipeable
          animated
          control-color="secondary"
          control-type="flat"
          navigation
          padding=""
          arrows
          class=" shadow-1 rounded-borders"
        >
          <q-carousel-slide :name="1" class="column no-wrap">
            <div vertical class="row fit justfy-start items-center q-gutter-xs q-col-gutter no-wrap">
              <q-card  class="name-card-l bg-secondary text-white q-py-md">
                <q-card-section v-if="drawedPlayers.length > 0">
                  <div class="text-h2">{{ drawedPlayers[0] }}</div>
                </q-card-section>
                <q-card-section v-if="drawedPlayers.length === 0">
                  <div class="text-h2">Merry Christmas</div>
                </q-card-section>
              </q-card>
            </div>
          </q-carousel-slide>
          <q-carousel-slide 
            v-for="(_, index) in drawedPlayers"
            :name="index+2" class="column no-wrap">
            <div class="row fit justfy-start items-center q-gutter-xs q-col-gutter q-mt-md no-wrap">
              <q-card class="bg-secondary name-card q-py-md">
                <q-card-section class="bg-secondary text-white">
                  <div class="text-h3">{{ drawedPlayers[index] }}</div>
                </q-card-section>
              </q-card>
              <q-icon name="redeem" color="secondary" size="4rem" style="margin-left: 20px;"/>
              <q-icon name="east" color="secondary" size="4rem" style="margin-right: 20px;" />
              <q-card v-if="index === (drawedPlayers.length-1)" class="bg-secondary name-card q-py-md">
                <q-card-section  class="text-white">
                  <div class="text-h3">{{ drawedPlayers[0] }}</div>
                </q-card-section>
              </q-card>
              <q-card v-if="index < (drawedPlayers.length-1)" class="bg-secondary name-card q-py-md">
                <q-card-section class="text-white">
                  <div class="text-h3">{{ drawedPlayers[index+1] }}</div>
                </q-card-section>
              </q-card>
            </div>
          </q-carousel-slide>
        </q-carousel>
      </div>
      <div class="q-pa-md q-gutter-sm">
        <q-btn :disable="drawID > drawedPlayers.length - 2 ? true : false" align="between" class="btn-fixed-width" @click="run" label="start shuffle" icon="shuffle" color="secondary"/>
        <q-btn :disable="drawID > drawedPlayers.length - 2 ? true : false" align="between" class="btn-fixed-width" @click="draw" label="draw player" icon="card_giftcard" color="secondary"/>
      </div>
    </div>
  </div>

  <!-- alert -->
    <q-dialog v-model="alert" seamless position="top">
      <q-card class="bg-primary text-white" style="width: 260px">
        <!-- <q-card-section>
          <div class="text-h6">Alert</div>
        </q-card-section> -->

        <q-card-actions align="between">
          <div class="q-px-md">{{ alertMessage }}</div>
          <q-btn flat size="sm" label="OK" color="white" v-close-popup />
        </q-card-actions>
      </q-card>
    </q-dialog>
</template>

<script setup>
import { ref, onMounted, onBeforeUpdate, reactive } from 'vue'
import { morph } from 'quasar'
import { invoke } from "@tauri-apps/api/tauri";

const message = ref("");
const name = ref("");
const players = ref([])
const drawedPlayers = ref([])
const drawID = ref(0)
const alert = ref(false)
const alertMessage = ref("")
let swapLoop = false
let runFlag = false
const slide = ref(1)
let candidates = []
let order = []
const btnDisable = ref(false)

async function getPlayers() {
  players.value = await invoke("get_players")
  for (let p of players.value) {
    drawedPlayers.value.push("who?")
    candidates.push(p)
  }
}

async function addPlayer() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  if (players.value.includes(name.value)) {
    alert.value = true
    alertMessage.value = "player "+name.value+" already in the list."
    return
  }
  players.value.push(name.value)
  drawedPlayers.value.push("who?")
  candidates.push(name.value)
  await invoke("add_player", {name: name.value})
}

async function removePlayer(index) {
  await invoke("remove_player", {name: players.value[index]})
  players.value.splice(index, 1)
  drawedPlayers.value.pop()
  candidates.splice(index, 1)
  // console.log(candidates)
}

async function run() {
  if (drawID.value === drawedPlayers.value.length-1) { 
    return
   }
  console.log(drawID.value)
  runFlag = true
  console.log(drawedPlayers.value)
  // last 2 players to decide by 1 draw
  if (drawID.value === drawedPlayers.value.length-2) {
    while(runFlag === true) {
      order = await invoke("random_order", { maxIndex: candidates.length })  
      drawedPlayers.value[drawID.value] = candidates[order[0]]
      drawedPlayers.value[drawID.value+1] = candidates[order[1]]
      console.log("while : " + drawedPlayers.value)
    }
    alert.value = true
    alertMessage.value = "Raffle Done."
    btnDisable.value = true
    console.log("drawed last 2")
  } 
  // others players to draw excpet last 2
  else {
    while(runFlag === true) {
      order = await invoke("random_order", { maxIndex: candidates.length })  
      drawedPlayers.value[drawID.value] = candidates[order[0]]
      console.log("while : " + drawedPlayers.value)
    }
  }
  drawID.value += 1
  candidates.splice(order[0], 1)
    // console.log(player)
  console.log("after while:"+drawedPlayers.value)
}

async function draw() {
  if (runFlag === true) {
    // drawID.value = (drawID.value + 1) % drawedPlayers.value.length
    runFlag = false
    console.log(candidates)
  }
}

async function randomOrderStart() {
  swapLoop = true
  if (players.value.length <= 1) {
    alert.value = true
    alertMessage.value = "Please add at least 2 players."
    return
  }
  while(swapLoop === true) {
    let order = await invoke("random_order", { maxIndex: players.value.length })
    let tmpPlayers = [].concat(players.value)
    for (let i in order) {
      players.value[order[i]] = tmpPlayers[i]
    }
  }
}

async function randomOrderStop() {
  swapLoop = false
}

onMounted(() => {
  let r = getPlayers()
  console.log(r)
})
</script>

<style lang="sass" scoped>
.btn-fixed-width
  width: 180px
.chip-player
  min-width: 150px
.step-title
  color: white
.name-card
  width: 230px
  text-align: center
.name-card-l
  min-width: 300px
</style>