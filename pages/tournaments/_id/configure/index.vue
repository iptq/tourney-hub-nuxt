<template>
  <div class="tourney-config-wrapper">
    <div class="meta-settings">
      <h2>Meta</h2>
      <FormsTextInput :label="'Tournament Name'" :fieldName="'tourneyName'" />
    </div>
    <div class="game-settings">
      <h2>Game Settings</h2>
      <div class="game-dropdowns">
        <FormsDropdown
          :label="'Win Condition'"
          :fieldName="'winCondition'"
          :options="winConditionOptions"
        />
        <FormsDropdown
          :label="'Max Team Size'"
          :fieldName="'awfwinCondition'"
          :options="winConditionOptions"
        />
        <FormsDropdown
          :label="'Min Team Size'"
          :fieldName="'awinCondition'"
          :options="winConditionOptions"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

export default {
  layout: "tourney_config",
  data() {
    return {
      winConditionOptions: ["scorev2", "scorev1", "accuracy", "max combo"],
    };
  },
};
</script>

<style lang="scss" scoped>
div.tourney-config-wrapper {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
  gap: 1px;
  background-color: var(--bg-color-4);

  > div.game-settings,
  div.meta-settings {
    display: flex;
    flex-direction: column;
    gap: var(--pad-size);

    background-color: var(--bg-color-1);
    padding: var(--pad-size);

    > div.game-dropdowns {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
      gap: var(--pad-size);
    }
  }
}

h2 {
  margin: 0;
}
</style>
