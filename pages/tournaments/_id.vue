<template></template>

<script lang="ts">
import Vue from "vue";
import Component from "vue-class-component";

@Component
export default class extends Vue {
  beforeRouteEnter(to, from, next) {
    next();
  }
}
</script>
