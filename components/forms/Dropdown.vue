<template>
  <div class="dropdown">
    <small>{{ label }}</small>
    <select :name="fieldName" class="round-border">
      <option v-for="option in options" :key="option" value="option">
        {{ option }}
      </option>
    </select>
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";
import { PropType } from "vue";

@Component({
  props: {
    fieldName: String,
    options: Array as PropType<string[]>,
    label: String,
  },
})
export default class extends Vue {}
</script>

<style lang="scss" scoped>
div.dropdown {
  display: flex;
  flex-direction: column;
  gap: var(--dense-pad-size);

  > select {
    padding: var(--dense-pad-size);
    background-color: unset;
    color: unset;

    > option {
      color: initial;
    }
  }
}
</style>
