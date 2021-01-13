<template>
  <navbar></navbar>
  <Home v-if="currentRoute === '/'"></Home>
  <Learn v-if="currentRoute === '/learn'"></Learn>
  <Game v-if="currentRoute === '/game'"></Game>
</template>

<script>

import Home from "@/components/pages/Home";
import Navbar from "@/components/Navbar";
import Game from "@/components/pages/game/Game";
import Learn from "@/components/pages/Learn";
import NotFound from "@/components/pages/NotFound";

const routes = {
  '/': Game,
  '/learn': Learn
}

export default {
  name: 'App',
  components: {Game, Learn, Navbar, Home},
  data() {
    return {
      currentRoute: window.location.pathname,
      game: null
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
