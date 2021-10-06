<template>
  <a class="button" :href="href">
    <img class="icon" :src="require('@/assets/icons/'+iconName+'.svg')" alt="icon" />
    {{ text }}
  </a>
</template>

<script lang="ts">
  export default {
    props: ['href', 'iconName', 'text']
  };
</script>

<style lang="scss" scoped>
  a.button {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: var(--dense-pad-size);

    padding: var(--dense-pad-size) var(--pad-size);
    width: max-content;

    border: 1px solid var(--main-font-color);
    border-radius: var(--border-radius);

    color: var(--bg-color-1);
    background-color: var(--accent-color);

    :hover {
      opacity: var(--faded-accent-opacity);
      text-decoration: none;
      cursor: pointer;
    }

    > img.icon {
      filter: invert(8%) sepia(6%) saturate(925%) hue-rotate(187deg) brightness(93%)
      contrast(94%);

      width: 1rem;
      height: 1rem;
    }
  }
</style>
