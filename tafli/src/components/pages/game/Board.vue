<template>
  <div id="board">
    <div v-for="y in 11" :key="y" class="row g-0 board-row">
      <div v-for="x in 11" :key="x"
           class="col board-tile"
           v-bind:class="color_tile(x-1, y-1)"
           v-on:click="click_square(x-1, y-1)">
        <div v-if="game.board.fields[x - 1][y - 1] === FieldState.WhitePiece"
             class="board-piece board-piece-white"></div>
        <div v-if="game.board.fields[x - 1][y - 1] === FieldState.WhiteKing"
             class="board-piece board-piece-white board-piece-king"></div>
        <div v-if="game.board.fields[x - 1][y - 1] === FieldState.BlackPiece"
             class="board-piece board-piece-black"></div>
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
import {FieldState, player} from "@/ts/board_configuration";
import {Game} from "@/ts/game";

export default {
  name: 'board',
  props: {
    game: Game,
  },
  data() {
    return {
      FieldState: FieldState,
      active_square: null,
      legal_moves: []
    }
  },
  methods: {
    click_square(x, y) {
      if (this.game.status !== 'Playing') {
        this.active_square = null;
        this.legal_moves = [];
        return;
      }
      //First move
      if (this.active_square === null) {
        //If we don't click on our own piece
        if (player(this.game.board.fields[x][y]) !== this.game.board.turn) {
          this.active_square = null;
          this.legal_moves = [];
          return;
        }

        //Select the piece, and get legal moves
        this.select_square(x, y);
      } else {
        //If we click the same piece again, deselect
        if (this.active_square[0] === x && this.active_square[1] === y) {
          this.active_square = null;
          this.legal_moves = [];
          return;
        }
        //If we click another piece of ourself, select that
        if (player(this.game.board.fields[x][y]) === this.game.board.turn) {
          this.select_square(x, y);
          return;
        }
        //Else, make a move
        let from = this.active_square;
        let to = [x, y];
        fetch("http://localhost:8000/api/make_move?from=" + from[0] + "," + from[1] + "&to=" + to[0] + "," + to[1], {method: 'POST'});
        this.active_square = null;
        this.legal_moves = [];
      }
    },
    select_square(x, y) {
      this.active_square = [x, y];
      this.legal_moves = [];
      fetch("http://localhost:8000/api/legal_moves?pos=" + this.active_square[0] + "," + this.active_square[1])
          .then(res => res.json())
          .then(data => {
            this.legal_moves = data.moves;
          });
    },
    color_tile(x, y) {
      return {
        'selected': this.active_square !== null && !(this.active_square[0] !== x || this.active_square[1] !== y),
        'legal-move': this.legal_moves.filter(m => m[0] === x && m[1] === y).length > 0,
        'prev-move': this.game !== null && this.game.prev_move_info.last_move !== null &&
            ((this.game.prev_move_info.last_move[0][0] === x && this.game.prev_move_info.last_move[0][1] === y) ||
                (this.game.prev_move_info.last_move[1][0] === x && this.game.prev_move_info.last_move[1][1] === y)),
        'captured': this.game !== null && this.game.prev_move_info.captures.filter(p => p[0] === x && p[1] === y).length > 0
      }
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
  background: rgba(210, 90, 10, .3);
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

.board-tile.selected {
  background: rgba(40, 167, 69, .6) !important;
}
.board-tile.prev-move {
  background: rgb(255, 246, 186) !important;
}
.board-tile.captured {
  background: rgb(248, 189, 189) !important;
}
.board-tile.legal-move {
  background: rgba(40, 167, 69, .2) !important;
}
</style>