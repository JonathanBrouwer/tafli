<template>
  <!-- Navbar -->
  <nav class="navbar navbar-expand-lg navbar-light">
    <div class="container"><a href="/" class="navbar-brand mr-3"><span class="logo">Litafl</span></a>
      <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
        <span class="navbar-toggler-icon"></span>
      </button>
      <div id="navbarSupportedContent" class="navbar-collapse collapse" style="">
        <ul class="navbar-nav mr-auto">
          <li class="nav-item"><a href="/" class="nav-link">Play</a></li>
          <li class="nav-item"><a href="/learn" class="nav-link">Learn</a></li>
        </ul>
      </div>
    </div>
  </nav>
</template>

<script>

export default {
  name: 'App',
  data() {
    return {
      msg: "hello"
    }
  }
}
</script>

<style>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}

#app {
  height: 100%;
  background: #f0f0f0;
}
</style>
