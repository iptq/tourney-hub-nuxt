<template>
  <div class="index-container">
    <div class="container card">
      <CardTitle title="Home" />
      <div class="card-content">
        Tourney Hub is an online platform for centralized osu! tournament
        hosting. Currently, the website is only planned to be developed for osu!
        gameplay tournaments, but may potentially, not necessarily, possibly be
        expanded to mapping tournaments in the future.
      </div>
    </div>

    <div class="container card">
      <CardTitle title="goals"/>
      <div class="card-content">
        <p>Main goals:</p>
        <ul>
          <li>finish tournament creation functionality</li>
          <li>Challonge-like feature</li>
          <li>
            tournament staff team with appropriate tournament-configuration
            permissions
          </li>
          <li>rank-restriction</li>
          <li>show all tournaments on tournaments page</li>
          <li>create filters for tournament searching</li>
          <li>mappool caching</li>
        </ul>
      </div>
      <div class="card-content">
        <p>Feature ideas:</p>
        <ul>
          <li>rescheduling requests</li>
          <li>BWS rank calculation</li>
          <li>auto-ref bots</li>
          <li>team creation?</li>
          <li>discord bot for scheduling and stuff</li>
          <li>
            automatic calendar creation for importing into google calendar and
            thunderbird
          </li>
        </ul>
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
    align-items: stretch;

    > div.card {
      width: calc(100% - 2px);
    }
  }

</style>
