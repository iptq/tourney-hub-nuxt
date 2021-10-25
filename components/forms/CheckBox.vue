<template>
  <div class="custom-checkbox">
    <label class="checkbox-container">
      <input type="checkbox" />
      <span class="checkmark" />
    </label>
    {{ text }}
  </div>
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

@Component({
  props: ["text"],
})
export default class extends Vue {}
</script>

<style lang="scss" scoped>
div.custom-checkbox {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: var(--dense-pad-size);
}

label.checkbox-container {
  display: block;
  position: relative;
  user-select: none;
  cursor: pointer;
}

label.checkbox-container > input {
  position: absolute;
  height: 0;
  width: 0;
  margin: 0;
  opacity: 0;
}

label.checkbox-container > span.checkmark {
  position: relative;
  display: inline-block;
  background-color: var(--bg-color-1);
  border: 1px solid var(--bg-color-4);
  border-radius: var(--border-radius);
  height: 1rem;
  width: 1rem;
}
label.checkbox-container:hover input ~ .checkmark {
  background-color: var(--bg-color-3);
}
label.checkbox-container input:checked ~ .checkmark {
  background-color: var(--accent-color);
  border: 1px solid var(--accent-color);
}

.checkmark:after {
  content: "";
  position: absolute;
  display: none;
}

.checkbox-container input:checked ~ .checkmark:after {
  display: block;
}

.checkbox-container .checkmark:after {
  top: 0.1rem;
  left: 0.35rem;
  width: 0.2rem;
  height: 0.5rem;
  border: solid var(--bg-color-1);
  border-width: 0 3px 3px 0;
  border-radius: 1px;
  transform: rotate(45deg);
}
</style>
