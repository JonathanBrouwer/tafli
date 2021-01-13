<template>
  <div class="container-fluid" id="game">
    <div class="row justify-content-center">
      <div class="col-12 col-lg-5 col-xl-3">
        <meta-view :game="game"></meta-view>
      </div>
      <div class="col-12 col-lg-7 col-xl-5">
        <board ref="boardComp" v-if="game !== null" :game="game"></board>
        <span v-if="game === null">Loading...</span>
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
  data() {
    return {
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
    let ws = new WebSocket("ws://localhost:8000/api/get_game");
    ws.onmessage = event => {
      let game = deserialize(Game, event.data);
      game.board = plainToClass(BoardConfiguration, game.board);
      this.clear_focus();
      this.game = game;
    }
  }
}
</script>

<style>
</style>