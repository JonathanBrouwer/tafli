<template>
  <div class="mb-3 meta-view-line">
    <div class="meta-view-box">
      <h3>Hnefatafl (Copenhagen)</h3>
      <h5>15+15 • Casual • Classical</h5>
      Maybe some text here?
    </div>
  </div>
  <div class="mb-3 meta-view-line">
      <div class="row gap-3 g-0">
        <div class="col meta-view-box" :class="{active: game !== null && game.board.turn === Player.White}">
          <div class="player-indicator-name">
            <div class="board-piece board-piece-white player-indicator-icon"></div><span>Player Name (Rating)</span>
          </div>
          <div class="player-indicator-time">
            3:45
          </div>
        </div>
        <div class="col meta-view-box"  :class="{active: game !== null && game.board.turn === Player.Black}">
          <div class="player-indicator-name">
            <div class="board-piece board-piece-black player-indicator-icon"></div><span>Player Name (Rating)</span>
          </div>
          <div class="player-indicator-time">
            1:20
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
  methods: {
  }
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
</style>