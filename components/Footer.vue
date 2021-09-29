<template>
  <nav>
    <div class="container">
      <div>
      <a class="nav-link brand" href="/">
        <img class="logo" src={logo} alt="logo" />
        <span>Tourney Hub</span>
      </a>
      {#each links as link}
        <a class="nav-link nav-left" href={link.url}>
          {link.name}
        </a>
      {/each}
    </div>
    <div>
      {#if $session.isLoggedIn}
        <a class="nav-link nav-right" href="/user/{$session.user.osu_id}" >{$session.user.username}</a >
      {:else}
        <a class="nav-link nav-right" href="/auth/login">Login</a>
      {/if}
    </div>
    </div>
  </nav>
</template>

<script lang="ts">
</script>

<style lang="scss" scoped>
  nav {
    background-color: var(--bg-color-2);
    border-bottom: 1px solid var(--bg-color-4);
    margin-bottom: var(--pad-size);
  }
  nav > div.container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--pad-size) var(--dense-pad-size);
  }
  nav > div.container > div {
    display: flex;
    flex-direction: row;
  }
  nav > div.container a.nav-link {
    padding: var(--dense-pad-size) var(--pad-size);
    text-decoration: none;
    color: var(--main-font-color);
    text-transform: uppercase;
  }
  nav > div.container a.nav-link:not(.brand) {
    padding: var(--pad-size);
  }
  nav > div.container a.nav-link:hover {
    box-shadow: inset 0 -3px 0 0 var(--accent-color);
  }
  a.brand {
    font-weight: bold;
    display: flex;
    align-items: center;
    column-gap: 1rem;
  }
  a.brand > img.logo {
    width: 2rem;
    height: 2rem;
  }
  .nav-left {
    flex-direction: row;
    align-self: flex-start;
  }
  .nav-right {
    align-self: flex-end;
    flex-direction: row-reverse;
  }
</style>
