<template>
  <img :src="'https://a.ppy.sh/' + userId" :style="cssProps" />
</template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

const styleProps = Vue.extend({
  props: {
    userId: Number,
    width: { type: String, default: "1.5rem" },
    height: { type: String, default: "1.5rem" },
  },
});

@Component
export default class OsuPfp extends styleProps {
  get cssProps() {
    return {
      "--img-width": this.width,
      "--img-height": this.height,
    };
  }
}
</script>

<style lang="scss" scoped>
img {
  display: inline-block;
  text-align: center;
  width: var(--img-width);
  height: var(--img-height);
  border-radius: 1rem;
}
</style>
