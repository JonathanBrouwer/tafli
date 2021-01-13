<template>
  <div class="container" id="home">
    <div class="row g-0">
      <div class="col-12 col-lg-6">
        <div class="mx-3">
          a
        </div>
      </div>
      <div class="col-12 col-lg-6 px-3">
        <h3 class="mx-1">Create game</h3>
        <div class="row g-0">
          <div class="col-4 px-1 mb-2" v-for="time in time_modes" :key="time">
            <div class="card py-3 mb-2 text-center time-card">
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
            <tr>
              <td>Jonathan</td>
              <td>Copenhagen</td>
              <td>5 + 3</td>
              <td>10 seconds ago</td>
            </tr>
            <tr>
              <td>Jonathan</td>
              <td>Copenhagen</td>
              <td>5 + 3</td>
              <td>10 seconds ago</td>
            </tr>
            <tr>
              <td>Jonathan</td>
              <td>Copenhagen</td>
              <td>5 + 3</td>
              <td>10 seconds ago aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa</td>
            </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'home',
  props: {},
  data() {
    return {
      time_modes: [[2, 1, "Bullet"], [3, 2, "Blitz"], [5, 3, "Blitz"], [10, 5, "Rapid"], [15, 15, "Classical"]]
    }
  },
  methods: {}
}
</script>

<style>
.homeblock {
  background: hsla(0, 0%, 100%, .5);
  border: .05367vw solid rgba(0, 0, 0, .1);
  padding: 20px;
}

.time-card {
  background: white;
  height: 100%;
}
</style>