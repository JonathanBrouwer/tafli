<template>
  <div class="mb-3 meta-view-line">
    <div class="meta-view-box">
      <h3>Hnefatafl (Copenhagen)</h3>
      <h5>15+15 • Casual • Classical</h5>
      <div class="player-indicator-name">
        <div class="board-piece board-piece-white player-indicator-icon"></div>
        <span>Player Name (Rating)</span>
      </div>
      <div class="player-indicator-name">
        <div class="board-piece board-piece-black player-indicator-icon"></div>
        <span>Player Name (Rating)</span>
      </div>
      <hr>
      <div class="meta-victory">
        <span v-if="game !== null && game.status === 'Playing'">Game is ongoing</span>
        <span v-if="game !== null && game.status.Won !== undefined && game.status.Won[0] === Player.White">
          White is victorious • {{ game.status.Won[1] }}
        </span>
        <span v-if="game !== null && game.status.Won !== undefined && game.status.Won[0] === Player.Black">
          Black is victorious • {{ game.status.Won[1] }}
        </span>
      </div>
    </div>
  </div>
  <div class="mb-3 meta-view-line">
    <div class="row gap-3 g-0">
      <div :class="{active: game !== null && game.status === 'Playing' && game.board.turn === Player.White}"
           class="col meta-view-box">
        <div class="player-indicator-name justify-content-center">
          <div class="board-piece board-piece-white player-indicator-icon"></div>
          <span>Player Name</span>
        </div>
        <div class="player-indicator-time">
          00:00
        </div>
      </div>
      <div :class="{active: game !== null && game.status === 'Playing' && game.board.turn === Player.Black}"
           class="col meta-view-box text-center">
        <div class="player-indicator-name justify-content-center">
          <div class="board-piece board-piece-black player-indicator-icon"></div>
          <span>Player Name</span>
        </div>
        <div class="player-indicator-time">
          00:00
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import {Game} from "@/ts/game";
import {Player} from "@/ts/board_configuration";

export default {
  name: 'metaview',
  props: {
    game: Game
  },
  data() {
    return {
      Player: Player
    }
  },
  methods: {}
}
</script>

<style>
.meta-view-box {
  background: hsla(0, 0%, 100%, .5);
  border: .05367vw solid rgba(0, 0, 0, .1);
  padding: 10px;
}

.meta-view-box.active {
  background: rgba(40, 167, 69, .2);
}

.player-indicator-name {
  display: flex;
  align-items: center;
}

.player-indicator-icon {
  width: 20px;
  height: 20px;
  margin-right: 10px;
}

.player-indicator-time {
  font-size: 50px !important;
  line-height: 50px !important;
  font-weight: 500;
  text-align: center;
}

.player-indicator-win {
  margin-left: 30px;
}
</style>