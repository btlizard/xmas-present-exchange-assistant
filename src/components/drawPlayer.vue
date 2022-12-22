<template>    
  <div class="row">
    <div class="row-4">
      <h4>Step 1</h4>
      <div class="card">
        <input id="player-input" v-model="name" placeholder="Enter a name..." />
        <button type="button" @click="addPlayer()">Add</button>
      </div>
      <div class="q-pa-md">
        <div class="q-gutter-xs column" style="max-width: 320px" :class="{ 'truncate-chip-labels': false}">
          <div v-for="(name, index) in players" class="col" >
            <q-chip
              removable
              icon="face"
              color="primary"
              text-color="white"
              :title="name"
              :label="name"
              :key="index"
              @click="zoomImage(index)"
              @remove="removePlayer(index)"
            />
          </div>
        </div>
      </div>
      <div class="card">
        <button type="button" @click="drawStart()">开始</button>
        <button type="button" @click="drawStop()">抽取</button>
      </div>
      <p>{{ message }}</p>
    </div>
    <div class="row-8">
      <h4>Step 2</h4>
    </div>
  </div>

  <!-- alert -->
    <q-dialog v-model="alert" position="top">
      <q-card>
        <q-card-section>
          <div class="text-h6">Alert</div>
        </q-card-section>

        <q-card-section class="q-pt-none">
          {{ alertMessage }}
        </q-card-section>

        <q-card-actions align="right">
          <q-btn flat label="OK" color="primary" v-close-popup />
        </q-card-actions>
      </q-card>
    </q-dialog>
</template>

<script setup>
import { ref, onBeforeUpdate, reactive } from 'vue'
import { morph } from 'quasar'
import { invoke } from "@tauri-apps/api/tauri";

const message = ref("");
const name = ref("");
const thumbRef = ref([])
const indexZoomed = ref(void 0)
const players = ref([])
const playerModels = reactive([])
const drawID = ref(0)
const alert = ref(false)
const alertMessage = ref("")
let swapLoop = false

players.value.push('顾毓', "王宁", "井井", "面面")
playerModels.push(true, true, true, true)

async function addPlayer() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  message.value = await invoke("greet", { name: name.value })
  players.value.push(name.value)
}

async function removePlayer(index) {
  let a = players.value
  // console.log(a)
  // console.log(a.splice(1,1))
  // console.log(a)
  // console.log(index)
  // players.value = players.value.splice(index, 1)
  console.log(players.value)
  players.value.splice(index, 1)
  console.log(players.value)
}

async function drawStart() {
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

async function drawStop() {
  swapLoop = false
}

function zoomImage (index) {
  const indexZoomedState = indexZoomed.value
  let cancel = void 0

  indexZoomed.value = void 0

  if (index !== void 0 && index !== indexZoomedState) {
    cancel = morph({
      from: thumbRef.value[ index ].$el,
      onToggle: () => {
        indexZoomed.value = index
      },
      duration: 500,
      onEnd: end => {
        if (end === 'from' && indexZoomed.value === index) {
          indexZoomed.value = void 0
        }
      }
    })
  }

  if (
    indexZoomedState !== void 0 &&
    (cancel === void 0 || cancel() === false)
  ) {
    morph({
      from: thumbRef.value[ indexZoomedState ].$el,
      waitFor: 100,
      duration: 300
    })
  }
}

// Make sure to reset the dynamic refs before each update.
onBeforeUpdate(() => {
  thumbRef.value = []
})

</script>

<!-- <style lang="sass" scoped>
.truncate-chip-labels > .q-chip
  max-width: 300px
</style> -->