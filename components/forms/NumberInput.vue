<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

@Component({
  props: {
    min: Number,
    max: Number,
    widthRem: Number,
    name: String,
    text: String,
  },
})
export default class extends Vue {}
</script>

<template>
  <div>
    <input
      type="number"
      :name="name"
      :style="'--width:' + widthRem"
      :min="min"
      :max="max"
    />
    {{ text }}
  </div>
</template>

<style lang="scss" scoped>
div {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: var(--dense-pad-size);

  > input {
    background-color: unset;
    border: 1px solid var(--bg-color-4);
    border-radius: var(--border-radius);
    color: unset;
    width: calc(var(--width) * 1rem);

    &:focus {
      outline: 2px solid var(--bg-color-4);
    }

    &::-webkit-outer-spin-button,
    input::-webkit-inner-spin-button {
      -webkit-appearance: none;
      margin: 0;
    }

    &[type="number"] {
      -moz-appearance: textfield;
    }
  }
}
</style>
