<template>
  <div class="t-listing">
    <div class="content-header">
      <div class="panel-top">
        <div class="search-bar">
          <img
            class="icon search-icon"
            src="/icons/search-solid.svg"
            alt="icon"
          />
          <input type="text" placeholder="search" />
        </div>
        <IconButton :href="'/tournaments/create'" :iconName="'plus-square-regular'" :text="'Create New'"/>
      </div>
      <div class="filters">
        <CheckBox text="filter1" />
        <CheckBox text="filter2" />
        <CheckBox text="filter3" />
        <FormsRadio text="filter1" :checked="true" />
        <FormsRadio text="filter2" />
        <FormsRadio text="filter3" />
        <NumberInput min="{0}" max="{100}" name="wtf" text="wtf" />
      </div>
    </div>
    <div class="tourney-list card">
      <CardTitle title="Listing" />
      {#each tournaments as tournament}
      <hr />
      <div class="tourney-card">
        <img
          src="https://i.redd.it/n7fud5vc85n71.jpg"
          alt="tourney icon"
          class="tourney-icon"
        />
        <div class="tourney-card-content">
          <a href="/tournaments/{tournament.id}">{tournament.name}</a> by
          <a href="/user/{tournament.admin.osu_id}"
            >{tournament.admin.username}</a
          >
        </div>
      </div>
      {/each}
    </div>
  </div>
</template>

<style lang="scss">
div.t-listing {
  display: flex;
  flex-direction: column;
  gap: var(--pad-size);
}
div.t-listing > div.content-header {
  display: flex;
  flex-direction: column;
  gap: var(--pad-size);
}
div.t-listing > div.content-header > div.panel-top {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  gap: var(--pad-size);
}
div.t-listing > div.content-header > div.filters {
  display: flex;
  flex-direction: row;
  gap: calc(1.5 * var(--pad-size));
}
div.search-bar {
  display: flex;
  flex-direction: row;
  flex-grow: 1;
  align-items: center;
  gap: var(--pad-size);
  padding: var(--dense-pad-size) var(--pad-size);
  border: 1px solid var(--bg-color-4);
  border-radius: var(--border-radius);
}
div.search-bar:focus-within {
  outline: 2px solid var(--bg-color-4);
}
div.search-bar > input {
  width: 100%;
  color: var(--main-font-color);
  background-color: unset;
  border: unset;
}
div.search-bar > input:focus {
  outline: none;
}
div.tourney-list {
  display: flex;
  flex-direction: column;
  gap: 0;
}
div.tourney-card {
  display: flex;
  flex-direction: row;
}
div.tourney-card > img.tourney-icon {
  height: 7rem;
  width: 7rem;
}
div.tourney-card-content {
  width: 100%;
  padding: var(--pad-size);
  background-color: var(--bg-color-2);
}
.search-icon {
  filter: invert(96%) sepia(1%) saturate(4258%) hue-rotate(193deg)
    brightness(96%) contrast(91%);
}
</style>
