<template>
  <nav>
    <div class="container">
      <div>
        <a class="nav-link brand" href="/">
          <img class="logo" src="~/assets/logo.svg" alt="logo" />
          <span>Tourney Hub</span>
        </a>
        <NuxtLink
          class="nav-link"
          :to="link.url"
          v-for="link in links"
          :key="link.url"
        >
          {{ link.name }}
        </NuxtLink>
      </div>
      <b>HELLOSU {{ isAuthenticated }} {{ loggedInUser }} END</b>
      <div>
        <NuxtLink
          class="nav-link nav-right"
          :to="`/user/`"
          v-if="isAuthenticated"
          >Username</NuxtLink
        >
        <a class="nav-link nav-right" href="/api/auth/login" v-else>Login</a>
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
  links: Link[] = [
    { url: "/tournaments", name: "Tournaments" },
    { url: "/standings", name: "Standings" },
  ];

  get isAuthenticated() {
    return this.$auth.loggedIn;
  }

  get loggedInUser() {
    return this.$auth.user;
  }

  async logout() {
    await this.$auth.logout();
  }
}
</script>

<style lang="scss" scoped>
nav {
  background-color: var(--bg-color-2);
  border-bottom: 1px solid var(--bg-color-4);
  background-image: linear-gradient(
    45deg,
    var(--bg-color-2) 25%,
    var(--bg-color-3) 25%,
    var(--bg-color-3) 50%,
    var(--bg-color-2) 50%,
    var(--bg-color-2) 75%,
    var(--bg-color-3) 75%,
    var(--bg-color-3) 100%
  );
  background-size: 33.94px 33.94px;

  > div.container {
    background-color: var(--bg-color-2);
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--pad-size) var(--dense-pad-size);

    @media screen and (max-width: 600px) {
      flex-flow: column wrap;
      gap: 0;
    }

    > div {
      display: flex;
      flex-flow: row wrap;

      @media screen and (max-width: 600px) {
        align-items: center;
        flex-flow: column wrap;
      }
    }

    .nav-link {
      padding: var(--dense-pad-size) var(--pad-size);
      text-decoration: none;
      color: var(--main-font-color);
      text-transform: uppercase;

      &:not(.brand) {
        padding: var(--pad-size);
      }

      &:hover {
        box-shadow: inset 0 -3px 0 0 var(--accent-color);
      }
    }
  }
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
