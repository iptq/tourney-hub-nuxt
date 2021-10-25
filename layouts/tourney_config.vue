<template>
  <div id="app">
    <div class="content-wrap">
      <Navbar />

      <div class="content-body">
        <div class="config-grid">
          <aside class="sidebar">
            <h2>Tournament Name</h2>
            <div class="menu-section">
              <input id="general-toggle" type="checkbox" />
              <label id="general-toggle-label" for="general-toggle">
                <a class="major-link">General</a>
                <i class="arrow" />
              </label>
              <div class="collapse-menu" id="general-menu">
                <a>Overview</a>
                <a>Timeline</a>
                <a>Staff</a>
                <a>Signups</a>
              </div>
            </div>
            <div class="menu-section">
              <input id="mappools-toggle" type="checkbox" />
              <label id="mappools-toggle-label" for="mappools-toggle">
                <a class="major-link">Mappools</a>
                <i class="arrow down" />
              </label>
              <div class="collapse-menu" id="mappools-menu">
                <a>General</a>
                <a>Suggestions</a>
                <a>Qualifiers</a>
                <a>Round of 64</a>
                <a>Round of 32</a>
                <a>Round of 16</a>
                <a>Quarterfinals</a>
                <a>Semifinals</a>
                <a>Finals</a>
                <a>Grand Finals</a>
              </div>
            </div>
            <div class="menu-section">
              <input id="utilities-toggle" type="checkbox" />
              <label id="utilities-toggle-label" for="utilities-toggle">
                <a class="major-link">Utilities</a>
                <i class="arrow down" />
              </label>
              <div class="collapse-menu" id="utilities-menu">
                <a>Referee Bots</a>
              </div>
            </div>
          </aside>
          <div class="config-body">
            <div class="config-body-title">
              <p>General > Meta</p>
            </div>
            <Nuxt />
          </div>
        </div>
      </div>

      <footer class="footer">
        <Footer />
      </footer>
    </div>
  </div>
</template>

<!--<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

export interface Link {
  url: string;
  name: string;
}

const styleProps = Vue.extend({
  props: {
    baseUrl: this.$route.fullPath
  }
})

@Component
export default class extends styleProps {
  data () {
    return {
      baseUrl: this.$route.fullPath
    }
  }
  links: Link[] = [
    { url: baseUrl + "/tournaments", name: "Tournaments" },
    { url: "/standings",   name: "Standings"}
  ];

}
</script>-->

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

div.config-grid {
  display: grid;
  grid-template-columns: auto 1fr;
  height: 100%;

  > aside.sidebar {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 12rem;
    border-right: 1px solid var(--bg-color-4);
    position: sticky;
    top: 0;
    overflow-y: scroll;
    padding: 0 var(--pad-size);

    h2 {
      margin: 0;
      padding: var(--pad-size) var(--pad-size) 0 var(--pad-size);
    }

    div.menu-section {
      display: flex;
      flex-direction: column;
      margin-top: 1rem;
      width: 100%;

      > input[type="checkbox"] {
        display: none;
      }

      > label {
        width: 100%;
      }

      > input {
        &#general-toggle:not(:checked) ~ #general-menu,
        &#mappools-toggle:not(:checked) ~ #mappools-menu,
        &#utilities-toggle:not(:checked) ~ #utilities-menu {
          transform: scaleY(0);
          height: 0;
        }

        &#general-toggle:not(:checked) ~ #general-toggle-label,
        &#mappools-toggle:not(:checked) ~ #mappools-toggle-label,
        &#utilities-toggle:not(:checked) ~ #utilities-toggle-label {
          > i.arrow {
            transform: rotate(-45deg);
            -webkit-transform: rotate(-45deg);
          }
        }
      }

      a {
        padding: var(--dense-pad-size) var(--pad-size);
        color: var(--alt-font-color);

        &:hover {
          color: var(--main-font-color);
          cursor: pointer;
          text-decoration: none;
        }
      }

      > label {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;

        &:hover {
          cursor: pointer;

          > a.major-link {
            color: var(--main-font-color);
          }

          > i.arrow {
            border: solid var(--main-font-color);
            border-width: 0 3px 3px 0;
          }
        }

        > a.major-link {
          font-size: 1.2rem;
        }

        > i.arrow {
          border: solid var(--alt-font-color);
          border-width: 0 3px 3px 0;
          display: inline-block;
          padding: 3px;

          transform: rotate(45deg);
          -webkit-transform: rotate(45deg);
        }
      }

      > div.collapse-menu {
        display: flex;
        flex-direction: column;

        > a {
          font-size: unset;
          text-decoration: none;

          &:hover {
            background-color: var(--bg-color-2);
          }
        }
      }
    }
  }

  div.config-body {
    > div.config-body-title {
      border-bottom: 1px solid var(--bg-color-4);
      padding: var(--dense-pad-size) var(--pad-size);

      p {
        margin: 0;
      }
    }
  }
}
</style>
