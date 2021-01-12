<template>
  <!-- Navbar -->
  <nav class="navbar navbar-expand-lg navbar-light bg-white mb-4" style="box-shadow: 0 0 10px rgba(0,0,0,.05);">
    <div class="container">
      <a class="navbar-brand" href="/">Tafli</a>
      <button aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation"
              class="navbar-toggler"
              data-bs-target="#navbarSupportedContent" data-bs-toggle="collapse" type="button">
        <span class="navbar-toggler-icon"></span>
      </button>
      <div id="navbarSupportedContent" class="navbar-collapse collapse" style="">
        <ul class="navbar-nav mr-auto">
          <li class="nav-item"><a class="nav-link" href="/">Play</a></li>
          <li class="nav-item"><a class="nav-link" href="/learn">Learn</a></li>
        </ul>
      </div>
    </div>
  </nav>
</template>

<script>
export default {
  name: 'navbar',
  data() {
    return {}
  }
}
</script>