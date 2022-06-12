<template>
  <div class="tablebox">
    <div class="imgbox">
      <img alt="Nature logo" width="32" height="32" src="../assets/logo.png" />
    </div>
    <span style="line-height: var(--ep-menu-item-height)">Nature</span>
  </div>
</template>

<script lang="ts">
import init from "nature-manager";

export default {
  components: {},
  methods: {
    showTips() {
      init().then((wasm) => {
        wasm.greet();
      });
    },
  },
};
</script>
<style scoped>
.tablebox {
  display: table;
  height: var(--ep-menu-item-height);
}
.imgbox {
  display: table-cell;
  vertical-align: middle;
  padding: 0 10px 0 var(--ep-menu-base-level-padding);
}
</style>
