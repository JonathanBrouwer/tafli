<template>
  <navbar></navbar>
  <component v-bind:is="CurrentComponent"></component>
</template>

<script>

import Navbar from "@/components/Navbar";
import Game from "@/components/pages/game/Game";
import Learn from "@/components/pages/Learn";
import NotFound from "@/components/pages/NotFound";
import * as Vue from "@vue/runtime-core";

const routes = {
  '/': Game,
  '/learn': Learn
}

export default {
  name: 'App',
  components: {Navbar},
  data() {
    return {
      currentRoute: window.location.pathname
    }
  },
  computed: {
    CurrentComponent() {
      return routes[this.currentRoute] || NotFound
    }
  }
}
</script>

<style>
html, body, #app {
  height: 100%;
  background: #f0f0f0;
}
</style>
