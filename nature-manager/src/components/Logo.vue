<template>
  <div id="logo">
    <img
      @click="showTips()"
      alt="Nature logo"
      width="32"
      height="32"
      src="../assets/logo.png"
    /><span class="h3 align-bottom"> Nature</span>
  </div>
  <Tips ref="tips" />
</template>

<script lang="ts">
import { Vue, Options } from "vue-class-component";
import Tips from "./ModalTips.vue";

@Options({
  components: {
    Tips,
  },
  methods: {
    showTips() {
      this.$refs.tips.show();
    },
  },
})
export default class Logo extends Vue {}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped lang="stylus">
#logo {
  position: fixed;
  z-index: 1000;
  top: 10px;
  left: 10px;
}
</style>
