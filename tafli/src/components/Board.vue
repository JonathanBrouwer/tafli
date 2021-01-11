<template>
  <div id="board">
    <div v-for="y in 11" :key="y" class="row g-0 board-row">
      <div v-for="x in 11" :key="x" class="col board-tile">
        <div v-if="state.fields[x - 1][y - 1] === FieldState.WhitePiece" class="board-piece board-piece-white"></div>
        <div v-if="state.fields[x - 1][y - 1] === FieldState.WhiteKing"
             class="board-piece board-piece-white board-piece-king"></div>
        <div v-if="state.fields[x - 1][y - 1] === FieldState.BlackPiece" class="board-piece board-piece-black"></div>
      </div>
      <div class="col count row-count">{{ 12 - y }}</div>
    </div>
    <div class="row g-0">
      <div v-for="x in 11" :key="x" class="col count col-count">{{ String.fromCharCode(65 + x - 1) }}</div>
      <div class="col count row-count"></div>
    </div>

  </div>
</template>

<script>
import {BoardConfiguration, FieldState} from "../ts/board_configuration";

export default {
  name: 'board',
  props: {
    state: BoardConfiguration
  },
  data() {
    return {
      FieldState: FieldState
    }
  }
}
</script>

<style>
.count {
  font-size: 11px;
  font-weight: bolder;
  display: flex;
  align-items: center;
  justify-content: center;
}

.row-count {
  max-width: 20px;
}

.col-count {
  height: 20px;
}

#board .board-row:nth-child(1) .board-tile:nth-child(1),
#board .board-row:nth-child(11) .board-tile:nth-child(1),
#board .board-row:nth-child(1) .board-tile:nth-child(11),
#board .board-row:nth-child(11) .board-tile:nth-child(11),
#board .board-row:nth-child(6) .board-tile:nth-child(6) {
  background: rgba(210, 90, 10, .3) !important;
}

.board-tile {
  background: hsla(0, 0%, 100%, .5);
  border: .05367vw solid rgba(0, 0, 0, .1);
  cursor: pointer;

  display: flex;
  align-items: center;
  justify-content: center;
}

.board-tile:before {
  content: '';
  float: left;
  padding-top: 100%;
}

.board-piece {
  border: .14312vw solid #222;
  border-radius: 5px;
  height: 60%;
  width: 60%;

  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.board-piece.board-piece-white {
  background: white;
}

.board-piece.board-piece-black {
  background: black;
}

.board-piece.board-piece-king::after {
  content: "";
  display: block;
  position: absolute;
  border-radius: 100%;
  width: 50%;
  height: 50%;
  background: #111;
}
</style>