<template>
  <div class="text-input">
    <small>{{ label }}</small>
    <input type="text" class="round-border" :name="fieldName">
  </div>
</template>

<script lang="ts">
  import Vue from "vue";
  import Component from "vue-class-component";

  @Component({
    props: {
      fieldName:              String,
      placeholderText: {type: String, default: ""},
      label:                  String
    }
  })
  export default class extends Vue{}
</script>

<style lang="scss" scoped>
  div.text-input {
    display: flex;
    flex-direction: column;
    gap: var(--dense-pad-size);

    > input[type=text] {
      padding: var(--dense-pad-size);
      background-color: unset;
      color: unset;

      &:focus {
        outline: 2px solid var(--bg-color-4)
      }
    }
  }
</style>
