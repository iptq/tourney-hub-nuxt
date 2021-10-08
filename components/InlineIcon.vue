<template>
  <img class="icon" :src="require('@/assets/icons/'+iconName+'.svg')" :alt="'icon ' + iconName" />
</template>

<script lang="ts">
  export default {
    props: {
      iconName: String
    }
  };
</script>

<style lang="scss" scoped>
  img.icon {
    filter: invert(88%) sepia(17%) saturate(117%) hue-rotate(202deg) brightness(102%) contrast(85%);
    width:  1rem;
    height: 1rem;
  }
</style>
