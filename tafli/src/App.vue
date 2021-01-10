<template>
  <navbar></navbar>
  <div class="container-fluid">
    <div class="row">
      <div class="col-12 col-lg">Left</div>
      <div class="col-12 col-lg">
        <board :state="{x: 5}"></board>
      </div>
      <div class="col-12 col-lg">Right</div>
    </div>
  </div>
</template>

<script>

import Board from "@/components/Board";
import Navbar from "@/components/Navbar";

export default {
  name: 'App',
  components: {Board, Navbar},
  data() {
    return {}
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
