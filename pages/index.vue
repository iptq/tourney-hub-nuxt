<template>
  <div class="index-container container">
    <div class="index-grid">
      <div class="content-side">
        <div class="card">
          <div class="card-content stat-box">
            <span class="stat-value">69</span>
            <span class="stat-name">Tournaments Hosted</span>
          </div>
          <div class="card-content stat-box">
            <span class="stat-value">727</span>
            <span class="stat-name">Matches Played</span>
          </div>
          <div class="card-content stat-box">
            <span class="stat-value">1337</span>
            <span class="stat-name">Players Registered</span>
          </div>
        </div>
      </div>
      <div class="content-body">
        <iframe
          class="livestream"
          src="https://player.twitch.tv/?riotgames&parent=localhost:3001"
          allowfullscreen="<allowfullscreen>">
        </iframe>
      </div>
    </div>
    <div class="index-grid">
      <div class="content-side">
        <div class="card">
          <CardTitle :title="'New Tournaments'" />
          <div class="card-content">
            yes
          </div>
        </div>
      </div>
      <div class="content-body">
        <div class="card">
          <CardTitle :title="'Recent Matches'" />
          <div class="card-content">
            osu
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
export default Vue.extend({});
</script>

<style lang="scss" scoped>
  div.index-container {
    display: flex;
    flex-direction: column;
    gap: var(--pad-size);

    div.index-grid {
      display: grid;
      grid-template-columns: 1fr 3fr;
      grid-template-areas: "content-side content-body";
      gap: var(--pad-size);

      > div.content-side {
        grid-area: content-side;
        display: flex;
        flex-direction: column;
        gap: var(--pad-size);

        div.stat-box {
          display: flex;
          flex-direction: column;
          align-items: center;

          .stat-value {
            font-size: 3rem;
            font-weight: bold;
          }

          .stat-name {
            opacity: var(--faded-opacity);
          }
        }
      }

      > div.content-body {
        grid-area: content-body;
        display: flex;
        flex-direction: column;
        gap: var(--pad-size);

        > iframe.livestream {
          border: 1px solid var(--bg-color-4);
          border-radius: var(--border-radius);
          height: 100%;
        }
      }
    }
  }
</style>
