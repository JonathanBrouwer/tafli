<template>
  <!-- Navbar -->
  <nav class="navbar navbar-expand-lg navbar-light bg-white mb-4" style="box-shadow: 0 0 10px rgba(0,0,0,.05);">
    <div class="container">
      <a href="/" class="navbar-brand">Tafli</a>
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
  name: 'navbar',
  data() {
    return {
    }
  }
}
</script>