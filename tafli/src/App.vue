<template>
  <navbar></navbar>
  <div class="container-fluid">
    <div class="row">
      <div class="col-12 col-lg-7 order-lg-1 col-xl-5">
        <board v-if="game !== null" :state="game.board"></board>
        <span v-if="game === null">Loading...</span>
      </div>
      <div class="col-12 col-lg-5 order-lg-0 col-xl">Left</div>
      <div class="col-12 col-lg-12 order-lg-2 col-xl"></div>
    </div>
  </div>
</template>

<script>

import Board from "@/components/Board";
import Navbar from "@/components/Navbar";
import {Game} from "@/ts/game";
import {deserialize, plainToClass} from "class-transformer";
import {BoardConfiguration} from "@/ts/board_configuration";

export default {
  name: 'App',
  components: {Board, Navbar},
  data() {
    return {
      game: null
    }
  },
  mounted() {
    let ws = new WebSocket("ws://localhost:8000/api/get_game");
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
