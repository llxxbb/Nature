<template>
  <el-container style="height: 100%">
    <el-header class="header"
      ><mode-header ref="logo" @modeChanged="changeEvent"></mode-header
    ></el-header>
    <el-main>
      <domain-mode v-show="isMode(0)" />
      <relation-mode v-show="isMode(1)" />
      <instance-mode v-show="isMode(2)" />
    </el-main>
  </el-container>
</template>

<script lang="ts">
import { NatureMode } from "./business/enum/mode";
export default {
  data() {
    return {
      mode: NatureMode.Domain,
    };
  },
  name: "App",
  methods: {
    changeMode(event: Event, mode: NatureMode) {
      event.preventDefault();
      this.$refs.logo.setMode(mode);
    },
    KeyDown(event: KeyboardEvent) {
      if (event.ctrlKey) {
        switch (event.key) {
          case "d": {
            this.changeMode(event, NatureMode.Domain);
            return;
          }
          case "r": {
            this.changeMode(event, NatureMode.Relation);
            return;
          }
          case "i": {
            this.changeMode(event, NatureMode.Instance);
            return;
          }
        }
      }
    },
    isMode(mode: NatureMode): boolean {
      return mode == this.mode;
    },
    changeEvent(mode: NatureMode) {
      this.mode = mode;
    },
  },
  mounted() {
    window.addEventListener("keydown", this.KeyDown, true); //监听按键事件
  },
  beforeUnmount() {
    window.removeEventListener("keydown", this.KeyDown);
  },
};
</script>

<style lang="sass" scoped>
.header
  border-bottom-style: outset
  border-bottom-color: cornsilk

.el-header
  padding: 10px
  --el-header-height: 52px

.el-main
  --el-main-padding: 5px
</style>