<template>
  <div id="game" class="container-fluid">
    <div v-if="game !== null" class="row justify-content-center">
      <div class="col-12 col-lg-5 col-xl-3">
        <meta-view :game="game"></meta-view>
      </div>
      <div class="col-12 col-lg-7 col-xl-5">
        <board ref="boardComp" :game="game"></board>
      </div>
    </div>
    <div v-if="game === null && !connected" class="row justify-content-center">
      <div class="col-12 col-md-6 col-lg-4">
        <div class="card p-3 mb-2">
          Connecting to game server...
          <a class="btn btn-block btn-outline-dark mt-3" href="/">Cancel</a>
        </div>
      </div>
    </div>
    <div v-if="game === null && connected" class="row justify-content-center">
      <div class="col-12 col-md-6 col-lg-4">
        <div class="card p-3 mb-2">
          Searching for game...
          <a class="btn btn-block btn-outline-dark mt-3" href="/">Cancel</a>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import Board from "@/components/pages/game/Board";
import MetaView from "@/components/pages/game/MetaView";
import {deserialize, plainToClass} from "class-transformer";
import {Game} from "@/ts/game";
import {BoardConfiguration} from "@/ts/board_configuration";

export default {
  name: 'game',
  components: {
    Board,
    MetaView
  },
  props: {
    gameid: Number
  },
  data() {
    return {
      connected: false,
      game: null
    }
  },
  methods: {
    clear_focus() {
      if (this.$refs.boardComp !== undefined) {
        this.$refs.boardComp.$data.active_square = null;
        this.$refs.boardComp.$data.legal_moves = [];
      }
    }
  },
  mounted() {
    let ws = new WebSocket("ws://localhost:8000/api/get_game?id=" + this.gameid);
    ws.onopen = event => {
      this.connected = true;
    }
    ws.onmessage = event => {
      let game = deserialize(Game, event.data);
      game.board = plainToClass(BoardConfiguration, game.board);
      this.clear_focus();
      this.game = game;
    }
    ws.onclose = event => {
      this.connected = false;
    }
  }
}
</script>

<style>
</style>