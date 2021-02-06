<template>
  <div id="learn" class="container" style="max-width: 800px">
    <h1>Learn Hnefatafl</h1>

    <div class="rule row">
      <h3 class="mx-1">1) Starting State</h3>
      <div class="rule-text col-12">
        The game is played by two players on a board of 11x11 tiles.
        One player takes control of the black pieces (The Attackers), the other takes control of the white pieces (The
        Defenders).
        The game is played in turns, the attackers start. The starting position is shown below.
      </div>
      <div class="col-12">
        <img src="/assets/learn-1.png" width="100%">
      </div>
    </div>

    <div class="rule row">
      <h3 class="mx-1">2) Moving</h3>
      <div class="rule-text col-12 col-sm-12">
        Pieces can move horizontally and vertically (like rooks in chess).
        Pieces may not jump over other pieces.
        Only the King (the white piece with the dot) may move onto the bases (the orange tiles), but other pieces can
        jump over the bases.
        The last move is marked with yellow tiles.
      </div>
      <div class="col-6 col-md-4">
        <img src="/assets/learn-2a.png" width="100%">
      </div>
      <div class="col-6 col-md-4">
        <img src="/assets/learn-2b.png" width="100%">
      </div>
    </div>

    <div class="rule row">
      <h3 class="mx-1">3) Capturing</h3>
      <div class="rule-text col-12 col-sm-12">
        A piece other than the king is captured when it is caught between two enemies along a row or column.
        You can move a piece between two enemy pieces, without it being captured.
        A piece is also captured if it is caught between an enemy and a base.
        The captured piece is marked with a red tile.
      </div>
      <div class="col-6 col-md-4">
        <img src="/assets/learn-3a.png" width="100%">
      </div>
      <div class="col-6 col-md-4">
        <img src="/assets/learn-3b.png" width="100%">
      </div>
      <div class="rule-text col-12 col-sm-12">
        A row of pieces at the edge of the board may be captured by completely surrounding them against the board edge, so that none of them have room to move.
        The capturing move must be a flanking move to a square at the edge of the board. This is the "shield wall" capture.
      </div>
      <div class="col-6 col-md-4">
        <img src="/assets/learn-3c.png" width="100%">
      </div>
    </div>

    <div class="rule row">
      <h3 class="mx-1">4) Winning</h3>
      <div class="rule-text col-12 col-sm-12">
        The attackers win if
        <ol>
          <li>The king is surrounded on 4 sides by attackers.</li>
          <li>The king is surrounded on 3 sides by attackers, and the remaining side is the central base.</li>
          <li>All the defender's pieces are surrounded by attacker's pieces, so that none can reach the board edges. (Unimplemented!)</li>
        </ol>
      </div>
      <div class="rule-text col-12 col-sm-12">
        The defenders win if
        <ol>
          <li>The king reaches one of the corner squares.</li>
          <li>The king is surrounded on 3 sides by attackers, and the remaining side is the central base.</li>
          <li>The king constructs an "edge fort": A layout of defenders at the edge of the board which cannot be captured, and which allows the king free movement inside. (Unimplemented!)</li>
        </ol>
      </div>
      <div class="rule-text col-12 col-sm-12">
        Rules regarding repetition:
        <ol>
          <li>A player who has no legal moves, loses the game. (Unimplemented!)</li>
          <li>Perpetual repetition is illegal. If a move would cause a board position to be repeated for a third time, this move is illegal. (Unimplemented!)</li>
        </ol>
      </div>
    </div>

  </div>
</template>

<script>
export default {
  name: 'learn',
  props: {},
  data() {
    return {}
  },
  methods: {}
}
</script>

<style>
</style>