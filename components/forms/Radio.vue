<template>
  <div class="custom-radio">
    <template v-if="checked">
      <label class="radio-container">
        <input type="radio" checked="checked" name="radio" />
        <span class="checkmark" />
      </label>
    </template>

    <template v-else>
      <label class="radio-container">
        <input type="radio" name="radio" />
        <span class="checkmark" />
      </label>
    </template>

    {{ text }}
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

@Component({
  props: ["text", "checked"],
})
export default class extends Vue {}
</script>

<style lang="scss" scoped>
div.custom-radio {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: var(--dense-pad-size);
}
label.radio-container {
  display: block;
  position: relative;
  user-select: none;
  cursor: pointer;

  > input {
    position: absolute;
    height: 0;
    width: 0;
    margin: 0;
    opacity: 0;
  }

  > span.checkmark {
    position: relative;
    display: inline-block;
    background-color: var(--bg-color-1);
    border: 1px solid var(--bg-color-4);
    border-radius: 100%;
    height: 1rem;
    width: 1rem;
  }
}
label.radio-container:hover input ~ span.checkmark {
  background-color: var(--bg-color-3);
}
label.radio-container input:checked ~ span.checkmark {
  background-color: var(--accent-color);
  border: 1px solid var(--accent-color);
}
span.checkmark:after {
  content: "";
  position: absolute;
  display: none;
}
label.radio-container input:checked ~ span.checkmark:after {
  display: block;
}
label.radio-container span.checkmark:after {
  top: 0.3rem;
  left: 0.3rem;
  width: 0.4rem;
  height: 0.4rem;
  border-radius: 100%;
  background: var(--bg-color-1);
}
</style>
