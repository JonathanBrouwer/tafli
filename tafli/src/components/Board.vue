<template>
  <div id="board">
    {{state.x}}
    <div class="row g-0 board-row" v-for="y in 11" :key="y">
      <div v-for="x in 11" :key="x" class="col board-tile bg-white">

      </div>
      <div class="col count row-count">{{y}}</div>
    </div>
    <div class="row g-0">
      <div class="col count col-count" v-for="x in 11" :key="x">{{String.fromCharCode(65 + x - 1)}}</div>
      <div class="col count row-count"></div>
    </div>

  </div>
</template>

<script>
// let bases = [[0,0]];
export default {
  name: 'board',
  props: {
    state: Object
  },
  data() {
    return {
    }
  }
}
</script>

<style>
.board-tile {
  border: .05367vw solid rgba(0,0,0,.1);
  cursor: pointer;
}
.board-tile:before {
  content:'';
  float:left;
  padding-top:100%;
}
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
#board .board-row:nth-child(6) .board-tile:nth-child(6){
  background: rgba(210,90,10,.3) !important;
}
</style>