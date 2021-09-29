<template>
  <nav>
    <div class="container">
      <div>
        <a class="nav-link brand" href="/">
          <img class="logo" src="{logo}" alt="logo" />
          <span>Tourney Hub</span>
        </a>
        <NuxtLink class="nav-link" to="link.url" v-for="link in links" :key="link.url">
          {{ link.name }}
        </NuxtLink>
      </div>
      <div>
        {#if $session.isLoggedIn}
        <a class="nav-link nav-right" href="/user/{$session.user.osu_id}"
          >{$session.user.username}</a
        >
        {:else}
        <a class="nav-link nav-right" href="/auth/login">Login</a>
        {/if}
      </div>
    </div>
  </nav>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

export interface Link {
  url: string;
  name: string;
}

@Component
export default class extends Vue {
  links: Link[] = [{ url: "/tournaments", name: "Tournaments" }];
}
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
nav > div.container .nav-link {
  padding: var(--dense-pad-size) var(--pad-size);
  text-decoration: none;
  color: var(--main-font-color);
  text-transform: uppercase;
}
nav > div.container .nav-link:not(.brand) {
  padding: var(--pad-size);
}
nav > div.container .nav-link:hover {
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
