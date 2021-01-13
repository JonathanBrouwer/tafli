<template>
  <navbar></navbar>
  <Home v-if="route === '/'"></Home>
  <Learn v-if="route === '/learn'"></Learn>
  <Game v-if="route.startsWith('/game')" :gameid="parseInt(route.split('/')[2])"></Game>
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
      route: window.location.pathname,
    }
  },
  computed: {
    CurrentComponent() {
      return routes[this.route] || NotFound
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
