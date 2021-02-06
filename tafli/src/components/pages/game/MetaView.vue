<template>
  <div class="mb-3 meta-view-line">
    <div class="meta-view-box">
      <h3>Hnefatafl (Copenhagen)</h3>
      <h5>{{ game.time_control[0] }} + {{ game.time_control[1] }} • Casual</h5>
      <div class="player-indicator-name">
        <div class="board-piece board-piece-white player-indicator-icon"></div>
        <span>{{ game.player_white.name }}</span>
        <svg v-if="this.side === BoardSide.White" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person-fill" viewBox="0 0 16 16" style="margin-left: 5px;">
          <path d="M3 14s-1 0-1-1 1-4 6-4 6 3 6 4-1 1-1 1H3zm5-6a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"/>
        </svg>
      </div>
      <div class="player-indicator-name">
        <div class="board-piece board-piece-black player-indicator-icon"></div>
        <span>{{ game.player_black.name }}</span>
        <svg v-if="this.side === BoardSide.Black" xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-person-fill" viewBox="0 0 16 16" style="margin-left: 5px;">
          <path d="M3 14s-1 0-1-1 1-4 6-4 6 3 6 4-1 1-1 1H3zm5-6a3 3 0 1 0 0-6 3 3 0 0 0 0 6z"/>
        </svg>
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
          <span>{{ game.player_white.name }}</span>
        </div>
        <div class="player-indicator-time">
          00:00
        </div>
      </div>
      <div :class="{active: game !== null && game.status === 'Playing' && game.board.turn === Player.Black}"
           class="col meta-view-box text-center">
        <div class="player-indicator-name justify-content-center">
          <div class="board-piece board-piece-black player-indicator-icon"></div>
          <span>{{ game.player_black.name }}</span>
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
import {BoardSide, Player} from "@/ts/board_configuration";

export default {
  name: 'metaview',
  props: {
    game: Game,
    side: BoardSide
  },
  data() {
    return {
      Player: Player,
      BoardSide: BoardSide
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