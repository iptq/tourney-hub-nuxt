<template>
  <div class="card">
    <CardTitle title="Create New Tournament" />
    <div class="card-content">
      <form on:submit|preventDefault|stopPropagation="{submit}">
        <div class="centered">
          <input type="text" placeholder="new tournament name" />
          <input
            type="submit"
            disabled="{spinning}"
            class="button"
            value="Create"
          />
        </div>
      </form>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  middleware: "auth",
};
</script>

<style lang="scss" scoped>
div.centered {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--pad-size);
  padding: var(--pad-size);
  margin: auto;
  max-width: 25rem;
  width: 100%;
  border: 1px solid var(--bg-color-4);
  border-radius: var(--border-radius);

  input {
    &[type="text"] {
      color: unset;
      background: var(--bg-color-1);
      border: 1px solid var(--bg-color-4);
      border-radius: var(--border-radius);
      padding: var(--dense-pad-size) var(--pad-size);
      width: calc(100% - 2 * var(--pad-size));

      &:focus {
        outline: 2px solid var(--bg-color-4);
      }
    }
  }
}
</style>
