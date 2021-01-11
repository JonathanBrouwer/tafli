<template>
  <navbar></navbar>
  <div class="container-fluid">
    <div class="row">
      <div class="col-12 col-lg">Left</div>
      <div class="col-12 col-lg">
        <board :state="boarddata"></board>
      </div>
      <div class="col-12 col-lg">Right</div>
    </div>
  </div>
</template>

<script>

import Board from "@/components/Board";
import Navbar from "@/components/Navbar";
import {BoardConfiguration, FieldState, Player} from "./ts/board_configuration";

export default {
  name: 'App',
  components: {Board, Navbar},
  data() {
    return {
      boarddata: new BoardConfiguration([
        [FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty],
        [FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty],
        [FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty],
        [FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty],
        [FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty],
        [FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty],
        [FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty],
        [FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty],
        [FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty],
        [FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty],
        [FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty, FieldState.Empty]], Player.Black)
    }
  },
  mounted() {
    let ws = new WebSocket("ws://localhost:8000/api/get_board");
    ws.onmessage = event => {
      let board = JSON.parse(event.data);
      this.boarddata = Object.assign(BoardConfiguration, board);
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
