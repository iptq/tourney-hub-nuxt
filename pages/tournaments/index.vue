<template>
  <div class="t-listing">
    <div class="content-header">
      <div class="panel-top">
        <div class="search-bar">
          <InlineIcon :iconName="'search-solid'" />
          <input type="text" placeholder="search" />
        </div>
        <IconButton :href="'/tournaments/create'" :iconName="'plus-square-regular'" :text="'Create New'"/>
      </div>
      <div class="filters">
        <FormsCheckBox text="filter1" />
        <FormsCheckBox text="filter2" />
        <FormsCheckBox text="filter3" />
        <FormsRadio text="filter4" :checked="true" />
        <FormsRadio text="filter5" />
        <FormsRadio text="filter6" />
        <FormsNumberInput :min="0" :max="100" :name="'wtf'" :widthRem="4" :text="'filter7'" />
      </div>
    </div>
    <hr/>
    <h1>Featured {{flag}}</h1>
    <div class="featured">
      <div class="featured-banner">
        <!-- <div class="featured-tags">
          <BadgeText :text="'4v4'" :thinMode="true" :fontColor="'var(--main-font-color)'"/>
          <BadgeText :text="'Double Elimination'" :thinMode="true" :fontColor="'var(--main-font-color)'"/>
          <BadgeText :text="'std'" :thinMode="true" :fontColor="'var(--main-font-color)'"/>
        </div> -->
      </div>
      <div class="featured-body">
        <div class="featured-body-head">
          <h1>{{featuredTitle}}</h1>
          <span class="light-font">{{featuredDesc}}</span>
        </div>
        <div class="featured-body-footer">
          <div class="host">
            <OsuPfp :userId="3214844" :width="'2rem'" :height="'2rem'"/>
            <div class="host-info">
              <span class="host-name">Feiri</span>
              <span class="host-role light-font">Host</span>
            </div>
          </div>
          <div class="tags">
            <BadgeText :text="'4v4'" :thinMode="true" :fontColor="'var(--main-font-color)'"/>
            <BadgeText :text="'Double Elimination'" :thinMode="true" :fontColor="'var(--main-font-color)'"/>
            <BadgeText :text="'std'" :thinMode="true" :fontColor="'var(--main-font-color)'"/>
          </div>
        </div>
      </div>
    </div>
    <h1>Listing</h1>
    <div class="tourney-list">
      <div class="tourney-card" v-for="tourney in tournaments" :key="tourney">
        <div class="tourney-banner round-border" :style="'--bg-url:url('+tourney.bannerUrl+')'">
          <div class="tags">
            <BadgeText v-for="tag in tourney.tags" :text="tag" :thinMode="true" :fontColor="'var(--main-font-color)'" v-bind:key="tag"/>
          </div>
        </div>
        <div class="tourney-card-body">
          <div class="tourney-card-body-top">
            <h3>{{ tourney.name }}</h3>
            <span class="light-font">{{ tourney.desc }}</span>
          </div>
          <div class="tourney-card-body-bottom">
            <div class="host">
              <OsuPfp :userId="tourney.host.userId" :width="'2rem'" :height="'2rem'"/>
              <div class="host-info">
                <span class="host-name">{{ tourney.host.name }}</span>
                <span class="host-role light-font">Host</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import {flag} from '~/plugins/flag';
export default {
  data() {
    return {
      flag: flag('CA'),
      featuredTitle: "Norsace Fake Tournament",
      featuredDesc: "A very cool 4v4 tournament for very pro players!!! Come play and have a good friggin time yes osu",
      tournaments: [{
        id:        2,
        name:      "Tournament A",
        host:      {userId: 124493, name: "chocomint"},
        desc:      "A poggerific tournament heck yea",
        bannerUrl: "https://i.redd.it/vwueyudqxlr71.jpg",
        tags:      ["2v2", "Double Elimination", "mania"]
      }, {
        id:        3,
        name:      "the tournament of EPIC",
        host:      {userId: 50265, name: "hvick225"},
        desc:      "haha hvick would never come back haha jk... unless???",
        bannerUrl: "https://i.redd.it/exhsqxd9bmr71.jpg",
        tags:      ["1v1", "Double Elimination", "std"]
      }, {
        id:        4,
        name:      "osu!wtf????",
        host:      {userId: 2, name: "peppy"},
        desc:      "this tournament will be so much cooler than osu! world cup heheee hoo hoo",
        bannerUrl: "https://i.redd.it/kd9tecmbvkc61.jpg",
        tags:      ["1v1", "Double Elimination", "std"]
      }, {
        id:        5,
        name:      "the official osu! onetwo tournament",
        host:      {userId: 4452992, name: "Sotarks"},
        desc:      "1273downtherockefellerstreet",
        bannerUrl: "https://i.redd.it/yge9tdl3c4551.png",
        tags:      ["1v1", "Double Elimination", "std"]
      }]
    }
  }
}

export let featuredTitle: string = "Norsace Fake Tourney";
</script>

<style lang="scss" scoped>
div.t-listing {
  display:        flex;
  flex-direction: column;
  gap:            var(--pad-size);

  > div.featured {
    display:               grid;
    grid-template-columns: 3fr 4fr;
    gap:                   var(--pad-size);

    > div.featured-banner {
      background-image:  linear-gradient(var(--faded-bg-color), var(--faded-bg-color)), url(https://i.redd.it/5vbatj2sa0s71.png);
      background-size:   cover;
      background-repeat: no-repeat;

      height:        12rem;
      padding:       var(--pad-size);
      border:        1px solid var(--bg-color-4);
      border-radius: var(--border-radius);

      display:         flex;
      flex-direction:  column-reverse;
      justify-content: space-between;

      > div.featured-tags {
        display:        flex;
        flex-direction: row;
        gap:            var(--dense-pad-size);
      }
    }

    > div.featured-body {
      display:         flex;
      flex-direction:  column;
      justify-content: space-between;

      > div.featured-body-head {
        display:        flex;
        flex-direction: column;
        gap:            var(--dense-pad-size);
      }

      > div.featured-body-footer {
        display:         flex;
        flex-direction:  row;
        justify-content: space-between;
      }
    }
  }

  > div.content-header {
    display:        flex;
    flex-direction: column;
    gap:            var(--pad-size);

    > div.filters {
      display:        flex;
      flex-direction: row;
      gap:            calc(1.5 * var(--pad-size));
    }

    > div.panel-top {
      display:         flex;
      flex-direction:  row;
      justify-content: space-between;
      gap:             var(--pad-size);

      > div.search-bar {
        display:        flex;
        flex-direction: row;
        flex-grow:      1;
        align-items:    center;
        gap:            var(--pad-size);
        padding:        var(--dense-pad-size) var(--pad-size);
        border:         1px solid var(--bg-color-4);
        border-radius:  var(--border-radius);

        &:focus-within {
          outline: 2px solid var(--bg-color-4);
        }

        > input {
          width:            100%;
          color:            var(--main-font-color);
          background-color: unset;
          border:           unset;

          &:focus {
            outline: none;
          }
        }
      }
    }
  }

  > div.tourney-list {
    display:               grid;
    grid-template-columns: 1fr 1fr 1fr 1fr;
    grid-auto-rows:        1fr;
    gap:                   calc(2 * var(--pad-size)) var(--pad-size);

    > div.tourney-card {
      display:        flex;
      flex-direction: column;
      gap: var(--dense-pad-size);

      > div.tourney-banner {
        background-image:  var(--bg-layer), var(--bg-url);
        background-size:   cover;
        background-repeat: no-repeat;

        display:        flex;
        flex-direction: column-reverse;

        height: 6rem;
        padding: var(--pad-size);
      }

      > div.tourney-card-body {
        display:         flex;
        flex-direction:  column;
        flex:            1;
        justify-content: space-between;
        gap:             var(--dense-pad-size);

        > div.tourney-card-body-top {
          display:         flex;
          flex-direction:  column;
          gap:             var(--dense-pad-size);
        }
      }
    }
  }
}

div.host {
  display:        flex;
  flex-direction: row;
  gap:            var(--dense-pad-size);

  > div.host-info {
    display:        flex;
    flex-direction: column;

    > span.host-name {
      font-size: 1.1rem;
    }

    > span.host-role {
      font-size: 0.8rem;
    }
  }
}

div.tags {
  display:        flex;
  flex-direction: row;
  gap:            var(--dense-pad-size);
  align-self:     flex-end;
}

h1, h2, h3 {
  margin: 0;
}

</style>
