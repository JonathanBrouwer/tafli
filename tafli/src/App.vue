<template>
  <navbar></navbar>
  <div class="container-fluid">
    <div class="row justify-content-center">
      <div class="col-12 col-lg-5 col-xl-3">
        <meta-view :game="game"></meta-view>
      </div>
      <div class="col-12 col-lg-7 col-xl-5">
        <board v-if="game !== null" :game="game"></board>
        <span v-if="game === null">Loading...</span>
      </div>
    </div>
  </div>
</template>

<script>

import Board from "@/components/Board";
import Navbar from "@/components/Navbar";
import {Game} from "@/ts/game";
import {deserialize, plainToClass} from "class-transformer";
import {BoardConfiguration} from "@/ts/board_configuration";
import MetaView from "@/components/MetaView";

export default {
  name: 'App',
  components: {Board, Navbar, MetaView},
  data() {
    return {
      game: null
    }
  },
  mounted() {
    let ws = new WebSocket("ws://192.168.2.19:8000/api/get_game");
    ws.onmessage = event => {
      let game = deserialize(Game, event.data);
      game.board = plainToClass(BoardConfiguration, game.board);

      this.game = game;
    }
  }
}
</script>

<style>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

html, body, #app {
  height: 100%;
  background: #f0f0f0;
}
</style>
