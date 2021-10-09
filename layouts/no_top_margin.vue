<template>
  <div id="app">
    <div class="content-wrap">
      <Navbar />

      <div class="content-body container">
        <Nuxt />
      </div>

      <footer class="footer">
        <Footer />
      </footer>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  $footer-height: 62px;

  #app {
    position: relative;
    min-height: 100vh;
    .content-wrap {
      padding-bottom: $footer-height;
    }
  }

  footer.footer {
    position: absolute;
    bottom: 0;
    width: 100%;
    height: $footer-height;
  }
</style>
