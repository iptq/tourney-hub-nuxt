<template>
  <div class="badge" :style="thinMode ? cssPropsThin : cssProps">
    {{ text }}
  </div>
</template>

<script lang="ts">
  import Vue from "vue";
  import Component from "vue-class-component";

  const StyleProps = Vue.extend({
    props: {
      badgeColor: {type: String,  default: "var(--accent-color)"},
      fontColor:  {type: String,  default: "var(--bg-color-1"},
      fontSize:   {type: String,  default: "1rem"},
      text:              String,
      thinMode:   {type: Boolean, default: false}
    }
  })

  @Component
  export default class BadgeText extends StyleProps {
    get cssProps() {
      return {
        '--badge-color':  this.badgeColor,
        '--border-color': this.badgeColor,
        '--font-color':   this.fontColor,
        '--font-size':    this.fontSize
      }
    }
    get cssPropsThin() {
      return {
        '--badge-color':  "#00000000",
        '--border-color': this.fontColor,
        '--font-color':   this.fontColor,
        '--font-size':    this.fontSize
      }
    }
  }
</script>

<style lang="scss" scoped>
  div.badge {
    background-color: var(--badge-color);
    padding: 0.1rem var(--dense-pad-size);
    color: var(--font-color);
    font-size: var(--font-size);
    width: max-content;
    border: 1.5px solid var(--border-color);
    border-radius: 1rem;
  }
</style>
