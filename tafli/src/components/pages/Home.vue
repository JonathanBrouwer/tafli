<template>
  <div id="home" class="container">
    <div class="row g-0">
      <div class="col-12 col-lg-3 px-0 px-lg-2 px-xl-3">
        <h3 class="mx-1">How to play</h3>
        <div class="card p-3 mb-2">
          Tafli is a place to play Hnefatafl (Aka Viking Chess) online with others. This site implements the Copenhagen
          rules, make sure to read the rules before playing.
          <a class="btn btn-block btn-outline-dark mt-3" href="/learn">Learn the rules</a>
        </div>
        <h3 class="mx-1">Enter Name</h3>
        <div class="card p-3 mb-2">
          <label class="mb-2" for="name">What is your name?</label>
          <input id="name" v-model="name" placeholder="Anonymous">
        </div>

        <h3 class="mx-1 d-none d-lg-block">Discord</h3>
        <span class="d-none d-lg-block">//todo</span>
      </div>
      <div class="col-12 col-lg-9 col-xl-6 px-0 px-lg-2 px-xl-3">
        <h3 class="mx-1">Create game</h3>
        <div class="row g-0">
          <div v-for="time in time_modes" :key="time" class="col-4 px-1 mb-2">
            <div class="card py-3 mb-2 text-center time-card" v-on:click="create_game(time[0], time[1])">
              <h1>{{ time[0] }} + {{ time[1] }}</h1>
              <h4>{{ time[2] }}</h4>
            </div>
          </div>
          <div class="col-4 px-1 mb-2">
            <div class="card py-3 mb-2 text-center time-card">
              <h1>? + ?</h1>
              <h4>Custom</h4>
            </div>
          </div>
        </div>
        <h3 class="mx-1">Join game</h3>
        <div class="card table-responsive m-0">
          <table class="table table-striped table-hover m-0">
            <thead>
            <tr>
              <th scope="col">Player</th>
              <th scope="col">Rules</th>
              <th scope="col">Time</th>
              <th scope="col">Created at</th>
            </tr>
            </thead>
            <tbody>
            <tr v-for="pgame in part_games" :key="pgame" style="cursor: pointer;" v-on:click="join_game(pgame.game_id)">
              <td>{{ pgame.player.name }}</td>
              <td>Copenhagen</td>
              <td>{{ pgame.time_start }} + {{ pgame.time_incr }}</td>
              <td>{{ Math.round((Date.now() / 1000 - pgame.created_at) / 60) }} minutes ago</td>
            </tr>
            <tr v-if="part_games.length === 0">
              <td colspan="4">There are currently no games waiting for players</td>
            </tr>
            </tbody>
          </table>
        </div>
      </div>
      <div class="col-12 col-lg-3 px-0 px-lg-2 px-xl-3">
        <h3 class="mx-1">Watch</h3>
        //todo
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'home',
  components: {},
  props: {},
  data() {
    return {
      time_modes: [[2, 1, "Bullet"], [3, 2, "Blitz"], [5, 3, "Blitz"], [10, 5, "Rapid"], [15, 15, "Classical"]],
      part_games: [],
      name: ""
    }
  },
  methods: {
    create_game(t1, t2) {
      let url = new URL("http://localhost:8000/api/create_game");
      url.search = new URLSearchParams({
        player_name: this.name,
        time_start: t1,
        time_incr: t2
      });
      fetch(url.toString(), {method: 'POST', credentials: "include"})
          .then(res => res.json())
          .then(game_id => {
            window.location.href = "/game/" + game_id;
          });

    },
    join_game(id) {
      let url = new URL("http://localhost:8000/api/join_game");
      url.search = new URLSearchParams({
        player_name: this.name,
        gameid: id
      });
      fetch(url.toString(), {method: 'POST', credentials: "include"})
          .then(res => res.json())
          .then(res => {
            if (res) {
              window.location.href = "/game/" + id;
            } else {
              window.alert("Failed to join game.")
            }
          });
    }
  },
  mounted() {
    fetch("http://localhost:8000/api/list_partial_games", {credentials: "include"})
        .then(res => res.json())
        .then(data => {
          this.part_games = data;
        });
  }
}
</script>

<style>
.time-card {
  background: white;
  height: 100%;
  cursor: pointer;
}
</style>