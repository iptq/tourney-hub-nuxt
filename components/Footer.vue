<template>
  <footer>
    <hr />
    <div class="container">
      <div class="footer-links">
        <a :href="link.url" v-for="link in links">{{ link.name }}</a>
      </div>
    </div>
  </footer>
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
  links: Link[] = [
    { url: "https://github.com/iptq/tourney-hub", name: "source" },
    { url: "https://kit.svelte.dev", name: "svelte" },
  ];

  get isLoggedIn() {
    return this.$store.getters.isLoggedIn;
  }
}
</script>

<style>
footer {
  background-color: var(--bg-color-2);
  height: 100%;
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
}

div.footer-links {
  display: flex;
  gap: var(--pad-size);
  text-transform: capitalize;
}
</style>
